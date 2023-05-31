<script setup lang="ts">
import { h } from 'vue'

import { RouterLink } from 'vue-router'

import {
  NLayoutSider, NMenu,
  NCard, NAvatar, NSpace, NText, NP,
  type MenuOption
} from 'naive-ui'

import { useConfigStore } from '../store'
import { storeToRefs } from 'pinia'

import { app } from '@tauri-apps/api'
import { computedAsync } from '@vueuse/core'

const configStore = useConfigStore()

const { needsConfig, config } = storeToRefs(configStore)

const menuOptions: MenuOption[] = [
  {
    label: () =>
      h(
        RouterLink,
        {
          to: '/'
        },
        { default: () => 'Home' }
      ),
    key: 'home'
  },
  {
    type: 'group',
    label: 'Students',
    key: 'students',
    children: [
      {
        label: () =>
          h(
            RouterLink,
            {
              to: '/emp_skills'
            },
            { default: () => 'Employability Skills' }
          ),
        key: 'emp_skills'
      },
      {
        label: () =>
          h(
            RouterLink,
            {
              to: '/tech_skills'
            },
            { default: () => 'Technical Skills' }
          ),
        key: 'tech_skills'
      },
      {
        label: () =>
          h(
            RouterLink,
            {
              to: '/credentials'
            },
            { default: () => 'Industry Certifications' }
          ),
        key: 'credentials'
      },
      {
        label: () =>
          h(
            RouterLink,
            {
              to: '/cte_courses'
            },
            { default: () => 'CTE Courses' }
          ),
        key: 'cte_courses'
      }
    ]
  }
]

const appVersion = computedAsync(app.getVersion, '0.0.0')
</script>

<template>
  <n-layout-sider
    content-style="padding: 8px 12px; height: 100%; display: flex; flex-direction: column;"
    :native-scrollbar="false"
    :collapsed-width="0"
    :collapsed="needsConfig"
    bordered
  >
    <n-menu
      :options="menuOptions"
      style="flex-grow:  1;"
      default-value="home"
    />
    <n-card content-style="padding: 8px">
      <n-space
        align="center"
        item-style="display: flex"
      >
        <n-avatar
          round
        >
          {{ config.creds.username?.substring(0, 2).toUpperCase() }}
        </n-avatar>
        <n-text>{{ config.creds.username }}</n-text>
      </n-space>
    </n-card>
    <n-p
      style="text-align: center; margin-top: 8px; font-size: smaller;"
      depth="3"
    >
      CPAT v{{ appVersion }}
    </n-p>
  </n-layout-sider>
</template>
