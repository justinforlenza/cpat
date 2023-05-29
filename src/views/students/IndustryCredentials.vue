<script setup lang="ts">
import { NGradientText } from 'naive-ui'

import { useAsyncState } from '@vueuse/core'

const handleClick = async () => {
  console.log('blah')
  // await invoke('get_schools', config.value.creds)
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
