<script setup lang="ts">
import { computed } from 'vue'

import {
  NConfigProvider, NGlobalStyle,
  NLayout, NLayoutSider,
  useOsTheme, darkTheme
} from 'naive-ui'

import { useConfigStore } from './store'
import { storeToRefs } from 'pinia'

const osTheme = useOsTheme()

const configStore = useConfigStore()

const { needsConfig, config } = storeToRefs(configStore)

const theme = computed(() => {
  const { theme } = config.value
  switch (theme) {
    case 'light':
      return null
    case 'dark':
      return darkTheme
    default:
      return osTheme.value === 'dark' ? darkTheme : null
  }
})

</script>

<template>
  <n-config-provider :theme="theme">
    <n-global-style />
    <n-layout has-sider>
      <n-layout-sider
        content-style="padding: 24px"
        :native-scrollbar="false"
        :collapsed-width="0"
        :collapsed="needsConfig"
        bordered
      />
      <n-layout
        id="main"
        content-style="padding: 24px; display: flex; min-height: 100vh;"
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
