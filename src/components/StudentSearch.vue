<script setup lang="ts">
import { ref, h } from 'vue'

import {
  NForm, NFormItemGi, NSelect, NButton,
  NGrid, NSpace, NDataTable,
  useMessage,
  type DataTableColumns, type FormInst
} from 'naive-ui'

import { useAsyncState } from '@vueuse/core'
import invoke, { type Student } from '../invoke'
import { shell } from '@tauri-apps/api'

const emit = defineEmits<{(e: 'search', scholars: Student[]): void, (e: 'checked', checked: number[]): void}>()

const message = useMessage()

const { state: schools, isLoading: schoolsLoading } = useAsyncState(invoke.schools.list, [])

const { state: pathways, isLoading: pathwaysLoading, execute: loadPathways } = useAsyncState(async () => {
  if (filters.value.school !== null) {
    return await invoke.pathways.list(filters.value.school)
  }
  return []
}, [], { immediate: false })

const filters = ref<{school: null | number, pathway: string | null, grade: number | null}>({
  school: null,
  pathway: null,
  grade: null
})

const formRef = ref<FormInst | null>(null)

const rules = {
  school: {
    required: true,
    message: 'School is required'
  },
  pathway: {
    required: true,
    message: 'Pathway is required'
  }
}

const { isLoading, execute: handleSubmit } = useAsyncState(async () => {
  if (formRef.value !== null) {
    await formRef.value.validate()

    try {
      const { school, pathway, grade } = filters.value
      const result = await invoke.students.list(school, pathway, grade)
      students.value = result
      emit('search', result)
    } catch (e) {
      message.error('Student Search Failed', { duration: 6500 })
      console.warn(e)
    }
  }
}, null, {
  immediate: false
})

const students = ref<Student[]>([])

const checkedRowKeysRef = ref<number[]>([])

const columns: DataTableColumns<Student> = [
  {
    type: 'selection'
  },
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
  {
    key: 'open_profile',
    width: 250,
    render (rowData) {
      return h(
        NButton,
        {
          strong: true,
          tertiary: true,
          size: 'small',
          onClick: () => shell.open(`https://careerpathways.nyc/Students/Index/${rowData.id}`)
        },
        { default: () => 'Open Profile' }
      )
    }
  }
]

function handleCheck (rowKeys: number[]) {
  checkedRowKeysRef.value = rowKeys
  emit('checked', rowKeys)
}

</script>

<template>
  <n-form
    ref="formRef"
    :model="filters"
    :rules="rules"
    size="large"
    :disabled="isLoading"
  >
    <n-grid
      :span="24"
      :x-gap="24"
    >
      <n-form-item-gi
        label="School"
        path="school"
        :span="10"
      >
        <n-select
          v-model:value="filters.school"
          :options="schools.map(s => ({value: s.id, label: `${s.name} (${s.dbn})`}))"
          :loading="schoolsLoading"
          filterable
          clearable
          @update:value="() => {loadPathways(); filters.pathway = null}"
        />
      </n-form-item-gi>
      <n-form-item-gi
        :span="10"
        label="Pathway"
        path="pathway"
      >
        <n-select
          v-model:value="filters.pathway"
          :options="pathways.map(p => ({value: p.id, label: p.name}))"
          :disabled="pathways.length === 0 || isLoading"
          :loading="pathwaysLoading"
          filterable
          clearable
        />
      </n-form-item-gi>
      <n-form-item-gi
        :span="4"
        label="Grade"
        path="grade"
      >
        <n-select
          v-model:value="filters.grade"
          :options="Array(6).fill(0).map((_, index) => ({value: index + 1, label: (index+9).toString()}))"
          filterable
          clearable
        />
      </n-form-item-gi>
    </n-grid>
  </n-form>
  <n-space justify="end">
    <n-button
      :loading="isLoading"
      type="primary"
      size="large"
      @click="() => handleSubmit()"
    >
      Search
    </n-button>
  </n-space>

  <n-data-table
    style="margin-top: 24px;"
    :columns="columns"
    :data="students"
    :row-key="(row: Student) => row.id"
    max-height="33vh"
    @update:checked-row-keys="(rowKeys) => handleCheck(rowKeys as number[])"
  />
</template>

<style scoped>
:deep(td[data-col-key="open_profile"]) {
  text-align: center;
}
</style>
