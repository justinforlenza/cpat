<script setup lang="ts">
import { computed, onMounted } from 'vue'

import { useRouter } from 'vue-router'

import {
  NConfigProvider, NGlobalStyle,
  NLayout, NLayoutSider,
  useOsTheme, darkTheme
} from 'naive-ui'
import settingsManager from './settings'
import { useSidebarCollapsed } from './store'

const osTheme = useOsTheme()

const theme = computed(() => osTheme.value === 'dark' ? darkTheme : null)

const router = useRouter()

const collapsed = useSidebarCollapsed()

onMounted(async () => {
  try {
    const username = await settingsManager.get('username')
    const password = await settingsManager.get('password')

    if (username === undefined || password === undefined) {
      router.push('/welcome')
    } else {
      collapsed.value = true
    }
  } catch {
    router.push('/welcome')
  }
})

</script>

<template>
  <n-config-provider :theme="theme">
    <n-global-style />
    <n-layout has-sider>
      <n-layout-sider
        content-style="padding: 24px;"
        :native-scrollbar="false"
        bordered
        collapsed-width="0"
        :collapsed="collapsed"
      >
        <p>Home</p>
        <p>Home</p>
        <p>Home</p>
      </n-layout-sider>
      <n-layout
        content-style="padding: 24px;"
        :native-scrollbar="false"
      >
        <router-view />
      </n-layout>
    </n-layout>
  </n-config-provider>
</template>

<style>
* {
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
}
</style>
