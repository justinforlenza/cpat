import { createApp } from 'vue'

import router from './router'

import App from './App.vue'

import 'vfonts/Lato.css'
import 'vfonts/FiraCode.css'

createApp(App)
  .use(router)
  .mount('#app')
