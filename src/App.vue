<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'

import { NConfigProvider, NGlobalStyle, useOsTheme, darkTheme } from 'naive-ui'
import settingsManager from './settings'

const osTheme = useOsTheme()

const theme = computed(() => osTheme.value === 'dark' ? darkTheme : null)

const router = useRouter()

onMounted(async () => {
  try {
    const username = await settingsManager.get('username')
    const password = await settingsManager.get('password')

    if (username === undefined || password === undefined) {
      router.push('/welcome')
    }
  } catch {
    router.push('/welcome')
  }
})

</script>

<template>
  <n-config-provider :theme="theme">
    <n-global-style />
    <router-view />
  </n-config-provider>
</template>

<style>
#app {
  min-height: 100vh;
}
* {
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
}
.n-config-provider{
  height: 100%;
}
</style>
