<script setup lang="ts">
import { computed, ref } from "vue";

type Mode = "encode" | "decode";

const mode = ref<Mode>("encode");
const input = ref("Hello, ToolKit!");
const error = ref("");

const output = computed(() => {
  error.value = "";
  const raw = input.value;
  if (!raw) return "";
  try {
    if (mode.value === "encode") {
      return btoa(unescape(encodeURIComponent(raw)));
    }
    return decodeURIComponent(escape(atob(raw.trim())));
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    return "";
  }
});

function swap() {
  if (!output.value) return;
  input.value = output.value;
  mode.value = mode.value === "encode" ? "decode" : "encode";
}

async function copyOutput() {
  if (!output.value) return;
  await navigator.clipboard.writeText(output.value);
}
</script>

<template>
  <div class="tool">
    <div class="toolbar">
      <div class="modes">
        <button
          type="button"
          class="mode"
          :class="{ active: mode === 'encode' }"
          @click="mode = 'encode'"
        >
          编码
        </button>
        <button
          type="button"
          class="mode"
          :class="{ active: mode === 'decode' }"
          @click="mode = 'decode'"
        >
          解码
        </button>
      </div>
      <div class="actions">
        <button type="button" class="btn" :disabled="!output" @click="swap">互换</button>
        <button type="button" class="btn primary" :disabled="!output" @click="copyOutput">
          复制结果
        </button>
      </div>
    </div>

    <div class="panes">
      <div class="pane">
        <div class="pane-label">{{ mode === "encode" ? "原文" : "Base64" }}</div>
        <textarea v-model="input" spellcheck="false" placeholder="输入内容…" />
      </div>
      <div class="pane">
        <div class="pane-label">{{ mode === "encode" ? "Base64" : "原文" }}</div>
        <pre v-if="output" class="out">{{ output }}</pre>
        <div v-else-if="error" class="err">{{ error }}</div>
        <div v-else class="placeholder">结果将显示在这里</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.tool {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.toolbar {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
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

.actions {
  display: flex;
  gap: 8px;
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

.btn:hover:not(:disabled) {
  border-color: var(--accent);
}

.btn.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.panes {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.pane {
  display: flex;
  flex-direction: column;
}

.pane-label {
  margin-bottom: 6px;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-faint);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

textarea,
.out,
.err,
.placeholder {
  flex: 1;
  min-height: 220px;
  margin: 0;
  padding: 12px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: var(--bg);
  font-family: var(--font-mono);
  font-size: 0.8rem;
  line-height: 1.5;
  resize: vertical;
  color: var(--text);
}

textarea:focus {
  outline: none;
  border-color: var(--accent);
}

.out {
  overflow: auto;
  white-space: pre-wrap;
  word-break: break-all;
}

.err {
  color: #b45309;
  background: #fffbeb;
  border-color: #fcd34d;
}

.placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-faint);
}

@media (max-width: 720px) {
  .panes {
    grid-template-columns: 1fr;
  }
}
</style>
