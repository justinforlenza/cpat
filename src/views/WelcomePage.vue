<script setup lang="ts">
import { ref } from 'vue'

import {
  NButton,
  NGradientText, NText,
  NAlert, NSpace
} from 'naive-ui'

import CredentialsModal from '../components/CredentialsModal.vue'

const showModal = ref(false)

</script>

<template>
  <credentials-modal
    :show="showModal"
    @esc="showModal = false"
    @mask-click="showModal = false"
  />
  <n-space
    id="WelcomePage"
    vertical
    align="center"
    justify="center"
    size="large"
  >
    <n-gradient-text :size="42">
      CPAT
    </n-gradient-text>
    <NText>Career Pathways Automation Tool</NText>
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
    >
      This software is provided as-is, without any warranties or guarantees of any kind, either express or implied, including but not limited to the implied warranties of merchantability and fitness for a particular purpose.
      The use of this software is at your own risk and we shall not be held responsible for any loss of data, damages, or any other issues that may arise from the use of this software.
    </n-alert>
  </n-space>
</template>

<style>
#WelcomePage {
  min-height: 100vh;
  max-width: 1024px;
  margin: 0 auto;
}
</style>
