<script setup lang="ts">
import { ref } from 'vue'

import {
  NGradientText,
  NForm, NFormItemGi, NSelect, NButton, NDatePicker,
  NGrid, NSpace,
  useMessage,
  type DataTableRowKey, type FormInst
} from 'naive-ui'

import StudentSearch from '../../components/StudentSearch.vue'

import { useAsyncState } from '@vueuse/core'
import invoke, { type Student } from '../../invoke'

const selectedStudents = ref<DataTableRowKey[]>([])

const students = ref<Student[]>([])

const formRef = ref<FormInst | null>(null)

const rules = {
  certification: {
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
  authority: {
    required: true,
    message: 'Authority is required'
  }
}

const formValues = ref({
  certification: null,
  date: null,
  status: null,
  authority: null
})

const { state: certs, isLoading: certsIsLoading, execute: loadCerts } = useAsyncState(invoke.certifications.list, [], { immediate: false })
const { state: authorities, isLoading: authoritiesIsLoading, execute: loadAuthorities } = useAsyncState(invoke.certifications.authorities, [], { immediate: false })

</script>

<template>
  <n-gradient-text :size="40">
    Industry Certifications
  </n-gradient-text>
  <student-search
    @checked="(v: DataTableRowKey[]) => selectedStudents = v"
    @search="(v: Student[]) => {students = v; loadCerts(undefined, v[0].id)}"
  />
  <n-form
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
        path="certification"
      >
        <n-select
          v-model:value="formValues.certification"
          :options="certs.map(s => ({value: s.id, label: s.name}))"
          :loading="certsIsLoading"
          filterable
          clearable
          @update:value="(v: string) => {loadAuthorities(0, students[0].id, v); formValues.authority = null}"
        />
      </n-form-item-gi>
      <n-form-item-gi
        :span="8"
        label="Certifying Authority"
        path="authority"
      >
        <n-select
          v-model:value="formValues.authority"
          :options="authorities.map(s => ({value: s.id, label: s.name}))"
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
          v-model:value="formValues.date"
          type="date"
          format="MM/dd/yyyy"
        />
      </n-form-item-gi>
    </n-grid>
  </n-form>
  <n-space justify="end">
    <n-button
      type="primary"
      size="large"
      :disabled="selectedStudents.length === 0"
    >
      Add Certifications
    </n-button>
  </n-space>
</template>
