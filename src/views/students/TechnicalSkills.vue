<script setup lang="ts">
import { ref } from 'vue'

import {
  NGradientText,
  type DataTableRowKey
} from 'naive-ui'

import StudentSearch from '../../components/StudentSearch.vue'

const selectedStudents = ref<DataTableRowKey[]>([])

</script>

<template>
  <n-gradient-text :size="40">
    Technical Skills
  </n-gradient-text>
  <student-search @checked="(v: DataTableRowKey[]) => selectedStudents = v" />
</template>
