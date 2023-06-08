<script setup lang="ts">
import { ref } from 'vue'

import {
  NGradientText,
  NForm, NFormItemGi, NSelect, NButton,
  NGrid, NSpace,
  useMessage,
  type FormInst
} from 'naive-ui'

import StudentSearch from '../../components/StudentSearch.vue'

import { useAsyncState } from '@vueuse/core'
import invoke, { type AddCourse, type Student } from '../../invoke'

const message = useMessage()

const selectedStudents = ref<number[]>([])

const students = ref<Student[]>([])

const formRef = ref<FormInst | null>(null)

const rules = {
  courseId: {
    required: true,
    message: 'Certification is required'
  },
  teacherId: {
    required: true,
    message: 'Date is required'
  },
  status: {
    required: true,
    message: 'Status is required'
  },
  termId: {
    required: true,
    message: 'Authority is required'
  },
  yearId: {
    required: true,
    message: 'Authority is required'
  }
}

const formValues = ref<AddCourse>({
  teacherId: null,
  courseId: null,
  status: null,
  termId: null,
  yearId: null
})

const { state: options, isLoading: optionsIsLoading, execute: loadOptions } = useAsyncState(
  invoke.courses.options,
  { courses: [], years: [], terms: [] },
  { immediate: false }
)
const { state: teachers, isLoading: teachersIsLoading, execute: loadTeachers } = useAsyncState(invoke.courses.teachers, [], { immediate: false })

const { isLoading, execute: handleSubmit } = useAsyncState(async () => {
  if (formRef.value !== null) {
    await formRef.value.validate()
    try {
      await invoke.students.addCourses(
        selectedStudents.value,
        formValues.value
      )
    } catch (e) {
      message.error('Bulk Courses Failed', { duration: 6500 })
      console.warn(e)
    }
  }
}, null, {
  immediate: false
})

function htmlDecode (input: string) {
  const doc = new DOMParser().parseFromString(input, 'text/html')
  return doc.documentElement.textContent
}

</script>

<template>
  <n-gradient-text :size="40">
    CTE Courses
  </n-gradient-text>
  <student-search
    @checked="(v: number[]) => selectedStudents = v"
    @search="(v: Student[]) => {students = v; loadOptions(undefined, v[0].id)}"
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
        :span="7"
        label="Course"
        path="courseId"
      >
        <n-select
          v-model:value="formValues.courseId"
          :options="options.courses.map(s => ({value: s.id, label: htmlDecode(s.name) ?? s.name}))"
          :loading="optionsIsLoading"
          filterable
          clearable
          @update:value="(v: string) => {loadTeachers(0, students[0].id, v); formValues.teacherId = null}"
        />
      </n-form-item-gi>
      <n-form-item-gi
        :span="6"
        label="Teacher"
        path="teacherId"
      >
        <n-select
          v-model:value="formValues.teacherId"
          :options="teachers.map(s => ({value: s.id, label: htmlDecode(s.name) ?? s.name}))"
          :loading="teachersIsLoading"
          filterable
          clearable
        />
      </n-form-item-gi>
      <n-form-item-gi
        :span="5"
        label="Year"
        path="yearId"
      >
        <n-select
          v-model:value="formValues.yearId"
          :options="options.years.map(s => ({value: s.id, label: htmlDecode(s.name) ?? s.name}))"
          :loading="optionsIsLoading"
          filterable
          clearable
        />
      </n-form-item-gi>
      <n-form-item-gi
        :span="3"
        label="Term"
        path="termId"
      >
        <n-select
          v-model:value="formValues.termId"
          :options="options.terms.map(s => ({value: s.id, label: htmlDecode(s.name) ?? s.name}))"
          :loading="optionsIsLoading"
          filterable
          clearable
        />
      </n-form-item-gi>
      <n-form-item-gi
        :span="3"
        label="Status"
        path="status"
      >
        <n-select
          v-model:value="formValues.status"
          :options="[{value: 'true', label: 'Pass'}, {value: 'false', label: 'Fail'}]"
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
      Add Courses
    </n-button>
  </n-space>
</template>
