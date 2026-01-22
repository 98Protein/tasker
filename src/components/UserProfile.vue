<script setup lang="ts">
import { ref, computed, h } from 'vue'
import { useMessage } from 'naive-ui'
import type { DropdownOption } from 'naive-ui'

const message = useMessage()

// 登录状态管理
const isLoggedIn = ref(false) // 默认未登录

// 模拟用户数据
const userInfo = ref({
  username: '0103租户1',
  avatar: ''
})

// 下拉菜单选项
const dropdownOptions = computed<DropdownOption[]>(() => {
  if (isLoggedIn.value) {
    return [
      {
        label: '退出登录',
        key: 'logout',
        icon: () => h('div', { class: 'i-carbon-logout' })
      }
    ]
  } else {
    return [
      {
        label: '登录',
        key: 'login',
        icon: () => h('div', { class: 'i-carbon-login' })
      }
    ]
  }
})

const handleSelect = (key: string) => {
  if (key === 'logout') {
    handleLogout()
  } else if (key === 'login') {
    handleLogin()
  }
}

const handleLogout = () => {
  isLoggedIn.value = false
  message.success('已退出登录')
}

const handleLogin = () => {
  isLoggedIn.value = true
  message.success('登录成功')
}
</script>

<template>
  <n-dropdown
    trigger="click"
    placement="bottom-end"
    :options="dropdownOptions"
    @select="handleSelect"
  >
    <!-- 胶囊状触发器 -->
    <div
      class="flex items-center gap-2 px-3 py-1.5 rounded-full bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer transition-colors duration-200"
    >
      <!-- 头像 -->
      <n-avatar
        round
        :size="28"
        :src="userInfo.avatar || undefined"
      >
        <template v-if="!userInfo.avatar" #icon>
          <div class="i-carbon-user" />
        </template>
      </n-avatar>
      <!-- 用户名 -->
      <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
        {{ isLoggedIn ? userInfo.username : '离线模式' }}
      </span>
    </div>
  </n-dropdown>
</template>
