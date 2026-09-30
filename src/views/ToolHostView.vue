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
    <header class="tool-header">
      <div class="tool-title-row">
        <span class="tool-icon">{{ tool.icon }}</span>
        <div>
          <h1>{{ tool.name }}</h1>
          <p>{{ tool.description }}</p>
        </div>
      </div>
      <span class="tool-badge">{{ CATEGORY_LABELS[tool.category] }}</span>
    </header>
    <div class="tool-body">
      <component :is="tool.component" />
    </div>
  </div>
  <div v-else class="missing">
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
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border);
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
  background: var(--surface);
  border: 1px solid var(--border);
  font-size: 1.3rem;
  flex-shrink: 0;
}

.tool-header h1 {
  margin: 0 0 4px;
  font-size: 1.35rem;
  font-weight: 700;
  letter-spacing: -0.02em;
}

.tool-header p {
  margin: 0;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.tool-badge {
  flex-shrink: 0;
  padding: 4px 10px;
  border-radius: 999px;
  background: var(--surface-hover);
  font-size: 0.7rem;
  color: var(--text-faint);
}

.tool-body {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 20px;
}

.missing {
  text-align: center;
  padding: 64px 16px;
  color: var(--text-muted);
}

.missing a {
  color: var(--accent);
}
</style>
