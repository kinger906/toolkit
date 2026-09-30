<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";

const now = ref(Date.now());
const input = ref(String(Math.floor(Date.now() / 1000)));
const unit = ref<"s" | "ms">("s");

let timer: ReturnType<typeof setInterval> | null = null;

onMounted(() => {
  timer = setInterval(() => {
    now.value = Date.now();
  }, 1000);
});

onUnmounted(() => {
  if (timer) clearInterval(timer);
});

const parsed = computed(() => {
  const n = Number(input.value.trim());
  if (!Number.isFinite(n)) return null;
  const ms = unit.value === "s" ? n * 1000 : n;
  const d = new Date(ms);
  if (Number.isNaN(d.getTime())) return null;
  return d;
});

const formatted = computed(() => {
  if (!parsed.value) return null;
  return {
    local: parsed.value.toLocaleString("zh-CN", { hour12: false }),
    iso: parsed.value.toISOString(),
    unix: Math.floor(parsed.value.getTime() / 1000),
    ms: parsed.value.getTime(),
  };
});

function useNow() {
  const t = Date.now();
  unit.value = "s";
  input.value = String(Math.floor(t / 1000));
}

async function copy(text: string) {
  await navigator.clipboard.writeText(text);
}
</script>

<template>
  <div class="tool">
    <div class="now-card">
      <div class="now-label">当前时间</div>
      <div class="now-value">{{ new Date(now).toLocaleString("zh-CN", { hour12: false }) }}</div>
      <div class="now-meta">
        Unix {{ Math.floor(now / 1000) }} · ms {{ now }}
      </div>
    </div>

    <div class="convert">
      <div class="toolbar">
        <label class="field">
          <span>单位</span>
          <select v-model="unit">
            <option value="s">秒 (s)</option>
            <option value="ms">毫秒 (ms)</option>
          </select>
        </label>
        <button type="button" class="btn" @click="useNow">填入当前</button>
      </div>

      <label class="input-wrap">
        <span class="pane-label">时间戳</span>
        <input v-model="input" type="text" spellcheck="false" placeholder="例如 1710000000" />
      </label>

      <div v-if="formatted" class="results">
        <div class="row">
          <span class="k">本地时间</span>
          <span class="v">{{ formatted.local }}</span>
          <button type="button" class="copy" @click="copy(formatted.local)">复制</button>
        </div>
        <div class="row">
          <span class="k">ISO 8601</span>
          <span class="v mono">{{ formatted.iso }}</span>
          <button type="button" class="copy" @click="copy(formatted.iso)">复制</button>
        </div>
        <div class="row">
          <span class="k">Unix (s)</span>
          <span class="v mono">{{ formatted.unix }}</span>
          <button type="button" class="copy" @click="copy(String(formatted.unix))">复制</button>
        </div>
        <div class="row">
          <span class="k">Unix (ms)</span>
          <span class="v mono">{{ formatted.ms }}</span>
          <button type="button" class="copy" @click="copy(String(formatted.ms))">复制</button>
        </div>
      </div>
      <div v-else class="err">无法解析该时间戳</div>
    </div>
  </div>
</template>

<style scoped>
.tool {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.now-card {
  padding: 16px;
  border-radius: 10px;
  background: var(--accent-soft);
  border: 1px solid color-mix(in srgb, var(--accent) 20%, transparent);
}

.now-label {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--accent);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.now-value {
  margin-top: 6px;
  font-size: 1.25rem;
  font-weight: 700;
  letter-spacing: -0.02em;
  color: var(--text);
}

.now-meta {
  margin-top: 4px;
  font-size: 0.8rem;
  font-family: var(--font-mono);
  color: var(--text-muted);
}

.convert {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.toolbar {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
}

.field {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.field select,
.input-wrap input {
  padding: 7px 10px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--bg);
  color: var(--text);
  font-family: inherit;
}

.input-wrap {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.input-wrap input {
  font-family: var(--font-mono);
  font-size: 0.9rem;
}

.input-wrap input:focus {
  outline: none;
  border-color: var(--accent);
}

.pane-label {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-faint);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.btn {
  padding: 7px 14px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: var(--bg);
  color: var(--text);
  font-size: 0.85rem;
  font-family: inherit;
  cursor: pointer;
}

.btn:hover {
  border-color: var(--accent);
}

.results {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.row {
  display: grid;
  grid-template-columns: 90px 1fr auto;
  gap: 10px;
  align-items: center;
  padding: 10px 12px;
  border-radius: 8px;
  background: var(--bg);
  border: 1px solid var(--border);
}

.k {
  font-size: 0.8rem;
  color: var(--text-faint);
}

.v {
  font-size: 0.875rem;
  color: var(--text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.mono {
  font-family: var(--font-mono);
}

.copy {
  padding: 4px 10px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--surface);
  color: var(--text-muted);
  font-size: 0.75rem;
  font-family: inherit;
  cursor: pointer;
}

.copy:hover {
  color: var(--accent);
  border-color: var(--accent);
}

.err {
  padding: 12px;
  border-radius: 8px;
  color: #b45309;
  background: #fffbeb;
  border: 1px solid #fcd34d;
  font-size: 0.85rem;
}
</style>
