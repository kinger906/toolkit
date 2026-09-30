<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import { getTool } from "@/core/registry";
import { CATEGORY_LABELS } from "@/types/tool";

const route = useRoute();

const tool = computed(() => {
  const id = route.params.id as string;
  return getTool(id);
});
</script>

<template>
  <div v-if="tool" class="tool-host">
    <header class="tool-header glass">
      <div class="tool-title-row">
        <span class="tool-icon glass-strong">{{ tool.icon }}</span>
        <div>
          <h1>{{ tool.name }}</h1>
          <p>{{ tool.description }}</p>
        </div>
      </div>
      <span class="tool-badge">{{ CATEGORY_LABELS[tool.category] }}</span>
    </header>
    <div class="tool-body glass-strong">
      <component :is="tool.component" />
    </div>
  </div>
  <div v-else class="missing glass">
    <h2>工具不存在</h2>
    <p>找不到 id 为「{{ route.params.id }}」的工具。</p>
    <router-link to="/">返回桌面</router-link>
  </div>
</template>

<style scoped>
.tool-host {
  max-width: 960px;
  margin: 0 auto;
}

.tool-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 16px;
  padding: 16px 18px;
  border-radius: var(--radius-lg);
}

.tool-title-row {
  display: flex;
  gap: 12px;
  align-items: flex-start;
}

.tool-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  border-radius: 14px;
  font-size: 1.3rem;
  flex-shrink: 0;
}

.tool-header h1 {
  margin: 0 0 4px;
  font-size: 1.35rem;
  font-weight: 700;
  letter-spacing: -0.03em;
}

.tool-header p {
  margin: 0;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.tool-badge {
  flex-shrink: 0;
  padding: 5px 11px;
  border-radius: 10px;
  background: var(--accent-soft);
  border: 1px solid color-mix(in srgb, var(--accent) 18%, transparent);
  font-size: 0.7rem;
  color: var(--accent);
  font-weight: 500;
}

.tool-body {
  border-radius: var(--radius-lg);
  padding: 20px;
}

.missing {
  text-align: center;
  padding: 64px 16px;
  border-radius: var(--radius-lg);
  color: var(--text-muted);
}

.missing a {
  color: var(--accent);
}
</style>
