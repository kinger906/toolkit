<script setup lang="ts">
import type { ToolModule } from "@/types/tool";

defineProps<{
  tool: ToolModule;
}>();
</script>

<template>
  <router-link :to="`/tools/${tool.id}`" class="tile" :title="tool.description">
    <div class="tile-icon">{{ tool.icon }}</div>
    <span class="tile-name">{{ tool.name }}</span>
  </router-link>
</template>

<style scoped>
.tile {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  width: 108px;
  padding: 14px 8px 10px;
  border-radius: 16px;
  text-decoration: none;
  color: inherit;
  transition: background 0.15s, transform 0.15s;
}

.tile:hover {
  background: color-mix(in srgb, var(--surface) 80%, transparent);
  transform: translateY(-2px);
}

.tile:active {
  transform: translateY(0);
}

.tile-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 64px;
  height: 64px;
  border-radius: 18px;
  background: var(--surface);
  border: 1px solid var(--border);
  font-size: 1.55rem;
  box-shadow: 0 2px 8px rgba(28, 25, 21, 0.06);
  transition: border-color 0.15s, box-shadow 0.15s;
}

.tile:hover .tile-icon {
  border-color: color-mix(in srgb, var(--accent) 45%, var(--border));
  box-shadow: 0 6px 18px rgba(15, 110, 86, 0.12);
}

.tile-name {
  max-width: 100%;
  font-size: 0.8rem;
  font-weight: 500;
  line-height: 1.25;
  text-align: center;
  color: var(--text);
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  word-break: break-word;
}
</style>
