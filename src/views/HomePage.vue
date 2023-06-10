<script setup lang="ts">
import { NGradientText, NAlert, NP, NSpace } from 'naive-ui'
</script>

<template>
  <n-space
    vertical
    style="max-width: 768px; margin: 0 auto;"
  >
    <n-gradient-text :size="64">
      Home
    </n-gradient-text>
    <n-p>
      The Career Pathways Automation Tool is a powerfull solution developed to address the inherent inefficiencies of the existing Career Pathways Portal website.
      Recognizing the limitations and time-consuming processes of the portal, this program aims to streamline tasks and optimize user productivity.
      The Career Pathways Portal Automation Program aims to address these inefficiencies by automating many repetitive tasks.
    </n-p>
    <n-p>
      This tool was developed out of passion (and a little frustration) to meet a specific set of needs.
      It may fit your needs, it may not. Use at your own discretion and be sure to test before using at large scale.
    </n-p>
    <n-alert
      title="Disclaimer"
      type="warning"
    >
      This software is provided as-is, without any warranties or guarantees of any kind, either express or implied, including but not limited to the implied warranties of merchantability and fitness for a particular purpose.
      The use of this software is at your own risk and we shall not be held responsible for any loss of data, damages, or any other issues that may arise from the use of this software.
    </n-alert>
  </n-space>
</template>
