<script setup lang="ts">
import type { ToolModule } from "@/types/tool";

defineProps<{
  tool: ToolModule;
}>();
</script>

<template>
  <router-link :to="`/tools/${tool.id}`" class="tile" :title="tool.description">
    <div class="tile-icon glass">{{ tool.icon }}</div>
    <span class="tile-name">{{ tool.name }}</span>
  </router-link>
</template>

<style scoped>
.tile {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  width: 112px;
  padding: 14px 8px 10px;
  border-radius: 18px;
  text-decoration: none;
  color: inherit;
  transition: background 0.2s, transform 0.2s;
}

.tile:hover {
  background: var(--surface);
  transform: translateY(-3px);
}

.tile:active {
  transform: translateY(-1px);
}

.tile-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 66px;
  height: 66px;
  border-radius: 20px;
  font-size: 1.55rem;
  transition: transform 0.2s, border-color 0.2s, background 0.2s;
}

.tile:hover .tile-icon {
  background: var(--surface-hover);
  border-color: color-mix(in srgb, var(--accent) 35%, var(--border));
  transform: scale(1.04);
}

.tile-name {
  max-width: 100%;
  font-size: 0.8rem;
  font-weight: 500;
  line-height: 1.25;
  text-align: center;
  color: var(--text);
  text-shadow: 0 1px 0 rgba(255, 255, 255, 0.35);
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  word-break: break-word;
}

@media (prefers-color-scheme: dark) {
  .tile-name {
    text-shadow: none;
  }
}
</style>
