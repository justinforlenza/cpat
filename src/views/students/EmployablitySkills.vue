<script setup lang="ts">
import { ref } from 'vue'

import {
  NGradientText,
  NForm, NFormItemGi, NSelect, NButton, NDatePicker,
  NGrid, NSpace,
  useMessage,
  type FormInst
} from 'naive-ui'

import StudentSearch from '../../components/StudentSearch.vue'

import { useAsyncState } from '@vueuse/core'
import invoke, { AddSkill, type Student } from '../../invoke'

const message = useMessage()

const selectedStudents = ref<number[]>([96665, 3, 4, 5, 6])

const students = ref<Student[]>([])

const formRef = ref<FormInst | null>(null)

const rules = {
  gradeId: {
    required: true,
    message: 'Grade is required'
  },
  date: {
    required: true,
    message: 'Date is required'
  },
  deadline: {
    required: true,
    message: 'Deadline is required'
  }
}

const formValues = ref<AddSkill>({
  date: null,
  deadline: null,
  gradeId: null
})

const { isLoading, execute: handleSubmit } = useAsyncState(async () => {
  if (formRef.value !== null) {
    await formRef.value.validate()
    try {
      await invoke.students.addSkills(
        selectedStudents.value,
        'employability',
        formValues.value
      )
    } catch (e) {
      message.error('Bulk Skills Failed', { duration: 6500 })
      console.warn(e)
    }
  }
}, null, {
  immediate: false
})
</script>

<template>
  <n-gradient-text :size="40">
    Employability Skills
  </n-gradient-text>

  <student-search
    @checked="(v: number[]) => selectedStudents = v"
    @search="(v: Student[]) => students = v"
  />
  <n-form
    ref="formRef"
    :model="formValues"
    :rules="rules"
    :disabled="students.length === 0"
    size="large"
    style="margin-top: 24px;"
  >
    <n-grid
      :span="24"
      :x-gap="24"
    >
      <n-form-item-gi
        :span="6"
        label="Grade"
        path="gradeId"
      >
        <n-select
          v-model:value="formValues.gradeId"
          :options="Array(6).fill(0).map((_, index) => ({value: index + 1, label: (index+9).toString()}))"
          filterable
          clearable
        />
      </n-form-item-gi>
      <n-form-item-gi
        :span="6"
        label="Date"
        path="date"
      >
        <n-date-picker
          v-model:formatted-value="formValues.date"
          type="date"
          format="MM/dd/yyyy"
          value-format="yyyy-MM-dd"
        />
      </n-form-item-gi>
      <n-form-item-gi
        :span="6"
        label="Deadline"
        path="deadline"
      >
        <n-date-picker
          v-model:formatted-value="formValues.deadline"
          type="date"
          format="MM/dd/yyyy"
          value-format="yyyy-MM-dd"
        />
      </n-form-item-gi>
    </n-grid>
  </n-form>
  <n-space justify="end">
    <n-button
      type="primary"
      size="large"
      :disabled="selectedStudents.length === 0"
      :loading="isLoading"
      @click="() => handleSubmit()"
    >
      Add Skills
    </n-button>
  </n-space>
</template>
