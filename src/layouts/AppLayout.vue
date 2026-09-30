<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useRoute } from "vue-router";
import TitleBar from "@/components/TitleBar.vue";

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
    <div class="ambient" aria-hidden="true">
      <span class="blob blob-a" />
      <span class="blob blob-b" />
      <span class="blob blob-c" />
    </div>

    <TitleBar />

    <header class="topbar glass">
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
  position: relative;
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
  overflow: hidden;
  background: transparent;
}

.ambient {
  position: absolute;
  inset: 0;
  z-index: 0;
  overflow: hidden;
  pointer-events: none;
  background:
    radial-gradient(1200px 700px at 10% -10%, var(--bg-accent-a), transparent 60%),
    radial-gradient(900px 600px at 90% 10%, var(--bg-accent-c), transparent 55%),
    radial-gradient(800px 500px at 50% 100%, var(--bg-accent-b), transparent 50%),
    var(--bg);
}

.blob {
  position: absolute;
  border-radius: 50%;
  filter: blur(64px);
  opacity: 0.55;
  animation: float 18s ease-in-out infinite;
}

.blob-a {
  width: 340px;
  height: 340px;
  left: -60px;
  top: 18%;
  background: color-mix(in srgb, var(--bg-accent-a) 80%, white);
}

.blob-b {
  width: 280px;
  height: 280px;
  right: -40px;
  top: 42%;
  background: color-mix(in srgb, var(--bg-accent-b) 75%, white);
  animation-delay: -6s;
}

.blob-c {
  width: 320px;
  height: 320px;
  left: 38%;
  bottom: -80px;
  background: color-mix(in srgb, var(--bg-accent-c) 70%, white);
  animation-delay: -11s;
}

@keyframes float {
  0%,
  100% {
    transform: translate(0, 0) scale(1);
  }
  50% {
    transform: translate(18px, -22px) scale(1.06);
  }
}

.topbar {
  position: relative;
  z-index: 2;
  display: flex;
  align-items: center;
  gap: 16px;
  margin: 4px 18px 0;
  padding: 12px 18px;
  border-radius: 16px;
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
  letter-spacing: -0.03em;
  flex-shrink: 0;
}

.brand-mark {
  color: var(--accent);
  font-size: 1.15rem;
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
  border: 1px solid var(--border-soft);
  border-radius: 12px;
  background: var(--surface-strong);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  color: var(--text);
  font-size: 0.875rem;
  font-family: inherit;
  outline: none;
  transition: border-color 0.15s, background 0.15s;
}

.search-input:focus {
  border-color: color-mix(in srgb, var(--accent) 45%, transparent);
  background: var(--surface-hover);
}

.search-input::placeholder {
  color: var(--text-faint);
}

.back-link {
  margin-left: auto;
  color: var(--text-muted);
  text-decoration: none;
  font-size: 0.875rem;
  padding: 7px 12px;
  border-radius: 10px;
  border: 1px solid transparent;
  transition: color 0.15s, background 0.15s, border-color 0.15s;
}

.back-link:hover {
  color: var(--accent);
  background: var(--accent-soft);
  border-color: color-mix(in srgb, var(--accent) 20%, transparent);
}

.content {
  position: relative;
  z-index: 1;
  flex: 1;
  overflow: auto;
  padding: 16px 28px 32px;
}

.content.desktop {
  display: flex;
  flex-direction: column;
}
</style>
