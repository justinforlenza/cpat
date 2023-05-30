<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

import { NModal, NGradientText, NP, NProgress, NCard, NButton, NAlert } from 'naive-ui'

import { listen, UnlistenFn } from '@tauri-apps/api/event'

interface TaskProgress {
  total: number,
  pass: number
  fail: number
}

const progress = ref({
  total: 1,
  pass: 0,
  fail: 0
})

const percentage = computed(() => Math.round(((progress.value.fail + progress.value.pass) / progress.value.total) * 100))

const isLoading = ref(false)

const isOpen = ref(false)

let startListener: UnlistenFn | undefined, progressListener: UnlistenFn | undefined, stopListener: UnlistenFn | undefined

onMounted(async () => {
  startListener = await listen<TaskProgress>('task_start', (e) => {
    isLoading.value = true
    isOpen.value = true
    progress.value = e.payload
  })
  progressListener = await listen<TaskProgress>('task_progress', (e) => {
    progress.value = e.payload
  })
  stopListener = await listen<TaskProgress>('task_stop', (e) => {
    isLoading.value = false
  })
})

onUnmounted(() => {
  startListener?.()
  progressListener?.()
  stopListener?.()
})

const alertType = computed(() => {
  if (progress.value.pass === progress.value.total) {
    return 'success'
  } else if (progress.value.fail === progress.value.total) {
    return 'error'
  }
  return 'warning'
})

</script>

<template>
  <n-modal
    :show="isOpen"
  >
    <n-card
      style="width: 600px"
      :bordered="false"
      size="huge"
      role="dialog"
      aria-modal="true"
      content-style="text-align: center;"
    >
      <n-gradient-text
        :size="32"
        type="info"
      >
        Hold Tight
      </n-gradient-text>
      <n-p>
        Please wait while your request is processing
      </n-p>
      <n-progress
        type="line"
        indicator-placement="inside"
        :percentage="percentage"
        :processing="isLoading"
      />
      <div
        v-show="!isLoading"
        style="margin-top: 24px"
      >
        <n-alert
          title="Complete"
          :type="alertType"
        >
          <n-p v-if="progress.pass === progress.total">
            Congrats! Your request is fully complete
          </n-p>
          <n-p v-else-if="progress.fail === progress.total">
            Uh-oh! We were ran into issues while processing your requests.
          </n-p>
          <n-p v-else>
            The good news: We were able to complete some of your requests <br>
            The bad news: We were not able to complete all of your requests
          </n-p>
        </n-alert>
        <n-button
          tertiary
          style="margin-top: 24px"
          @click="isOpen = false"
        >
          Continue
        </n-button>
      </div>
    </n-card>
  </n-modal>
</template>

<style scoped>

</style>
