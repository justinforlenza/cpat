import { createApp } from 'vue'

import router from './router'

import App from './App.vue'

import settingsManager from './settings'

import 'vfonts/Lato.css'
import 'vfonts/FiraCode.css'

settingsManager.initialize().catch(() => { alert('Unable to initialize settings') })

createApp(App)
  .use(router)
  .mount('#app')
