<script setup lang="ts">
import { ref } from 'vue'

import {
  NGradientText,
  NForm, NFormItemGi, NSelect, NButton,
  NGrid, NSpace
  // useMessage
} from 'naive-ui'

import StudentSearch from '../../components/StudentSearch.vue'
import StudentTableModal, { type Field } from '../../components/StudentTableModal.vue'

// import { useAsyncState } from '@vueuse/core'
import invoke, { type Student, type AddAssessment } from '../../invoke'

// const message = useMessage()

const selectedIds = ref<number[]>([])
const students = ref<Student[]>([])

const assessmentPart = ref('')

const showModal = ref(false)

const fields: Field<AddAssessment>[] = [
  { name: 'Date', key: 'date', type: 'date' },
  { name: 'Score', key: 'score' },
  { name: 'Status', key: 'status', type: 'select', options: [{ label: 'Pass', value: 'Pass' }, { label: 'Fail', value: 'Fail' }] }
]

async function handleSubmit (e: Record<string, AddAssessment>) {
  const data = Object.keys(e).map(k => ({
    ...e[k],
    student_id: parseInt(k, 10)
  })) as unknown

  await invoke.students.addAssesments(assessmentPart.value, data as AddAssessment[])
}
</script>

<template>
  <n-gradient-text :size="40">
    Technical Assessment
  </n-gradient-text>

  <student-search
    @checked="(v: number[]) => selectedIds = v"
    @search="(v: Student[]) => students = v"
  />
  <n-form
    :disabled="selectedIds.length === 0"
    size="large"
    style="margin-top: 24px;"
  >
    <n-grid
      :span="24"
      :x-gap="24"
    >
      <n-form-item-gi
        :span="6"
        label="Technical Assessment Part"
        path="part"
      >
        <n-select
          v-model:value="assessmentPart"
          :options="['Part 1 - Written', 'Part 2 - Pratical', 'Part 3 - School Developed (Portfolio / Project)'].map(v => ({value: v, label: v}))"
          filterable
          clearable
        />
      </n-form-item-gi>
    </n-grid>
  </n-form>
  <n-space justify="end">
    <n-button
      type="primary"
      size="large"
      :disabled="selectedIds.length === 0 || assessmentPart === ''"
      @click="showModal = true"
    >
      Continue
    </n-button>
  </n-space>
  <student-table-modal
    :show="showModal"
    :students="students.filter(s => selectedIds.includes(s.id))"
    :fields="fields"
    @submit="handleSubmit"
  />
</template>
