<script setup lang="ts">
import { NButton, NText, NSpace, NGradientText, NAlert } from 'naive-ui'
</script>

<template>
  <NSpace
    id="WelcomePage"
    vertical
    align="center"
    justify="center"
    size="large"
  >
    <NGradientText :size="42">
      CPAT
    </NGradientText>
    <NText>Career Pathways Automation Tool</NText>
    <br>
    <NButton
      type="primary"
      size="large"
    >
      Get Started
    </NButton>
    <br>
    <NAlert
      title="Disclaimer"
      type="warning"
    >
      The software provided is offered as-is, without any warranties or guarantees of any kind, either express or implied, including but not limited to the implied warranties of merchantability and fitness for a particular purpose.
      The use of this software is at your own risk and we shall not be held responsible for any loss of data, damages, or any other issues that may arise from the use of this software.
    </NAlert>
  </NSpace>
</template>

<style>
#WelcomePage {
  min-height: 100vh;
  max-width: 1024px;
  margin: 0 auto;
}
</style>
