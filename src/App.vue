<script setup lang="ts">
import { computed } from 'vue'

import { NConfigProvider, NGlobalStyle, useOsTheme, darkTheme } from 'naive-ui'

const osTheme = useOsTheme()

const theme = computed(() => osTheme.value === 'dark' ? darkTheme : null)

</script>

<template>
  <n-config-provider :theme="theme">
    <n-global-style />
    <router-view />
  </n-config-provider>
</template>

<style>

</style>
