<script setup lang="ts">
import { NGradientText } from 'naive-ui'

import { useAsyncState } from '@vueuse/core'
import { useConfigStore } from '../../store'
import { storeToRefs } from 'pinia'
import { invoke } from '@tauri-apps/api/tauri'

const configStore = useConfigStore()

const { config } = storeToRefs(configStore)

const handleClick = async () => {
  console.log('blah')
  const schools = await invoke('get_schools', config.value.creds)
  console.log(schools)
}

</script>

<template>
  <button @click="handleClick">
    click me
  </button>
  <br>
  <n-gradient-text :size="40">
    Industry Credentials
  </n-gradient-text>
  <p>Hello world</p>
</template>
