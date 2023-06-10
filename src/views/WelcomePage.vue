<script setup lang="ts">
import { ref } from 'vue'

import {
  NButton,
  NGradientText, NText,
  NAlert, NSpace
} from 'naive-ui'

import CredentialsModal from '../components/CredentialsModal.vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const showModal = ref(false)

const handleUpdate = () => {
  showModal.value = false
  router.push('/')
}
</script>

<template>
  <credentials-modal
    :show="showModal"
    @esc="showModal = false"
    @mask-click="showModal = false"
    @update="handleUpdate"
  />
  <n-space
    id="WelcomePage"
    vertical
    align="center"
    justify="center"
    style="flex-grow: 1; gap: 0;"
  >
    <n-gradient-text :size="64">
      CPAT
    </n-gradient-text>
    <n-text>Career Pathways Automation Tool</n-text>
    <br>
    <n-button
      type="primary"
      size="large"
      @click="showModal = true"
    >
      Get Started
    </n-button>
    <br>
    <n-alert
      title="Disclaimer"
      type="warning"
      style="max-width: 1024px;"
    >
      This software is provided as-is, without any warranties or guarantees of any kind, either express or implied, including but not limited to the implied warranties of merchantability and fitness for a particular purpose.
      The use of this software is at your own risk and we shall not be held responsible for any loss of data, damages, or any other issues that may arise from the use of this software.
    </n-alert>
  </n-space>
</template>
