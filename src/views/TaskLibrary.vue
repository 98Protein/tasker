<script setup lang="ts">
import { ref, computed, h, onMounted, onUnmounted } from 'vue'

// 任务数据类型
interface Task {
  id: number
  name: string
  trigger: string
  action: string
  lastRun: string
  nextRun: string
  enabled: boolean
  runningStatus: '' | 'running' | 'waiting'  // 空字符串表示空闲
}

// 模拟任务数据
const tasks = ref<Task[]>([
  {
    id: 1,
    name: '自动备份日志',
    trigger: '定时任务 (每日 02:00)',
    action: '压缩并上传到云端储存',
    lastRun: '2023-10-24 02:00',
    nextRun: '2023-10-25 02:00',
    enabled: true,
    runningStatus: ''
  },
  {
    id: 2,
    name: '屏幕亮度调节',
    trigger: '系统启动 (10分钟)',
    action: '降低屏幕显示亮度 50%',
    lastRun: '2023-10-24 14:20',
    nextRun: '-',
    enabled: false,
    runningStatus: ''
  },
  {
    id: 3,
    name: '临时文件清理',
    trigger: '系统启动',
    action: '清理 /tmp 及系统回收站冗余文件',
    lastRun: '2023-10-23 09:00',
    nextRun: '系统启动时',
    enabled: true,
    runningStatus: 'running'
  },
  {
    id: 4,
    name: '网络状态监控',
    trigger: '循环执行 (每 5 分钟)',
    action: '检测网关延迟并记录到数据库',
    lastRun: '2023-10-24 15:30',
    nextRun: '2023-10-24 15:35',
    enabled: true,
    runningStatus: ''
  },
  {
    id: 5,
    name: 'GitHub 代码同步',
    trigger: '系统唤醒',
    action: '自动拉取最新主干代码库',
    lastRun: '2023-10-24 08:15',
    nextRun: '系统唤醒时',
    enabled: true,
    runningStatus: 'waiting'
  },
  {
    id: 6,
    name: '系统更新',
    trigger: '每周日 04:00',
    action: '自动检查更新并安装',
    lastRun: '2023-10-24 04:00',
    nextRun: '2023-10-29 04:00',
    enabled: true,
    runningStatus: ''
  },
  {
    id: 7,
    name: '系统优化',
    trigger: '每月 1 号',
    action: '系统性能优化',
    lastRun: '2023-10-01 00:00',
    nextRun: '2023-11-01 00:00',
    enabled: true,
    runningStatus: ''
  },
  {
    id: 8,
    name: '系统安全扫描',
    trigger: '每周一 04:00',
    action: '扫描系统安全漏洞',
    lastRun: '2023-10-24 04:00',
    nextRun: '2023-10-30 04:00',
    enabled: true,
    runningStatus: ''
  },
  {
    id: 9,
    name: '系统日志清理',
    trigger: '每月 1 号',
    action: '清理系统日志',
    lastRun: '2023-10-01 00:00',
    nextRun: '-',
    enabled: false,
    runningStatus: ''
  },
  {
    id: 10,
    name: '系统日志分析',
    trigger: '每周一 04:00',
    action: '分析系统日志',
    lastRun: '2023-10-24 04:00',
    nextRun: '2023-10-30 04:00',
    enabled: true,
    runningStatus: ''
  }
])

const searchText = ref('')
const filterStatus = ref<'all' | 'enabled' | 'disabled'>('all')
const filterRunningStatus = ref<'all' | 'running' | 'waiting' | ''>('all')

// 分页状态
const currentPage = ref(1)
const pageSize = ref(10)

// 计算表格高度（用于固定表头）
const tableHeight = ref(500)

const updateTableHeight = () => {
  // 计算可用高度：视口高度 - 头部 - 底部 - 工具栏 - padding - footer
  const viewportHeight = window.innerHeight
  const headerHeight = 64 // 头部高度
  const footerHeight = 48 // 底部高度
  const toolbarHeight = 60 // 工具栏高度
  const bottomFooterHeight = 60 // 页面底部分页高度
  const padding = 48 // 上下 padding

  tableHeight.value = viewportHeight - headerHeight - footerHeight - toolbarHeight - bottomFooterHeight - padding
}

onMounted(() => {
  updateTableHeight()
  window.addEventListener('resize', updateTableHeight)
})

onUnmounted(() => {
  window.removeEventListener('resize', updateTableHeight)
})

// 计算统计数据
const stats = computed(() => {
  const total = tasks.value.length
  const enabled = tasks.value.filter(t => t.enabled).length
  const disabled = total - enabled
  return { total, enabled, disabled }
})

// 筛选后的任务列表
const filteredTasks = computed(() => {
  let result = tasks.value

  // 按启用/禁用状态筛选
  if (filterStatus.value === 'enabled') {
    result = result.filter(t => t.enabled)
  } else if (filterStatus.value === 'disabled') {
    result = result.filter(t => !t.enabled)
  }

  // 按运行状态筛选
  if (filterRunningStatus.value !== 'all') {
    result = result.filter(t => t.runningStatus === filterRunningStatus.value)
  }

  // 按搜索文本筛选
  if (searchText.value) {
    const search = searchText.value.toLowerCase()
    result = result.filter(t =>
      t.name.toLowerCase().includes(search) ||
      t.trigger.toLowerCase().includes(search)
    )
  }

  return result
})

// 分页后的任务列表
const paginatedTasks = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  return filteredTasks.value.slice(start, end)
})

// 总页数
const pageCount = computed(() => {
  return Math.ceil(filteredTasks.value.length / pageSize.value)
})

// 获取运行状态类型（用于标签显示）
const getRunningStatusType = (status: string) => {
  const map: Record<string, 'info' | 'warning' | 'default'> = {
    running: 'info',
    waiting: 'warning',
    '': 'default'
  }
  return map[status] || 'default'
}

// 获取运行状态文本
const getRunningStatusText = (status: string) => {
  const map: Record<string, string> = {
    running: '运行中',
    waiting: '等待中',
    '': '空闲'
  }
  return map[status] || '空闲'
}

const handleToggle = (task: Task) => {
  task.enabled = !task.enabled
  // 禁用时清空运行状态
  if (!task.enabled) {
    task.runningStatus = ''
    task.nextRun = '-'
  }
}

const handleNewTask = () => {
  console.log('新建任务')
}

// 表格列配置
const columns = [
  { title: '任务名称', key: 'name', width: 200, fixed: 'left' as const },
  { title: '触发条件', key: 'trigger', width: 200 },
  {
    title: '运行状态',
    key: 'runningStatus',
    width: 120,
    filterOptionValue: filterRunningStatus.value,
    filter: (value: string, row: Task) => {
      if (filterRunningStatus.value === 'all') return true
      return row.runningStatus === filterRunningStatus.value
    },
    renderFilterMenu: ({ hide }: { hide: () => void }) => {
      return h('div', { class: 'p-2' }, [
        h('div', {
          class: 'cursor-pointer px-3 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded',
          onClick: () => {
            filterRunningStatus.value = 'all'
            hide()
          }
        }, '全部'),
        h('div', {
          class: 'cursor-pointer px-3 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded',
          onClick: () => {
            filterRunningStatus.value = 'running'
            hide()
          }
        }, '运行中'),
        h('div', {
          class: 'cursor-pointer px-3 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded',
          onClick: () => {
            filterRunningStatus.value = 'waiting'
            hide()
          }
        }, '等待中'),
        h('div', {
          class: 'cursor-pointer px-3 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded',
          onClick: () => {
            filterRunningStatus.value = ''
            hide()
          }
        }, '空闲')
      ])
    }
  },
  { title: '上次运行时间', key: 'lastRun', width: 180 },
  { title: '下次运行时间', key: 'nextRun', width: 180 },
  { title: '操作', key: 'actions', width: 100, fixed: 'right' as const }
]
</script>

<template>
  <div class="task-library">
    <!-- 筛选和操作区域 -->
    <div class="toolbar">
      <div class="toolbar-left">
        <n-radio-group v-model:value="filterStatus" size="small">
          <n-radio-button value="all">全部 ({{ stats.total }})</n-radio-button>
          <n-radio-button value="enabled">已启用 ({{ stats.enabled }})</n-radio-button>
          <n-radio-button value="disabled">已禁用 ({{ stats.disabled }})</n-radio-button>
        </n-radio-group>
      </div>
      <div class="toolbar-right">
        <n-input
          v-model:value="searchText"
          placeholder="搜索任务名称、触发条件..."
          class="search-input"
          clearable
        >
          <template #prefix>
            <div class="i-carbon-search" />
          </template>
        </n-input>
        <n-button type="primary" @click="handleNewTask">
          <template #icon>
            <div class="i-carbon-add" />
          </template>
          新建任务
        </n-button>
      </div>
    </div>

    <!-- 任务列表 -->
    <n-data-table
      :columns="columns"
      :data="paginatedTasks"
      :bordered="false"
      :max-height="tableHeight"
    >
      <template #runningStatus="{ row }">
        <n-tag :type="getRunningStatusType(row.runningStatus)" size="small">
          {{ getRunningStatusText(row.runningStatus) }}
        </n-tag>
      </template>
      <template #actions="{ row }">
        <n-switch
          :value="row.enabled"
          @update:value="() => handleToggle(row)"
        />
      </template>
    </n-data-table>

    <!-- 底部分页 -->
    <div class="footer">
      <span>显示 {{ paginatedTasks.length }} / {{ filteredTasks.length }} 个任务（共 {{ stats.total }} 个）</span>
      <n-pagination
        v-model:page="currentPage"
        :page-count="pageCount"
        :page-size="pageSize"
        show-size-picker
        :page-sizes="[10, 20, 30, 50]"
        @update:page-size="pageSize = $event"
      />
    </div>
  </div>
</template>

<style scoped>
.task-library {
  padding: 24px;
}

/* 工具栏区域 */
.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  gap: 16px;
}

.toolbar-left {
  flex-shrink: 0;
}

.toolbar-right {
  display: flex;
  gap: 12px;
  align-items: center;
}

.search-input {
  width: 320px;
}

/* 底部分页 */
.footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 16px;
  padding: 12px 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .toolbar-right {
    flex-direction: column;
  }

  .search-input {
    width: 100%;
  }
}
</style>
