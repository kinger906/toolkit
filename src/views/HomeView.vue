<script setup lang="ts">
import { computed } from "vue";
import ToolCard from "@/components/ToolCard.vue";
import { toolsByCategory, searchTools } from "@/core/registry";
import { CATEGORY_LABELS } from "@/types/tool";
import type { ToolCategory, ToolModule } from "@/types/tool";

const props = defineProps<{
  keyword?: string;
}>();

const query = computed(() => props.keyword ?? "");
const isSearching = computed(() => query.value.trim().length > 0);

const matched = computed(() => searchTools(query.value));

const groups = computed(() => {
  if (isSearching.value) {
    return matched.value.length
      ? ([["other", matched.value]] as [ToolCategory, ToolModule[]][])
      : [];
  }
  return toolsByCategory.value;
});
</script>

<template>
  <div class="desktop">
    <template v-if="groups.length">
      <section v-for="[category, list] in groups" :key="category" class="group">
        <h2 v-if="!isSearching" class="group-title">
          {{ CATEGORY_LABELS[category] }}
        </h2>
        <h2 v-else class="group-title">
          找到 {{ list.length }} 个工具
        </h2>
        <div class="icons">
          <ToolCard v-for="tool in list" :key="tool.id" :tool="tool" />
        </div>
      </section>
    </template>

    <div v-else class="empty">
      <p>没有找到匹配的工具</p>
      <p class="hint">换个关键词，或到 <code>src/tools/</code> 添加新工具</p>
    </div>
  </div>
</template>

<style scoped>
.desktop {
  flex: 1;
  max-width: 1100px;
  margin: 0 auto;
  width: 100%;
  padding-top: 12px;
}

.group {
  margin-bottom: 28px;
}

.group-title {
  margin: 0 0 12px 8px;
  font-size: 0.72rem;
  font-weight: 600;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--text-faint);
}

.icons {
  display: flex;
  flex-wrap: wrap;
  gap: 10px 6px;
  padding: 8px;
  border-radius: var(--radius-lg);
  background: var(--surface);
  border: 1px solid var(--border);
  backdrop-filter: blur(var(--glass-blur)) saturate(1.3);
  -webkit-backdrop-filter: blur(var(--glass-blur)) saturate(1.3);
}

.empty {
  padding: 80px 16px;
  text-align: center;
  color: var(--text-muted);
  border-radius: var(--radius-lg);
  background: var(--surface);
  border: 1px solid var(--border);
  backdrop-filter: blur(var(--glass-blur));
  -webkit-backdrop-filter: blur(var(--glass-blur));
}

.empty .hint {
  margin-top: 8px;
  font-size: 0.85rem;
  color: var(--text-faint);
}

.empty code {
  padding: 1px 6px;
  border-radius: 6px;
  background: var(--surface-hover);
  font-family: var(--font-mono);
  font-size: 0.8em;
}
</style>
