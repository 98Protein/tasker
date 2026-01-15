<script setup lang="ts">
import { h, computed, ref } from 'vue'
import { RouterView, useRouter, useRoute } from 'vue-router'
import { useThemeStore } from '@/stores/theme'
import LayoutSwitcher from '@/components/LayoutSwitcher.vue'
import UserProfile from '@/components/UserProfile.vue'
import LanguageSwitcher from '@/components/LanguageSwitcher.vue'
import { t } from '@/locales'

// Icons
const renderIcon = (icon: string) => {
  return () => h('div', { class: icon })
}

const themeStore = useThemeStore()
const router = useRouter()
const route = useRoute()

// 当前选中的菜单项
const currentMenuKey = computed(() => {
  if (route.path === '/') return 'task-library'
  if (route.path === '/run-history') return 'run-history'
  if (route.path === '/system-settings') return 'system-settings'
  return 'task-library'
})

// 资源监控数据
const resources = ref({
  sqlite: '12.4 MB',
  cpu: 12,
  memory: 45,
  disk: 22,
  status: '系统正常运行中'
})

const menuOptions = computed(() => [
  {
    label: t('menu.taskLibrary'),
    key: 'task-library',
    icon: renderIcon('i-carbon-task'),
  },
  {
    label: t('menu.runHistory'),
    key: 'run-history',
    icon: renderIcon('i-carbon-time'),
  },
  {
    label: t('menu.systemSettings'),
    key: 'system-settings',
    icon: renderIcon('i-carbon-settings'),
  },
])

const handleUpdateValue = (key: string) => {
  if (key === 'task-library') router.push('/')
  else router.push('/' + key)
}
</script>

<template>
  <n-layout class="h-100vh" content-style="display: flex; flex-direction: column;">
    <n-layout-header bordered class="h-16 flex items-center justify-between px-6">
      <div class="flex items-center gap-8">
        <div class="flex items-center font-bold text-xl">
          <div class="i-carbon-cloud-service-management text-2xl mr-2 text-teal-600" />
          <span>Tasker</span>
        </div>
        <n-menu
          mode="horizontal"
          :value="currentMenuKey"
          :options="menuOptions"
          @update:value="handleUpdateValue"
        />
      </div>
      <div class="flex items-center gap-4">
        <slot name="header-extra"></slot>
        <LayoutSwitcher />
        <LanguageSwitcher />
        <button class="icon-btn i-carbon-sun dark:i-carbon-moon text-xl" @click="themeStore.toggleTheme" />
        <UserProfile />
      </div>
    </n-layout-header>
    <n-layout-content class="flex-1" content-style="padding: 0; overflow: hidden;">
      <RouterView />
    </n-layout-content>
    <n-layout-footer bordered class="footer">
      <div class="footer-left">
        <span class="footer-item">
          <div class="i-carbon-data-base mr-1" />
          SQLite: {{ resources.sqlite }}
        </span>
        <span class="footer-item">
          <div class="i-carbon-circle-filled mr-1 text-green-500" />
          {{ resources.status }}
        </span>
      </div>
      <div class="footer-right">
        <span class="footer-item">
          CPU
          <n-progress type="line" :percentage="resources.cpu" :show-indicator="false" style="width: 60px; margin: 0 8px" />
          {{ resources.cpu }}%
        </span>
        <span class="footer-item">
          内存
          <n-progress type="line" :percentage="resources.memory" status="success" :show-indicator="false" style="width: 60px; margin: 0 8px" />
          {{ resources.memory }}%
        </span>
        <span class="footer-item">
          磁盘
          <n-progress type="line" :percentage="resources.disk" status="warning" :show-indicator="false" style="width: 60px; margin: 0 8px" />
          {{ resources.disk }}%
        </span>
      </div>
    </n-layout-footer>
  </n-layout>
</template>
<style scoped>
.footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  font-size: 12px;
  color: #666;
}

.footer-left,
.footer-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.footer-item {
  display: flex;
  align-items: center;
}
</style>
