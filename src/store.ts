import { ref } from 'vue'

import { createGlobalState } from '@vueuse/core'

export const useSidebarCollapsed = createGlobalState(
  () => {
    const state = ref(true)
    return state
  }
)
