import { type RouteRecordRaw, createRouter, createWebHashHistory } from 'vue-router'

import { useConfigStore } from './store'

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

router.beforeEach(async (to) => {
  const { loadConfig } = useConfigStore()
  const { creds } = await loadConfig()

  const needsConfig = creds.email === null || creds.password === null

  if (needsConfig && to.path !== '/welcome') {
    return '/welcome'
  }

  if (!needsConfig && to.path === '/welcome') {
    return '/'
  }
})

export default router
