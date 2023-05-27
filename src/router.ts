import { type RouteRecordRaw, createRouter, createWebHashHistory } from 'vue-router'

import HomePage from './views/HomePage.vue'

const routes: RouteRecordRaw[] = [
  { path: '/', component: HomePage }
]

const router = createRouter({
  routes,
  history: createWebHashHistory()
})

export default router
