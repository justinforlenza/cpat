<script lang="ts" setup>
import { ref, watch } from 'vue'

import {
  NButton, NForm, NInput, NFormItem, type FormInst,
  NP, NModal, NCard, NSpace,
  useMessage
} from 'naive-ui'

import { useConfigStore } from '../store'
import { storeToRefs } from 'pinia'

import { useAsyncState } from '@vueuse/core'
import { invoke } from '@tauri-apps/api/tauri'

const message = useMessage()

const props = defineProps({
  show: Boolean
})

const emit = defineEmits(['esc', 'mask-click', 'update'])

const configStore = useConfigStore()

const { config } = storeToRefs(configStore)

const formRules = {
  username: {
    required: true,
    message: 'Email is required'
  },
  password: {
    required: true,
    message: 'Password is required'
  }
}

const formRef = ref<FormInst | null>(null)
const formValue = ref({
  password: null,
  username: null
})

const { isLoading, execute: handleSubmit } = useAsyncState(async (): Promise<void> => {
  if (formRef.value !== null) {
    await formRef.value.validate()
    try {
      await invoke('check_credentials', formValue.value)
      await configStore.storeConfig({
        ...config.value,
        creds: formValue.value
      })
      emit('update')
    } catch (e) {
      message.error('Login Failed', { duration: 6500 })
      console.warn(e)
    }
  }
}, null, { immediate: false })

watch(config, (value) => {
  formValue.value = JSON.parse(JSON.stringify(value.creds))
})

</script>

<template>
  <n-modal
    :show="props.show"
    @esc="emit('esc')"
    @mask-click="emit('mask-click')"
  >
    <n-card
      style="width: 600px"
      title="Enter Credentials"
      :bordered="false"
      size="huge"
      role="dialog"
      aria-modal="true"
    >
      <n-p>
        Enter your Career Pathways Portal credentials to continue
      </n-p>
      <n-form
        ref="formRef"
        :model="formValue"
        :rules="formRules"
      >
        <n-form-item
          label="Email"
          path="username"
        >
          <n-input
            v-model:value="formValue.username"
            placeholder="jdoe"
          />
        </n-form-item>
        <n-form-item
          label="Password"
          path="password"
        >
          <n-input
            v-model:value="formValue.password"
            type="password"
            placeholder="*******"
            @keydown.enter="handleSubmit"
          />
        </n-form-item>
      </n-form>
      <n-space
        justify="end"
        style="margin-top: 24px;"
      >
        <n-button
          type="primary"
          secondary
          :loading="isLoading"
          @click="handleSubmit"
        >
          Confirm
        </n-button>
      </n-space>
    </n-card>
  </n-modal>
</template>
