<script setup lang="ts">
import { computed } from 'vue'

import {
  NConfigProvider, NGlobalStyle, NMessageProvider,
  NLayout,
  useOsTheme, darkTheme
} from 'naive-ui'
import { themeOverrides } from './theme'

import NavigationAside from './components/NavigationAside.vue'
import CredentialsModal from './components/CredentialsModal.vue'
import ProgressDialog from './components/ProgressDialog.vue'

import { useConfigStore } from './store'
import { storeToRefs } from 'pinia'

const osTheme = useOsTheme()

const configStore = useConfigStore()

const { config } = storeToRefs(configStore)

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

const showCredentials = computed(() => config.value.creds.username !== null && config.value.creds.password === null)

</script>

<template>
  <n-config-provider
    :theme="theme"
    :theme-overrides="themeOverrides"
  >
    <n-global-style />
    <n-message-provider placement="bottom">
      <credentials-modal :show="showCredentials" />
      <progress-dialog />
      <n-layout has-sider>
        <navigation-aside />
        <n-layout
          id="main"
          content-style="padding: 24px; display: flex; min-height: 100vh; flex-direction: column;"
          :native-scrollbar="false"
        >
          <router-view />
        </n-layout>
      </n-layout>
    </n-message-provider>
  </n-config-provider>
</template>

<style>
* {
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
}
</style>
