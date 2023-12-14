import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import HomeView from "../views/HomeView.vue";
import StatementsView from "../views/StatementsView.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "home",
    component: HomeView,
  },
  {
    path: "/about",
    name: "about",
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () =>
      import(/* webpackChunkName: "about" */ "../views/AboutView.vue"),
  },
  {
    path: "/statements",
    name: "statements",
    component: StatementsView,
  },
  {
    path: "/proposals",
    name: "proposals",
    component: () =>
      import(/* webpackChunkName: "proposals" */ "../views/ProposalsView.vue"),
  },
  {
    path: "/campaigns",
    name: "campaigns",
    component: () =>
      import(/* webpackChunkName: "campaigns" */ "../views/CampaignsView.vue"),
  },
  {
    path: "/research",
    name: "research",
    component: () =>
      import(/* webpackChunkName: "research" */ "../views/ResearchView.vue"),
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
