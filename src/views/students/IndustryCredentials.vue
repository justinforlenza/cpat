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
import invoke, { AddCertification, type Student } from '../../invoke'

const message = useMessage()

const selectedStudents = ref<number[]>([])

const students = ref<Student[]>([])

const formRef = ref<FormInst | null>(null)

const rules = {
  certificationId: {
    required: true,
    message: 'Certification is required'
  },
  date: {
    required: true,
    message: 'Date is required'
  },
  status: {
    required: true,
    message: 'Status is required'
  },
  authorityId: {
    required: true,
    message: 'Authority is required'
  }
}

const formValues = ref<AddCertification>({
  certificationId: null,
  date: null,
  status: null,
  authorityId: null
})

const { state: certs, isLoading: certsIsLoading, execute: loadCerts } = useAsyncState(invoke.certifications.list, [], { immediate: false })
const { state: authorities, isLoading: authoritiesIsLoading, execute: loadAuthorities } = useAsyncState(invoke.certifications.authorities, [], { immediate: false })

const { isLoading, execute: handleSubmit } = useAsyncState(async () => {
  if (formRef.value !== null) {
    await formRef.value.validate()
    try {
      await invoke.students.addCertifications(
        selectedStudents.value,
        {
          ...formValues.value,
          date: (new Date(formValues.value.date ?? '').toISOString())
        }
      )
    } catch (e) {
      message.error('Bulk Certification Failed', { duration: 6500 })
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
    Industry Certifications
  </n-gradient-text>
  <student-search
    @checked="(v: number[]) => selectedStudents = v"
    @search="(v: Student[]) => {students = v; loadCerts(undefined, v[0].id)}"
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
        :span="8"
        label="Certification"
        path="certificationId"
      >
        <n-select
          v-model:value="formValues.certificationId"
          :options="certs.map(s => ({value: s.id, label: htmlDecode(s.name) ?? s.name}))"
          :loading="certsIsLoading"
          filterable
          clearable
          @update:value="(v: string) => {loadAuthorities(0, students[0].id, v); formValues.authorityId = null}"
        />
      </n-form-item-gi>
      <n-form-item-gi
        :span="8"
        label="Certifying Authority"
        path="authorityId"
      >
        <n-select
          v-model:value="formValues.authorityId"
          :options="authorities.map(s => ({value: s.id, label: htmlDecode(s.name) ?? s.name}))"
          :loading="authoritiesIsLoading"
          filterable
          clearable
        />
      </n-form-item-gi>
      <n-form-item-gi
        :span="4"
        label="Status"
        path="status"
      >
        <n-select
          v-model:value="formValues.status"
          :options="[{value: 'Pass', label: 'Pass'}, {value: 'Fail', label: 'Fail'}]"
        />
      </n-form-item-gi>
      <n-form-item-gi
        :span="4"
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
      Add Certifications
    </n-button>
  </n-space>
</template>
