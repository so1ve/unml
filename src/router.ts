import { setupLayouts } from "virtual:meta-layouts";
import { createRouter, createWebHistory } from "vue-router";
import { handleHotUpdate, routes } from "vue-router/auto-routes";

const router = createRouter({
	history: createWebHistory(),
	routes: setupLayouts(routes),
});

if (import.meta.hot) {
	handleHotUpdate(router);
}

export default router;
