import { createRouter, createWebHashHistory } from "vue-router";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      redirect: "/index",
      children: [
        {
          path: "/index",
          name: "index",
          component: () => import("../views/Index.vue"),
        },
        {
          path: "/setting",
          name: "setting",
          component: () => import("../views/Settings.vue"),
        },
      ],
    },
  ],
});

export default router;
