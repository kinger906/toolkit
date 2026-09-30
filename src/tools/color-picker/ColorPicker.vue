<script setup lang="ts">
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface ScreenColor {
  r: number;
  g: number;
  b: number;
  hex: string;
  x: number;
  y: number;
}

const picking = ref(false);
const error = ref("");
const toast = ref("");
const color = ref<ScreenColor>({
  r: 15,
  g: 118,
  b: 110,
  hex: "#0F766E",
  x: 0,
  y: 0,
});

const hexInput = ref(color.value.hex);

const hsl = computed(() => rgbToHsl(color.value.r, color.value.g, color.value.b));

const rgbText = computed(
  () => `rgb(${color.value.r}, ${color.value.g}, ${color.value.b})`,
);

const hslText = computed(() => {
  const { h, s, l } = hsl.value;
  return `hsl(${h}, ${s}%, ${l}%)`;
});

function clampByte(n: number) {
  return Math.max(0, Math.min(255, Math.round(n)));
}

function rgbToHsl(r: number, g: number, b: number) {
  const rr = r / 255;
  const gg = g / 255;
  const bb = b / 255;
  const max = Math.max(rr, gg, bb);
  const min = Math.min(rr, gg, bb);
  const l = (max + min) / 2;
  if (max === min) {
    return { h: 0, s: 0, l: Math.round(l * 100) };
  }
  const d = max - min;
  const s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
  let h = 0;
  switch (max) {
    case rr:
      h = ((gg - bb) / d + (gg < bb ? 6 : 0)) / 6;
      break;
    case gg:
      h = ((bb - rr) / d + 2) / 6;
      break;
    default:
      h = ((rr - gg) / d + 4) / 6;
  }
  return {
    h: Math.round(h * 360),
    s: Math.round(s * 100),
    l: Math.round(l * 100),
  };
}

function applyRgb(r: number, g: number, b: number, x = 0, y = 0) {
  const rr = clampByte(r);
  const gg = clampByte(g);
  const bb = clampByte(b);
  const hex = `#${[rr, gg, bb]
    .map((v) => v.toString(16).padStart(2, "0"))
    .join("")
    .toUpperCase()}`;
  color.value = { r: rr, g: gg, b: bb, hex, x, y };
  hexInput.value = hex;
}

function parseHex(raw: string): { r: number; g: number; b: number } | null {
  let t = raw.trim().replace(/^#/, "");
  if (/^[0-9a-fA-F]{3}$/.test(t)) {
    t = t
      .split("")
      .map((c) => c + c)
      .join("");
  }
  if (!/^[0-9a-fA-F]{6}$/.test(t)) return null;
  return {
    r: parseInt(t.slice(0, 2), 16),
    g: parseInt(t.slice(2, 4), 16),
    b: parseInt(t.slice(4, 6), 16),
  };
}

function onHexCommit() {
  const parsed = parseHex(hexInput.value);
  if (!parsed) {
    error.value = "HEX 格式无效，例如 #0F766E";
    hexInput.value = color.value.hex;
    return;
  }
  error.value = "";
  applyRgb(parsed.r, parsed.g, parsed.b, color.value.x, color.value.y);
}

async function copy(text: string, label: string) {
  try {
    await navigator.clipboard.writeText(text);
    toast.value = `已复制 ${label}`;
    setTimeout(() => {
      if (toast.value === `已复制 ${label}`) toast.value = "";
    }, 1600);
  } catch {
    error.value = "复制失败";
  }
}

async function pickFromScreen() {
  error.value = "";
  toast.value = "";
  picking.value = true;
  try {
    const result = await invoke<ScreenColor>("pick_screen_color");
    applyRgb(result.r, result.g, result.b, result.x, result.y);
    toast.value = `已取色 ${result.hex} @ (${result.x}, ${result.y})`;
  } catch (e) {
    const msg = typeof e === "string" ? e : String(e);
    if (!msg.includes("取消")) {
      error.value = msg;
    } else {
      toast.value = msg;
    }
  } finally {
    picking.value = false;
  }
}

async function sampleCursor() {
  error.value = "";
  try {
    const result = await invoke<ScreenColor>("get_cursor_color");
    applyRgb(result.r, result.g, result.b, result.x, result.y);
  } catch (e) {
    error.value = typeof e === "string" ? e : String(e);
  }
}
</script>

<template>
  <div class="tool">
    <div class="preview-row">
      <div
        class="swatch"
        :style="{ background: color.hex }"
        :title="color.hex"
      />
      <div class="meta">
        <div class="hex-line">
          <input
            v-model="hexInput"
            class="hex-input"
            spellcheck="false"
            @keydown.enter="onHexCommit"
            @blur="onHexCommit"
          />
          <button type="button" class="btn" @click="copy(color.hex, 'HEX')">
            复制
          </button>
        </div>
        <p class="pos">
          坐标 ({{ color.x }}, {{ color.y }})
        </p>
      </div>
    </div>

    <div class="actions">
      <button
        type="button"
        class="btn primary"
        :disabled="picking"
        @click="pickFromScreen"
      >
        {{ picking ? "取色中… 点击屏幕 / Esc 取消" : "屏幕取色" }}
      </button>
      <button type="button" class="btn" :disabled="picking" @click="sampleCursor">
        读取当前指针
      </button>
    </div>

    <div class="formats">
      <div class="row">
        <span class="k">HEX</span>
        <code class="v">{{ color.hex }}</code>
        <button type="button" class="copy" @click="copy(color.hex, 'HEX')">复制</button>
      </div>
      <div class="row">
        <span class="k">RGB</span>
        <code class="v">{{ rgbText }}</code>
        <button type="button" class="copy" @click="copy(rgbText, 'RGB')">复制</button>
      </div>
      <div class="row">
        <span class="k">HSL</span>
        <code class="v">{{ hslText }}</code>
        <button type="button" class="copy" @click="copy(hslText, 'HSL')">复制</button>
      </div>
      <div class="row">
        <span class="k">分量</span>
        <code class="v">R {{ color.r }} · G {{ color.g }} · B {{ color.b }}</code>
        <button
          type="button"
          class="copy"
          @click="copy(`${color.r}, ${color.g}, ${color.b}`, 'RGB 分量')"
        >
          复制
        </button>
      </div>
    </div>

    <p class="hint">
      点击「屏幕取色」后窗口会暂时隐藏，在目标位置单击左键取样；按 Esc 取消。
    </p>

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

.preview-row {
  display: flex;
  gap: 16px;
  align-items: center;
}

.swatch {
  width: 88px;
  height: 88px;
  border-radius: 18px;
  border: 1px solid var(--border);
  box-shadow: var(--glass-shadow);
  flex-shrink: 0;
}

.meta {
  flex: 1;
  min-width: 0;
}

.hex-line {
  display: flex;
  gap: 8px;
}

.hex-input {
  flex: 1;
  min-width: 0;
  padding: 10px 12px;
  border-radius: 10px;
  border: 1px solid var(--border-soft);
  background: var(--bg);
  color: var(--text);
  font-family: var(--font-mono);
  font-size: 1.05rem;
  font-weight: 600;
  letter-spacing: 0.04em;
}

.hex-input:focus {
  outline: none;
  border-color: color-mix(in srgb, var(--accent) 45%, transparent);
}

.pos {
  margin: 8px 0 0;
  font-size: 0.8rem;
  color: var(--text-faint);
  font-family: var(--font-mono);
}

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.btn {
  padding: 9px 14px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background: var(--surface-strong);
  color: var(--text);
  font-size: 0.85rem;
  font-family: inherit;
  cursor: pointer;
}

.btn.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
  font-weight: 600;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.formats {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.row {
  display: grid;
  grid-template-columns: 52px 1fr auto;
  gap: 10px;
  align-items: center;
  padding: 10px 12px;
  border-radius: 10px;
  border: 1px solid var(--border-soft);
  background: var(--bg);
}

.k {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-faint);
  letter-spacing: 0.04em;
}

.v {
  font-family: var(--font-mono);
  font-size: 0.85rem;
  color: var(--text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
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

.hint {
  margin: 0;
  font-size: 0.8rem;
  color: var(--text-faint);
  line-height: 1.45;
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
</style>
