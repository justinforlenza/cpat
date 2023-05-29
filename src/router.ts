import { type RouteRecordRaw, createRouter, createMemoryHistory } from 'vue-router'

import { useConfigStore } from './store'

import HomePage from './views/HomePage.vue'
import WelcomePage from './views/WelcomePage.vue'

const routes: RouteRecordRaw[] = [
  { path: '/', component: HomePage, name: 'Home' },
  { path: '/welcome', component: WelcomePage }
]

const router = createRouter({
  routes,
  history: createMemoryHistory()
})

router.beforeEach(async (to) => {
  const { loadConfig } = useConfigStore()
  const { creds } = await loadConfig()

  const needsConfig = creds.username === null

  if (needsConfig && to.path !== '/welcome') {
    return '/welcome'
  }

  if (!needsConfig && to.path === '/welcome') {
    return '/'
  }
})

export default router
