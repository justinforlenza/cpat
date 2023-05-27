<script lang="ts" setup>
import { ref } from 'vue'

import {
  NButton, NForm, NInput, NFormItem, type FormInst,
  NP, NModal, NCard, NSpace
} from 'naive-ui'

const props = defineProps({
  show: Boolean
})

const emit = defineEmits(['esc', 'mask-click'])

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
  username: '',
  password: ''
})

const handleSubmit = () => {
  if (formRef.value !== null) {
    formRef.value.validate(async errors => {
      if (errors === undefined) {
        // await settingsManager.set('username', formValue.value.username)
        // await settingsManager.set('password', formValue.value.password)
      }
    })
  }
}
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
        Enter your Career Pathways Portal credentials
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
            placeholder="jdoe@schools.nyc.gov"
          />
        </n-form-item>
        <n-form-item
          label="Password"
          path="password"
        >
          <n-input
            v-model:value="formValue.password"
            type="password"
            placeholder="Password"
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
          @click="handleSubmit"
        >
          Confirm
        </n-button>
      </n-space>
    </n-card>
  </n-modal>
</template>
