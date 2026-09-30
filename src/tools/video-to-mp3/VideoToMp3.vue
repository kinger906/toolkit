<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";

interface FfmpegStatus {
  ready: boolean;
  path: string;
  version: string | null;
  message: string;
}

interface ProbeResult {
  durationSecs: number;
}

interface ConvertResult {
  output: string;
  message: string;
}

const VIDEO_FILTERS = [
  {
    name: "视频",
    extensions: ["mp4", "mkv", "mov", "avi", "webm", "flv", "wmv", "m4v", "ts", "mpeg", "mpg"],
  },
];

const ffmpeg = ref<FfmpegStatus | null>(null);
const preparing = ref(false);
const converting = ref(false);
const probing = ref(false);

const inputPath = ref("");
const outputPath = ref("");
const durationSecs = ref<number | null>(null);

const enableTrim = ref(false);
const startText = ref("0:00");
const endText = ref("");
const bitrate = ref(192);

const error = ref("");
const toast = ref("");

const durationLabel = computed(() => {
  if (durationSecs.value == null) return "";
  return formatClock(durationSecs.value);
});

onMounted(() => {
  refreshFfmpeg();
});

watch(inputPath, (path) => {
  if (!path) {
    outputPath.value = "";
    durationSecs.value = null;
    return;
  }
  if (!outputPath.value || outputPath.value.endsWith(".mp3")) {
    outputPath.value = suggestMp3Path(path);
  }
});

watch(durationSecs, (d) => {
  if (d != null && !endText.value) {
    endText.value = formatClock(d);
  }
});

async function refreshFfmpeg() {
  try {
    ffmpeg.value = await invoke<FfmpegStatus>("get_ffmpeg_status");
  } catch (e) {
    error.value = String(e);
  }
}

async function prepareFfmpeg() {
  error.value = "";
  toast.value = "";
  preparing.value = true;
  try {
    ffmpeg.value = await invoke<FfmpegStatus>("ensure_ffmpeg");
    toast.value = ffmpeg.value.message;
  } catch (e) {
    error.value = typeof e === "string" ? e : String(e);
  } finally {
    preparing.value = false;
  }
}

async function pickFfmpeg() {
  error.value = "";
  toast.value = "";
  const selected = await open({
    multiple: false,
    filters: [{ name: "FFmpeg", extensions: ["exe"] }],
  });
  if (typeof selected !== "string" || !selected) return;
  try {
    ffmpeg.value = await invoke<FfmpegStatus>("set_ffmpeg_path", { path: selected });
    toast.value = ffmpeg.value.message;
    if (inputPath.value) await probeDuration();
  } catch (e) {
    error.value = typeof e === "string" ? e : String(e);
  }
}

async function pickInput() {
  error.value = "";
  const selected = await open({
    multiple: false,
    filters: VIDEO_FILTERS,
  });
  if (typeof selected !== "string" || !selected) return;
  inputPath.value = selected;
  await probeDuration();
}

async function pickOutput() {
  const selected = await save({
    defaultPath: outputPath.value || "output.mp3",
    filters: [{ name: "MP3", extensions: ["mp3"] }],
  });
  if (typeof selected !== "string" || !selected) return;
  outputPath.value = selected.endsWith(".mp3") ? selected : `${selected}.mp3`;
}

async function probeDuration() {
  if (!inputPath.value || !ffmpeg.value?.ready) return;
  probing.value = true;
  error.value = "";
  try {
    const result = await invoke<ProbeResult>("probe_media_duration", {
      path: inputPath.value,
    });
    durationSecs.value = result.durationSecs;
    endText.value = formatClock(result.durationSecs);
  } catch (e) {
    durationSecs.value = null;
    error.value = typeof e === "string" ? e : String(e);
  } finally {
    probing.value = false;
  }
}

function suggestMp3Path(videoPath: string): string {
  const normalized = videoPath.replace(/\\/g, "/");
  const slash = normalized.lastIndexOf("/");
  const base = slash >= 0 ? normalized.slice(slash + 1) : normalized;
  const dot = base.lastIndexOf(".");
  const name = dot > 0 ? base.slice(0, dot) : base;
  const dir = slash >= 0 ? videoPath.slice(0, videoPath.length - base.length) : "";
  return `${dir}${name}.mp3`;
}

function parseClock(text: string): number | null {
  const raw = text.trim();
  if (!raw) return null;
  if (/^\d+(\.\d+)?$/.test(raw)) {
    const n = Number(raw);
    return Number.isFinite(n) && n >= 0 ? n : null;
  }
  const parts = raw.split(":").map((p) => p.trim());
  if (parts.some((p) => p === "" || Number.isNaN(Number(p)))) return null;
  if (parts.length === 2) {
    const m = Number(parts[0]);
    const s = Number(parts[1]);
    if (m < 0 || s < 0 || s >= 60) return null;
    return m * 60 + s;
  }
  if (parts.length === 3) {
    const h = Number(parts[0]);
    const m = Number(parts[1]);
    const s = Number(parts[2]);
    if (h < 0 || m < 0 || m >= 60 || s < 0 || s >= 60) return null;
    return h * 3600 + m * 60 + s;
  }
  return null;
}

function formatClock(total: number): string {
  const secs = Math.max(0, Math.floor(total));
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  const s = secs % 60;
  if (h > 0) {
    return `${h}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
  }
  return `${m}:${String(s).padStart(2, "0")}`;
}

function fileName(path: string): string {
  const n = path.replace(/\\/g, "/");
  return n.slice(n.lastIndexOf("/") + 1);
}

async function convert() {
  error.value = "";
  toast.value = "";

  if (!ffmpeg.value?.ready) {
    error.value = "请先准备 FFmpeg";
    return;
  }
  if (!inputPath.value) {
    error.value = "请选择视频文件";
    return;
  }
  if (!outputPath.value) {
    error.value = "请指定输出路径";
    return;
  }

  let startSecs: number | undefined;
  let endSecs: number | undefined;

  if (enableTrim.value) {
    const start = parseClock(startText.value);
    const end = parseClock(endText.value);
    if (start == null) {
      error.value = "开始时间格式无效（如 0:30 或 90）";
      return;
    }
    if (end == null) {
      error.value = "结束时间格式无效（如 1:20 或 80）";
      return;
    }
    if (end <= start) {
      error.value = "结束时间必须大于开始时间";
      return;
    }
    if (durationSecs.value != null && start >= durationSecs.value) {
      error.value = "开始时间超出视频时长";
      return;
    }
    startSecs = start;
    endSecs = end;
  }

  converting.value = true;
  try {
    const result = await invoke<ConvertResult>("convert_video_to_mp3", {
      req: {
        input: inputPath.value,
        output: outputPath.value,
        startSecs: startSecs ?? null,
        endSecs: endSecs ?? null,
        bitrateKbps: bitrate.value,
      },
    });
    toast.value = `${result.message} → ${fileName(result.output)}`;
  } catch (e) {
    error.value = typeof e === "string" ? e : String(e);
  } finally {
    converting.value = false;
  }
}
</script>

<template>
  <div class="tool">
    <div class="status" :class="{ ready: ffmpeg?.ready }">
      <div class="status-text">
        <strong>{{ ffmpeg?.ready ? "FFmpeg 就绪" : "需要 FFmpeg" }}</strong>
        <span :title="ffmpeg?.path || ffmpeg?.message">
          {{ ffmpeg?.message || "检测中…" }}
        </span>
        <span v-if="ffmpeg?.path" class="path-hint" :title="ffmpeg.path">
          {{ ffmpeg.path }}
        </span>
      </div>
      <div class="status-actions">
        <button type="button" class="btn" :disabled="preparing" @click="pickFfmpeg">
          选择本机…
        </button>
        <button
          type="button"
          class="btn"
          :disabled="preparing || !!ffmpeg?.ready"
          @click="prepareFfmpeg"
        >
          {{ preparing ? "处理中…" : ffmpeg?.ready ? "已就绪" : "自动查找/下载" }}
        </button>
      </div>
    </div>

    <div class="field-block">
      <div class="field-label">视频文件</div>
      <div class="path-row">
        <div class="path-box" :title="inputPath">
          {{ inputPath ? fileName(inputPath) : "未选择" }}
        </div>
        <button type="button" class="btn" :disabled="converting" @click="pickInput">
          选择…
        </button>
      </div>
      <div v-if="durationSecs != null" class="meta">
        时长 {{ durationLabel }}
        <span v-if="probing"> · 解析中…</span>
      </div>
    </div>

    <div class="field-block">
      <div class="field-label">输出 MP3</div>
      <div class="path-row">
        <div class="path-box" :title="outputPath">
          {{ outputPath ? fileName(outputPath) : "未指定" }}
        </div>
        <button
          type="button"
          class="btn"
          :disabled="converting || !inputPath"
          @click="pickOutput"
        >
          另存为…
        </button>
      </div>
    </div>

    <label class="check">
      <input v-model="enableTrim" type="checkbox" :disabled="converting" />
      <span>剪切片段</span>
    </label>

    <div v-if="enableTrim" class="trim">
      <label class="field">
        <span>开始</span>
        <input v-model="startText" type="text" placeholder="0:00" :disabled="converting" />
      </label>
      <label class="field">
        <span>结束</span>
        <input v-model="endText" type="text" placeholder="1:30" :disabled="converting" />
      </label>
      <p class="hint">支持 <code>分:秒</code>、<code>时:分:秒</code> 或纯秒数</p>
    </div>

    <label class="field inline">
      <span>音质</span>
      <select v-model.number="bitrate" :disabled="converting">
        <option :value="128">128 kbps</option>
        <option :value="192">192 kbps</option>
        <option :value="256">256 kbps</option>
        <option :value="320">320 kbps</option>
      </select>
    </label>

    <button
      type="button"
      class="btn primary convert"
      :disabled="converting || preparing || !inputPath || !outputPath"
      @click="convert"
    >
      {{ converting ? "转换中…" : "开始转换" }}
    </button>

    <p v-if="toast" class="toast">{{ toast }}</p>
    <p v-if="error" class="err">{{ error }}</p>
  </div>
</template>

<style scoped>
.tool {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.status {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 12px 14px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background: var(--bg);
}

.status.ready {
  border-color: color-mix(in srgb, var(--accent) 30%, var(--border));
  background: var(--accent-soft);
}

.status-text {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.status-text strong {
  font-size: 0.85rem;
  color: var(--text);
}

.status-text span {
  font-size: 0.78rem;
  color: var(--text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.path-hint {
  font-family: var(--font-mono);
  font-size: 0.7rem !important;
  color: var(--text-faint) !important;
}

.status-actions {
  display: flex;
  flex-shrink: 0;
  gap: 8px;
}

.field-block {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.field-label {
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  color: var(--text-faint);
}

.path-row {
  display: flex;
  gap: 8px;
}

.path-box {
  flex: 1;
  min-width: 0;
  padding: 9px 12px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: var(--bg);
  font-size: 0.875rem;
  color: var(--text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.meta {
  font-size: 0.8rem;
  color: var(--text-muted);
}

.check {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  font-size: 0.9rem;
  color: var(--text);
  cursor: pointer;
  width: fit-content;
}

.trim {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  padding: 12px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background: var(--bg);
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 0.8rem;
  color: var(--text-muted);
}

.field.inline {
  flex-direction: row;
  align-items: center;
  gap: 10px;
}

.field input,
.field select {
  padding: 8px 10px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: var(--surface);
  color: var(--text);
  font-family: var(--font-mono);
  font-size: 0.875rem;
}

.field.inline select {
  font-family: inherit;
}

.field input:focus,
.field select:focus {
  outline: none;
  border-color: var(--accent);
}

.hint {
  grid-column: 1 / -1;
  margin: 0;
  font-size: 0.78rem;
  color: var(--text-faint);
}

.hint code {
  padding: 1px 5px;
  border-radius: 4px;
  background: var(--surface-hover);
  font-family: var(--font-mono);
  font-size: 0.9em;
}

.btn {
  padding: 8px 14px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: var(--surface);
  color: var(--text);
  font-size: 0.85rem;
  font-family: inherit;
  cursor: pointer;
  white-space: nowrap;
}

.btn:hover:not(:disabled) {
  border-color: var(--accent);
}

.btn.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
}

.btn.convert {
  align-self: flex-start;
  padding: 10px 20px;
  font-weight: 600;
}

.btn:disabled {
  opacity: 0.55;
  cursor: not-allowed;
}

.toast {
  margin: 0;
  padding: 10px 12px;
  border-radius: 8px;
  background: var(--accent-soft);
  color: var(--accent);
  font-size: 0.85rem;
}

.err {
  margin: 0;
  padding: 10px 12px;
  border-radius: 8px;
  background: #fffbeb;
  border: 1px solid #fcd34d;
  color: #b45309;
  font-size: 0.85rem;
}

@media (prefers-color-scheme: dark) {
  .err {
    background: #2a2110;
    border-color: #92610a;
    color: #fbbf24;
  }
}

@media (max-width: 560px) {
  .trim {
    grid-template-columns: 1fr;
  }
}
</style>
