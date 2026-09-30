<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

type Action = "shutdown" | "restart";

interface PowerActionResult {
  ok: boolean;
  message: string;
}

interface ScheduleState {
  action: Action;
  deadline: number;
}

const STORAGE_KEY = "toolkit:power-schedule";

const action = ref<Action>("shutdown");
const mode = ref<"preset" | "custom" | "at">("preset");
const customMinutes = ref(30);
const atTime = ref("");
const busy = ref(false);
const error = ref("");
const toast = ref("");
const now = ref(Date.now());
const schedule = ref<ScheduleState | null>(null);

const presets = [
  { label: "15 分钟", seconds: 15 * 60 },
  { label: "30 分钟", seconds: 30 * 60 },
  { label: "1 小时", seconds: 60 * 60 },
  { label: "2 小时", seconds: 2 * 60 * 60 },
  { label: "3 小时", seconds: 3 * 60 * 60 },
];

let timer: ReturnType<typeof setInterval> | null = null;

onMounted(() => {
  restoreSchedule();
  // 默认「指定时刻」为约 1 小时后
  const d = new Date(Date.now() + 60 * 60 * 1000);
  atTime.value = toLocalInput(d);
  timer = setInterval(() => {
    now.value = Date.now();
    if (schedule.value && schedule.value.deadline <= now.value) {
      schedule.value = null;
      localStorage.removeItem(STORAGE_KEY);
    }
  }, 250);
});

onUnmounted(() => {
  if (timer) clearInterval(timer);
});

const remainingMs = computed(() => {
  if (!schedule.value) return 0;
  return Math.max(0, schedule.value.deadline - now.value);
});

const remainingText = computed(() => formatDuration(remainingMs.value));

const deadlineText = computed(() => {
  if (!schedule.value) return "";
  return new Date(schedule.value.deadline).toLocaleString("zh-CN", {
    hour12: false,
  });
});

const actionLabel = computed(() =>
  schedule.value?.action === "restart" ? "重启" : "关机",
);

function toLocalInput(d: Date): string {
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}`;
}

function formatDuration(ms: number): string {
  const total = Math.ceil(ms / 1000);
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  if (h > 0) return `${h}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}

function restoreSchedule() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return;
    const parsed = JSON.parse(raw) as ScheduleState;
    if (parsed.deadline > Date.now()) {
      schedule.value = parsed;
    } else {
      localStorage.removeItem(STORAGE_KEY);
    }
  } catch {
    localStorage.removeItem(STORAGE_KEY);
  }
}

function persist(state: ScheduleState | null) {
  schedule.value = state;
  if (state) {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
  } else {
    localStorage.removeItem(STORAGE_KEY);
  }
}

function resolveSeconds(): number | null {
  error.value = "";
  if (mode.value === "preset") {
    return null; // 由按钮直接传入
  }
  if (mode.value === "custom") {
    const mins = Number(customMinutes.value);
    if (!Number.isFinite(mins) || mins < 0) {
      error.value = "请输入有效的分钟数";
      return null;
    }
    return Math.round(mins * 60);
  }
  // at
  if (!atTime.value) {
    error.value = "请选择关机时间";
    return null;
  }
  const target = new Date(atTime.value).getTime();
  if (Number.isNaN(target)) {
    error.value = "时间格式无效";
    return null;
  }
  const secs = Math.floor((target - Date.now()) / 1000);
  if (secs < 0) {
    error.value = "指定时间已过，请选择未来时间";
    return null;
  }
  return secs;
}

async function startWithSeconds(seconds: number) {
  error.value = "";
  toast.value = "";
  busy.value = true;
  try {
    const cmd =
      action.value === "restart" ? "schedule_restart" : "schedule_shutdown";
    const result = await invoke<PowerActionResult>(cmd, { seconds });
    const deadline = Date.now() + seconds * 1000;
    persist({ action: action.value, deadline });
    toast.value = result.message;
  } catch (e) {
    error.value = typeof e === "string" ? e : String(e);
  } finally {
    busy.value = false;
  }
}

async function startPreset(seconds: number) {
  await startWithSeconds(seconds);
}

async function startResolved() {
  const seconds = resolveSeconds();
  if (seconds === null) return;
  await startWithSeconds(seconds);
}

async function cancel() {
  error.value = "";
  toast.value = "";
  busy.value = true;
  try {
    const result = await invoke<PowerActionResult>("cancel_power_action");
    persist(null);
    toast.value = result.message;
  } catch (e) {
    // 系统侧可能已无计划，仍清理本地倒计时
    persist(null);
    error.value = typeof e === "string" ? e : String(e);
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <div class="tool">
    <div v-if="schedule" class="countdown-card">
      <div class="countdown-label">距{{ actionLabel }}还有</div>
      <div class="countdown-value">{{ remainingText }}</div>
      <div class="countdown-meta">预计 {{ deadlineText }}</div>
      <button type="button" class="btn danger" :disabled="busy" @click="cancel">
        取消计划
      </button>
    </div>

    <div class="panel" :class="{ dimmed: !!schedule }">
      <div class="row">
        <span class="label">操作</span>
        <div class="modes">
          <button
            type="button"
            class="mode"
            :class="{ active: action === 'shutdown' }"
            :disabled="!!schedule || busy"
            @click="action = 'shutdown'"
          >
            关机
          </button>
          <button
            type="button"
            class="mode"
            :class="{ active: action === 'restart' }"
            :disabled="!!schedule || busy"
            @click="action = 'restart'"
          >
            重启
          </button>
        </div>
      </div>

      <div class="row">
        <span class="label">方式</span>
        <div class="modes">
          <button
            type="button"
            class="mode"
            :class="{ active: mode === 'preset' }"
            :disabled="!!schedule || busy"
            @click="mode = 'preset'"
          >
            快捷
          </button>
          <button
            type="button"
            class="mode"
            :class="{ active: mode === 'custom' }"
            :disabled="!!schedule || busy"
            @click="mode = 'custom'"
          >
            自定义
          </button>
          <button
            type="button"
            class="mode"
            :class="{ active: mode === 'at' }"
            :disabled="!!schedule || busy"
            @click="mode = 'at'"
          >
            指定时刻
          </button>
        </div>
      </div>

      <div v-if="mode === 'preset'" class="presets">
        <button
          v-for="p in presets"
          :key="p.seconds"
          type="button"
          class="preset"
          :disabled="!!schedule || busy"
          @click="startPreset(p.seconds)"
        >
          {{ p.label }}
        </button>
      </div>

      <div v-else-if="mode === 'custom'" class="custom">
        <label class="field">
          <span>分钟后{{ action === "restart" ? "重启" : "关机" }}</span>
          <input
            v-model.number="customMinutes"
            type="number"
            min="0"
            step="1"
            :disabled="!!schedule || busy"
          />
        </label>
        <button
          type="button"
          class="btn primary"
          :disabled="!!schedule || busy"
          @click="startResolved"
        >
          开始倒计时
        </button>
      </div>

      <div v-else class="custom">
        <label class="field">
          <span>在此时间{{ action === "restart" ? "重启" : "关机" }}</span>
          <input
            v-model="atTime"
            type="datetime-local"
            :disabled="!!schedule || busy"
          />
        </label>
        <button
          type="button"
          class="btn primary"
          :disabled="!!schedule || busy"
          @click="startResolved"
        >
          开始倒计时
        </button>
      </div>

      <p class="hint">
        基于 Windows <code>shutdown</code> 命令，计划在系统层面生效；即使关闭本应用也会继续倒计时。
      </p>
    </div>

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

.countdown-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 24px 16px;
  border-radius: 14px;
  background: var(--accent-soft);
  border: 1px solid color-mix(in srgb, var(--accent) 25%, transparent);
  text-align: center;
}

.countdown-label {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--accent);
  letter-spacing: 0.04em;
  text-transform: uppercase;
}

.countdown-value {
  font-size: 2.75rem;
  font-weight: 700;
  font-family: var(--font-mono);
  letter-spacing: 0.04em;
  color: var(--text);
  line-height: 1.1;
}

.countdown-meta {
  font-size: 0.85rem;
  color: var(--text-muted);
  margin-bottom: 8px;
}

.panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
  transition: opacity 0.2s;
}

.panel.dimmed {
  opacity: 0.45;
  pointer-events: none;
}

.row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 12px;
}

.label {
  width: 48px;
  flex-shrink: 0;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.modes {
  display: inline-flex;
  padding: 3px;
  border-radius: 8px;
  background: var(--bg);
  border: 1px solid var(--border);
}

.mode {
  padding: 6px 14px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-muted);
  font-size: 0.85rem;
  font-family: inherit;
  cursor: pointer;
}

.mode.active {
  background: var(--surface);
  color: var(--accent);
  font-weight: 600;
  box-shadow: 0 1px 2px rgba(15, 23, 42, 0.06);
}

.mode:disabled {
  cursor: not-allowed;
}

.presets {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.preset {
  min-width: 96px;
  padding: 14px 16px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background: var(--bg);
  color: var(--text);
  font-size: 0.9rem;
  font-weight: 500;
  font-family: inherit;
  cursor: pointer;
  transition: border-color 0.15s, background 0.15s;
}

.preset:hover:not(:disabled) {
  border-color: var(--accent);
  background: var(--accent-soft);
  color: var(--accent);
}

.preset:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.custom {
  display: flex;
  flex-wrap: wrap;
  align-items: flex-end;
  gap: 12px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 0.8rem;
  color: var(--text-muted);
}

.field input {
  min-width: 200px;
  padding: 9px 12px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: var(--bg);
  color: var(--text);
  font-family: inherit;
  font-size: 0.9rem;
}

.field input:focus {
  outline: none;
  border-color: var(--accent);
}

.btn {
  padding: 9px 16px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: var(--bg);
  color: var(--text);
  font-size: 0.85rem;
  font-family: inherit;
  cursor: pointer;
}

.btn.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
}

.btn.danger {
  background: #b42318;
  border-color: #b42318;
  color: #fff;
  margin-top: 4px;
}

.btn:disabled {
  opacity: 0.55;
  cursor: not-allowed;
}

.hint {
  margin: 0;
  font-size: 0.8rem;
  color: var(--text-faint);
  line-height: 1.45;
}

.hint code {
  padding: 1px 5px;
  border-radius: 4px;
  background: var(--surface-hover);
  font-family: var(--font-mono);
  font-size: 0.85em;
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

  .btn.danger {
    background: #e5484d;
    border-color: #e5484d;
  }
}
</style>
