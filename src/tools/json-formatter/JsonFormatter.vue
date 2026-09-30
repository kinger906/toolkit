<script setup lang="ts">
import { computed, ref } from "vue";

const input = ref('{\n  "hello": "world",\n  "count": 1\n}');
const indent = ref(2);
const error = ref("");

const output = computed(() => {
  error.value = "";
  const raw = input.value.trim();
  if (!raw) return "";
  try {
    const parsed = JSON.parse(raw);
    return JSON.stringify(parsed, null, indent.value);
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    return "";
  }
});

function minify() {
  error.value = "";
  try {
    const parsed = JSON.parse(input.value);
    input.value = JSON.stringify(parsed);
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  }
}

async function copyOutput() {
  if (!output.value) return;
  await navigator.clipboard.writeText(output.value);
}
</script>

<template>
  <div class="tool">
    <div class="toolbar">
      <label class="field">
        <span>缩进</span>
        <select v-model.number="indent">
          <option :value="2">2 空格</option>
          <option :value="4">4 空格</option>
        </select>
      </label>
      <div class="actions">
        <button type="button" class="btn" @click="minify">压缩</button>
        <button type="button" class="btn primary" :disabled="!output" @click="copyOutput">
          复制结果
        </button>
      </div>
    </div>

    <div class="panes">
      <div class="pane">
        <div class="pane-label">输入</div>
        <textarea v-model="input" spellcheck="false" placeholder="粘贴 JSON…" />
      </div>
      <div class="pane">
        <div class="pane-label">输出</div>
        <pre v-if="output" class="out">{{ output }}</pre>
        <div v-else-if="error" class="err">{{ error }}</div>
        <div v-else class="placeholder">格式化结果将显示在这里</div>
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

.field {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.field select {
  padding: 6px 10px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--bg);
  color: var(--text);
  font-family: inherit;
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
  min-height: 320px;
}

.pane {
  display: flex;
  flex-direction: column;
  min-height: 0;
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
  min-height: 280px;
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
  word-break: break-word;
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
