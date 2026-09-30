<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

const appWindow = getCurrentWindow();

async function minimize() {
  await appWindow.minimize();
}

async function hideToTray() {
  await invoke("hide_to_tray");
}

async function close() {
  await appWindow.close();
}
</script>

<template>
  <div class="titlebar" data-tauri-drag-region>
    <div class="titlebar-left" data-tauri-drag-region>
      <span class="title-mark">◈</span>
      <span class="title-text">ToolKit</span>
    </div>

    <div class="controls">
      <button type="button" class="ctrl" title="最小化" @click="minimize">
        <svg viewBox="0 0 12 12" width="12" height="12" aria-hidden="true">
          <path d="M2 6h8" stroke="currentColor" stroke-width="1.2" />
        </svg>
      </button>
      <button
        type="button"
        class="ctrl tray"
        title="收起到系统托盘"
        @click="hideToTray"
      >
        <svg viewBox="0 0 12 12" width="12" height="12" aria-hidden="true">
          <rect
            x="2"
            y="2"
            width="8"
            height="6"
            rx="1"
            fill="none"
            stroke="currentColor"
            stroke-width="1.2"
          />
          <path d="M4 10h4" stroke="currentColor" stroke-width="1.2" />
        </svg>
      </button>
      <button type="button" class="ctrl close" title="关闭" @click="close">
        <svg viewBox="0 0 12 12" width="12" height="12" aria-hidden="true">
          <path d="M3 3l6 6M9 3l-6 6" stroke="currentColor" stroke-width="1.2" />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  position: relative;
  z-index: 20;
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 36px;
  padding: 0 6px 0 14px;
  flex-shrink: 0;
  user-select: none;
}

.titlebar-left {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.title-mark {
  color: var(--accent);
  font-size: 0.95rem;
  line-height: 1;
}

.title-text {
  font-size: 0.8rem;
  font-weight: 600;
  letter-spacing: -0.02em;
  color: var(--text-muted);
}

.controls {
  display: flex;
  align-items: center;
  gap: 2px;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.ctrl {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 28px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  transition: background 0.15s, color 0.15s;
}

.ctrl:hover {
  background: var(--surface-hover);
  color: var(--text);
}

.ctrl.tray:hover {
  color: var(--accent);
  background: var(--accent-soft);
}

.ctrl.close:hover {
  background: #e5484d;
  color: #fff;
}
</style>
