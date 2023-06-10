<script setup lang="ts" generic="T">
import { ref, computed, h } from 'vue'

import { NModal, NCard, NButton, NSpace, SelectOption, NDataTable, NP, DataTableColumn, NDatePicker, NInput, NSelect } from 'naive-ui'

import { Student } from '../invoke'

import { difference } from 'lodash'

export interface Field<T> {
  key: keyof T
  name: string
  type?: 'text' | 'date' | 'select'
  options?: SelectOption[]
}

const props = defineProps<{
  show: boolean,
  fields: Field<T>[],
  students: Student[]
}>()

const emit = defineEmits<{
  submit: [students: Record<number, T>]
}>()

const data = ref<Record<number, T>>({})

const columns = computed<DataTableColumn<Student>[]>(() => [
  {
    title: 'First Name',
    key: 'first_name'
  },
  {
    title: 'Last Name',
    key: 'last_name'
  },
  {
    title: 'Student ID',
    key: 'osis'
  },
  ...props.fields.map<DataTableColumn<Student>>(f => ({
    title: f.name,
    key: f.key as string,
    render (rowData, rowIndex) {
      switch (f.type) {
        case 'date':
          return h(NDatePicker, {
            type: 'date',
            format: 'MM/dd/yyyy',
            valueFormat: 'yyyy-MM-dd',
            onUpdateFormattedValue: (v) => {
              if (rowData.id in data.value) {
                data.value[rowData.id][f.key] = v as any
              } else {
                data.value[rowData.id] = {
                  [f.key]: v as any
                } as T
              }
            }
          })
        case 'select':
          return h(NSelect, {
            options: f.options,
            onUpdateValue: (v) => {
              if (rowData.id in data.value) {
                data.value[rowData.id][f.key] = v as any
              } else {
                data.value[rowData.id] = {
                  [f.key]: v as any
                } as T
              }
            }
          })
        default:
          return h(NInput, {
            onUpdateValue: (v) => {
              if (rowData.id in data.value) {
                data.value[rowData.id][f.key] = v as any
              } else {
                data.value[rowData.id] = {
                  [f.key]: v as any
                } as T
              }
            }
          })
      }
    }
  }))
])

const isCompleted = computed(() => {
  const a = difference(props.students.map(s => s.id), Object.keys(data.value).map(k => parseInt(k, 10))).length === 0
  if (!a) return false
  const b = Object.values(data.value).map(v => Object.keys(v as object).length === props.fields.map(f => f.key as string).length)
  return b.reduce((p, c) => p && c, true)
})

</script>

<template>
  <n-modal :show="props.show">
    <n-card
      style="width: 90vw; height: 90vh;"
      title="Enter Data"
      :bordered="false"
      size="huge"
      role="dialog"
      aria-modal="true"
    >
      <n-p>Please fill out the missing information for each student</n-p>
      <n-data-table
        :columns="columns"
        :data="students"
        :row-key="(row: Student) => row.id"
        max-height="calc(90vh - 248px)"
        size="small"
      />
      <n-space
        justify="end"
        style="margin-top: 24px;"
      >
        <n-button
          type="primary"
          size="large"
          :disabled="!isCompleted"
          @click="emit('submit', data)"
        >
          Submit
        </n-button>
      </n-space>
    </n-card>
  </n-modal>
</template>
