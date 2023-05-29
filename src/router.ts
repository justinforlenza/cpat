import { type RouteRecordRaw, createRouter, createMemoryHistory } from 'vue-router'

import { useConfigStore } from './store'

import HomePage from './views/HomePage.vue'
import WelcomePage from './views/WelcomePage.vue'

import EmployabilitySkills from './views/students/EmployablitySkills.vue'
import TechnicalSkills from './views/students/TechnicalSkills.vue'
import IndustryCredentials from './views/students/IndustryCredentials.vue'
import CTECourses from './views/students/CTECourses.vue'

const routes: RouteRecordRaw[] = [
  { path: '/', component: HomePage, name: 'Home' },
  { path: '/welcome', component: WelcomePage },
  { path: '/emp_skills', component: EmployabilitySkills },
  { path: '/tech_skills', component: TechnicalSkills },
  { path: '/credentials', component: IndustryCredentials },
  { path: '/cte_courses', component: CTECourses }
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
