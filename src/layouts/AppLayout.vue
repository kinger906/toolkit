<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useRoute } from "vue-router";

const keyword = ref("");
const emit = defineEmits<{
  search: [value: string];
}>();

const route = useRoute();
const isHome = computed(() => route.name === "home");

watch(keyword, (v) => emit("search", v));

watch(isHome, (home) => {
  if (!home) keyword.value = "";
});
</script>

<template>
  <div class="layout">
    <header class="topbar">
      <router-link to="/" class="brand">
        <span class="brand-mark">◈</span>
        <span class="brand-text">ToolKit</span>
      </router-link>

      <div v-if="isHome" class="search-wrap">
        <span class="search-icon">⌕</span>
        <input
          v-model="keyword"
          type="search"
          class="search-input"
          placeholder="搜索工具…"
          autocomplete="off"
        />
      </div>

      <router-link v-else to="/" class="back-link">← 返回桌面</router-link>
    </header>

    <main class="content" :class="{ desktop: isHome }">
      <slot />
    </main>
  </div>
</template>

<style scoped>
.layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
  overflow: hidden;
  background: var(--bg);
}

.topbar {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 14px 28px;
  flex-shrink: 0;
}

.brand {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text);
  text-decoration: none;
  font-weight: 700;
  font-size: 1.05rem;
  letter-spacing: -0.02em;
  flex-shrink: 0;
}

.brand-mark {
  color: var(--accent);
  font-size: 1.2rem;
}

.search-wrap {
  position: relative;
  margin-left: auto;
  width: min(320px, 40vw);
}

.search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-faint);
  pointer-events: none;
  font-size: 0.95rem;
}

.search-input {
  width: 100%;
  padding: 9px 14px 9px 34px;
  border: 1px solid var(--border);
  border-radius: 999px;
  background: var(--surface);
  color: var(--text);
  font-size: 0.875rem;
  font-family: inherit;
  outline: none;
  transition: border-color 0.15s, box-shadow 0.15s;
}

.search-input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.search-input::placeholder {
  color: var(--text-faint);
}

.back-link {
  margin-left: auto;
  color: var(--text-muted);
  text-decoration: none;
  font-size: 0.875rem;
  padding: 6px 12px;
  border-radius: 8px;
  transition: color 0.15s, background 0.15s;
}

.back-link:hover {
  color: var(--accent);
  background: var(--accent-soft);
}

.content {
  flex: 1;
  overflow: auto;
  padding: 8px 28px 32px;
}

.content.desktop {
  display: flex;
  flex-direction: column;
}
</style>
