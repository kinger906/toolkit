<script setup lang="ts">
import { ref, provide } from "vue";
import AppLayout from "@/layouts/AppLayout.vue";

const keyword = ref("");
provide("searchKeyword", keyword);

function onSearch(value: string) {
  keyword.value = value;
}
</script>

<template>
  <AppLayout @search="onSearch">
    <router-view v-slot="{ Component, route }">
      <component
        :is="Component"
        v-bind="route.name === 'home' ? { keyword } : {}"
      />
    </router-view>
  </AppLayout>
</template>
