<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

interface HistoryRecord {
  id: number
  taskName: string
  status: 'success' | 'failed' | 'running'
  startTime: string
  duration: string
  message: string
}

const records = ref<HistoryRecord[]>([
  {
    id: 1,
    taskName: '自动备份日志',
    status: 'success',
    startTime: '2023-10-24 02:00:15',
    duration: '2.3s',
    message: '备份完成，已上传 3 个文件'
  },
  {
    id: 2,
    taskName: '网络状态监控',
    status: 'success',
    startTime: '2023-10-24 15:30:00',
    duration: '0.8s',
    message: '网络正常，延迟 12ms'
  },
  {
    id: 3,
    taskName: '临时文件清理',
    status: 'failed',
    startTime: '2023-10-23 09:00:00',
    duration: '1.2s',
    message: '权限不足，无法删除部分文件'
  }
])

// 搜索和筛选状态
const searchText = ref('')
const filterStatus = ref<'all' | 'success' | 'failed'>('all')
const dateRange = ref<[number, number] | null>(null)

// 分页状态
const currentPage = ref(1)
const pageSize = ref(10)

// 计算统计数据
const stats = computed(() => {
  const total = records.value.length
  const success = records.value.filter(r => r.status === 'success').length
  const failed = records.value.filter(r => r.status === 'failed').length
  return { total, success, failed }
})

// 筛选后的记录列表
const filteredRecords = computed(() => {
  let result = records.value

  // 按状态筛选
  if (filterStatus.value !== 'all') {
    result = result.filter(r => r.status === filterStatus.value)
  }

  // 按时间范围筛选
  if (dateRange.value) {
    const [startTime, endTime] = dateRange.value
    result = result.filter(r => {
      const recordTime = new Date(r.startTime).getTime()
      return recordTime >= startTime && recordTime <= endTime
    })
  }

  // 按搜索文本筛选
  if (searchText.value) {
    const search = searchText.value.toLowerCase()
    result = result.filter(r =>
      r.taskName.toLowerCase().includes(search) ||
      r.message.toLowerCase().includes(search)
    )
  }

  return result
})

// 分页后的记录列表
const paginatedRecords = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  return filteredRecords.value.slice(start, end)
})

// 总页数
const pageCount = computed(() => {
  return Math.ceil(filteredRecords.value.length / pageSize.value)
})

const getStatusType = (status: string) => {
  const map: Record<string, 'success' | 'error' | 'info'> = {
    success: 'success',
    failed: 'error',
    running: 'info'
  }
  return map[status] || 'info'
}

const getStatusText = (status: string) => {
  const map: Record<string, string> = {
    success: '成功',
    failed: '失败',
    running: '运行中'
  }
  return map[status] || status
}

// 计算表格高度（用于固定表头）
const tableHeight = ref(500)

const updateTableHeight = () => {
  const viewportHeight = window.innerHeight
  const headerHeight = 64
  const footerHeight = 48
  const padding = 48

  tableHeight.value = viewportHeight - headerHeight - footerHeight - padding
}

onMounted(() => {
  updateTableHeight()
  window.addEventListener('resize', updateTableHeight)
})

onUnmounted(() => {
  window.removeEventListener('resize', updateTableHeight)
})
</script>

<template>
  <div class="run-history">
    <!-- 搜索和筛选工具栏 -->
    <div class="toolbar">
      <div class="toolbar-left">
        <n-radio-group v-model:value="filterStatus" size="small">
          <n-radio-button value="all">全部 ({{ stats.total }})</n-radio-button>
          <n-radio-button value="success">成功 ({{ stats.success }})</n-radio-button>
          <n-radio-button value="failed">失败 ({{ stats.failed }})</n-radio-button>
        </n-radio-group>
      </div>
      <div class="toolbar-right">
        <n-date-picker
          v-model:value="dateRange"
          type="datetimerange"
          clearable
          placeholder="选择时间范围"
          style="width: 360px"
        />
        <n-input
          v-model:value="searchText"
          placeholder="搜索任务名称、执行信息..."
          class="search-input"
          clearable
        >
          <template #prefix>
            <div class="i-carbon-search" />
          </template>
        </n-input>
      </div>
    </div>
    <n-data-table
      :columns="[
        { title: '任务名称', key: 'taskName', width: 200, fixed: 'left' },
        { title: '状态', key: 'status', width: 100 },
        { title: '开始时间', key: 'startTime', width: 180 },
        { title: '耗时', key: 'duration', width: 100 },
        { title: '执行信息', key: 'message', minWidth: 300 }
      ]"
      :data="paginatedRecords"
      :bordered="false"
      :max-height="tableHeight"
    >
      <template #status="{ row }">
        <n-tag :type="getStatusType(row.status)" size="small">
          {{ getStatusText(row.status) }}
        </n-tag>
      </template>
    </n-data-table>

    <!-- 底部分页 -->
    <div class="footer">
      <span>显示 {{ paginatedRecords.length }} / {{ filteredRecords.length }} 条记录（共 {{ stats.total }} 条）</span>
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
.run-history {
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
  width: 280px;
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
