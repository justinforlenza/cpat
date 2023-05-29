<script setup lang="ts">
import { h } from 'vue'

import { RouterLink } from 'vue-router'

import {
  NLayoutSider, NMenu,
  NCard, NAvatar, NSpace, NText,
  type MenuOption
} from 'naive-ui'

import { useConfigStore } from '../store'
import { storeToRefs } from 'pinia'

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
            { default: () => 'Credentials' }
          ),
        key: 'credentials'
      }
    ]
  }
]
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
  </n-layout-sider>
</template>
