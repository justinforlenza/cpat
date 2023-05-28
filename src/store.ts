import { ref, computed } from 'vue'

import { defineStore } from 'pinia'

import { invoke } from '@tauri-apps/api/tauri'

interface Config {
  creds: {
    username: string | null
    password: string | null
  }
  theme: string | null
}

export const useConfigStore = defineStore('config', () => {
  const config = ref<Config>({
    creds: {
      username: null,
      password: null
    },
    theme: null
  })

  const needsConfig = computed(() => config.value.creds.username === null)

  async function loadConfig (): Promise<Config> {
    const response = await invoke<Config>('get_config')
    config.value = response

    return response
  }

  async function storeConfig (newConfig: Config): Promise<void> {
    await invoke<Config>('set_config', { newConfig })

    config.value = newConfig
  }

  return { config, needsConfig, loadConfig, storeConfig }
})
