<template>
  <div class="quick-add-container">
    <n-card title="快速添加任务" :bordered="false">
      <n-form ref="formRef" :model="formData" :rules="rules">
        <n-form-item label="任务标题" path="title">
          <n-input
            v-model:value="formData.title"
            placeholder="请输入任务标题"
            @keyup.enter="handleSubmit"
          />
        </n-form-item>
      </n-form>
      <template #footer>
        <n-space justify="end">
          <n-button @click="handleCancel">取消</n-button>
          <n-button type="primary" @click="handleSubmit">确认</n-button>
        </n-space>
      </template>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

const formRef = ref()
const formData = ref({
  title: ''
})

const rules = {
  title: {
    required: true,
    message: '请输入任务标题',
    trigger: 'blur'
  }
}

const handleSubmit = async () => {
  try {
    await formRef.value?.validate()
    // TODO: 添加任务到主列表的逻辑
    console.log('Task added:', formData.value.title)
    await getCurrentWindow().close()
  } catch (error) {
    console.error('Validation failed:', error)
  }
}

const handleCancel = async () => {
  await getCurrentWindow().close()
}
</script>

<style scoped>
.quick-add-container {
  padding: 16px;
  height: 100vh;
  box-sizing: border-box;
}
</style>
