import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("@/views/HomeView.vue"),
      meta: { title: "全部工具" },
    },
    {
      path: "/tools/:id",
      name: "tool",
      component: () => import("@/views/ToolHostView.vue"),
      meta: { title: "工具" },
    },
    {
      path: "/:pathMatch(.*)*",
      redirect: "/",
    },
  ],
});

router.afterEach((to) => {
  const title = (to.meta.title as string) || "ToolKit";
  document.title = `${title} · ToolKit`;
});

export default router;
