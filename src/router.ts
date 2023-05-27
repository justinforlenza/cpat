import { type RouteRecordRaw, createRouter, createWebHashHistory } from 'vue-router'

import HomePage from './views/HomePage.vue'
import WelcomePage from './views/WelcomePage.vue'

const routes: RouteRecordRaw[] = [
  { path: '/', component: HomePage },
  { path: '/welcome', component: WelcomePage }
]

const router = createRouter({
  routes,
  history: createWebHashHistory()
})

export default router
