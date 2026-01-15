<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useThemeStore } from '@/stores/theme'
import { NIcon } from 'naive-ui'
import {
  SettingsOutline as SettingsIcon,
  GlobeOutline as NetworkIcon,
  ServerOutline as BackupIcon,
  InformationCircleOutline as InfoIcon
} from '@vicons/ionicons5'
import { getMacAddress } from '@/sys-methods'

const themeStore = useThemeStore()

// 常规设置
const generalSettings = ref({
  autoStart: false,
  minimizeToTray: true
})

// 网络信息
const networkInfo = ref({
  ipAddress: '获取中...',
  macAddress: '获取中...'
})

// 备份还原
const backupSettings = ref({
  databasePath: 'C:/Users/Admin/AppData/Local/zTasker/data.db'
})

// 关于程序
const appInfo = ref({
  name: 'zTasker Desktop',
  version: 'v1.4.2',
  status: 'Stable',
  license: 'MIT',
  copyright: '© 2024 LOCAL AUTOMATION'
})

// 获取网络信息
const fetchNetworkInfo = async () => {
  try {
    // 获取 MAC 地址
    const mac = await getMacAddress()
    networkInfo.value.macAddress = mac || '无法获取'

    // 获取 IP 地址（通过 WebRTC）
    const pc = new RTCPeerConnection({ iceServers: [] })
    pc.createDataChannel('')
    const offer = await pc.createOffer()
    await pc.setLocalDescription(offer)

    pc.onicecandidate = (ice) => {
      if (!ice || !ice.candidate || !ice.candidate.candidate) return
      const ipRegex = /([0-9]{1,3}(\.[0-9]{1,3}){3})/
      const match = ipRegex.exec(ice.candidate.candidate)
      if (match) {
        networkInfo.value.ipAddress = match[1]
        pc.close()
      }
    }
  } catch (error) {
    console.error('获取网络信息失败:', error)
    networkInfo.value.ipAddress = '无法获取'
    networkInfo.value.macAddress = '无法获取'
  }
}

// 组件挂载时获取网络信息
onMounted(() => {
  fetchNetworkInfo()
})

// 处理立即备份
const handleBackup = () => {
  console.log('执行立即备份')
  window.$message?.success('备份成功')
}

// 处理从文件还原
const handleRestore = () => {
  console.log('从文件还原')
  window.$message?.info('请选择备份文件')
}

// 处理复制路径
const handleCopyPath = () => {
  navigator.clipboard.writeText(backupSettings.value.databasePath)
  window.$message?.success('路径已复制到剪贴板')
}

// 检查更新
const handleCheckUpdate = () => {
  console.log('检查更新')
  window.$message?.info('当前已是最新版本')
}

// 打开用户手册
const handleOpenManual = () => {
  console.log('打开用户手册')
  window.open('https://github.com/yourusername/tasker', '_blank')
}
</script>

<template>
  <div class="system-settings">
    <!-- 设置内容区域 -->
    <div class="settings-content">
      <!-- 第一行：常规设置 + 网络信息 -->
      <div class="settings-row">
        <!-- 常规设置卡片 -->
        <n-card class="setting-card">
          <template #header>
            <div class="card-header">
              <n-icon :component="SettingsIcon" size="20" color="#3b82f6" />
              <span class="card-title">常规设置</span>
            </div>
          </template>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-label">开机自启动</div>
              <div class="setting-desc">在系统启动时自动运行程序</div>
            </div>
            <n-switch v-model:value="generalSettings.autoStart" />
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-label">关闭窗口时最小化到托盘</div>
              <div class="setting-desc">保持程序在后台默认运行</div>
            </div>
            <n-switch v-model:value="generalSettings.minimizeToTray" />
          </div>
        </n-card>

        <!-- 网络信息卡片 -->
        <n-card class="setting-card">
          <template #header>
            <div class="card-header">
              <n-icon :component="NetworkIcon" size="20" color="#3b82f6" />
              <span class="card-title">网络信息</span>
            </div>
          </template>

          <div class="network-info-section">
            <div class="network-info-item">
              <div class="network-info-label">
                <n-icon size="18" color="#10b981">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path fill="currentColor" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10s10-4.48 10-10S17.52 2 12 2M7.07 18.28c.43-.9 3.05-1.78 4.93-1.78s4.51.88 4.93 1.78C15.57 19.36 13.86 20 12 20s-3.57-.64-4.93-1.72m11.29-1.45c-1.43-1.74-4.9-2.33-6.36-2.33s-4.93.59-6.36 2.33A7.95 7.95 0 0 1 4 12c0-4.41 3.59-8 8-8s8 3.59 8 8c0 1.82-.62 3.49-1.64 4.83M12 6c-1.94 0-3.5 1.56-3.5 3.5S10.06 13 12 13s3.5-1.56 3.5-3.5S13.94 6 12 6m0 5c-.83 0-1.5-.67-1.5-1.5S11.17 8 12 8s1.5.67 1.5 1.5S12.83 11 12 11"/>
                  </svg>
                </n-icon>
                <span>IP 地址</span>
              </div>
              <div class="network-info-value">{{ networkInfo.ipAddress }}</div>
            </div>

            <div class="network-info-item">
              <div class="network-info-label">
                <n-icon size="18" color="#f59e0b">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path fill="currentColor" d="M4 6h18V4H4c-1.1 0-2 .9-2 2v11H0v3h14v-3H4zm19 2h-6c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h6c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1m-1 9h-4v-7h4z"/>
                  </svg>
                </n-icon>
                <span>MAC 地址</span>
              </div>
              <div class="network-info-value">{{ networkInfo.macAddress }}</div>
            </div>
          </div>
        </n-card>
      </div>

      <!-- 第二行：备份还原 + 关于程序 -->
      <div class="settings-row">
        <!-- 备份还原卡片 -->
        <n-card class="setting-card">
          <template #header>
            <div class="card-header">
              <n-icon :component="BackupIcon" size="20" color="#3b82f6" />
              <span class="card-title">备份还原</span>
            </div>
          </template>

          <div class="backup-section">
            <div class="setting-label">当前 SQLite 数据库路径</div>
            <div class="database-path">
              <n-input
                :value="backupSettings.databasePath"
                readonly
                style="flex: 1"
              />
              <n-button text @click="handleCopyPath">
                <template #icon>
                  <n-icon>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                      <path fill="currentColor" d="M19 21H8V7h11m0-2H8a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h11a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2m-3-4H4a2 2 0 0 0-2 2v14h2V3h12V1Z"/>
                    </svg>
                  </n-icon>
                </template>
              </n-button>
            </div>

            <div class="backup-actions">
              <n-button type="primary" @click="handleBackup">
                <template #icon>
                  <n-icon>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                      <path fill="currentColor" d="M5 20h14v-2H5m14-9h-4V3H9v6H5l7 7l7-7Z"/>
                    </svg>
                  </n-icon>
                </template>
                立即备份
              </n-button>

              <n-button @click="handleRestore">
                <template #icon>
                  <n-icon>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                      <path fill="currentColor" d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10s10-4.5 10-10S17.5 2 12 2m0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8s8 3.59 8 8s-3.59 8-8 8m4.59-12.42L10 14.17l-2.59-2.58L6 13l4 4l8-8l-1.41-1.42Z"/>
                    </svg>
                  </n-icon>
                </template>
                从文件还原
              </n-button>
            </div>
          </div>
        </n-card>

        <!-- 关于程序卡片 -->
        <n-card class="setting-card">
          <template #header>
            <div class="card-header">
              <n-icon :component="InfoIcon" size="20" color="#3b82f6" />
              <span class="card-title">关于程序</span>
            </div>
          </template>

          <div class="about-section">
            <div class="app-icon">
              <n-icon size="64" color="#3b82f6">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                  <path fill="currentColor" d="M13 2.05v2.02c3.95.49 7 3.85 7 7.93c0 4.08-3.05 7.44-7 7.93v2.02c5.05-.5 9-4.76 9-9.95c0-5.19-3.95-9.45-9-9.95M11 2.05c-5.05.5-9 4.76-9 9.95c0 5.19 3.95 9.45 9 9.95v-2.02c-3.95-.49-7-3.85-7-7.93c0-4.08 3.05-7.44 7-7.93V2.05M12 12.5L16 8l-4 7l-4-7l4 4.5Z"/>
                </svg>
              </n-icon>
            </div>

            <div class="app-info">
              <h3 class="app-name">{{ appInfo.name }}</h3>
              <p class="app-version">版本 {{ appInfo.version }} ({{ appInfo.status }})</p>
            </div>

            <div class="app-actions">
              <n-button type="info" @click="handleCheckUpdate">
                检查更新
              </n-button>
              <n-button @click="handleOpenManual">
                用户手册
              </n-button>
            </div>
          </div>
        </n-card>
      </div>
    </div>
  </div>
</template>

<style scoped>
.system-settings {
  height: 100%;
  overflow-y: auto;
  padding: 24px;
  box-sizing: border-box;
}

/* 页面标题区域 */
.page-header {
  margin-bottom: 32px;
}

.page-title {
  font-size: 28px;
  font-weight: 600;
  margin: 0 0 8px 0;
  color: var(--n-text-color);
}

.page-description {
  font-size: 14px;
  color: var(--n-text-color-3);
  margin: 0;
}

/* 设置内容区域 */
.settings-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
  max-width: 1400px;
  margin: 0 auto;
}

.settings-row {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
  align-items: stretch;
}

/* 卡片样式 */
.setting-card {
  height: 100%;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.card-title {
  font-size: 16px;
  font-weight: 600;
}

/* 设置项样式 */
.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 0;
  border-bottom: 1px solid var(--n-border-color);
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-info {
  flex: 1;
}

.setting-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--n-text-color);
  margin-bottom: 4px;
}

.setting-desc {
  font-size: 12px;
  color: var(--n-text-color-3);
}

/* 网络信息区域 */
.network-info-section {
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding: 8px 0;
}

.network-info-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.network-info-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  font-weight: 500;
  color: var(--n-text-color-2);
}

.network-info-value {
  font-size: 15px;
  font-weight: 600;
  color: var(--n-text-color);
  font-family: 'Consolas', 'Monaco', monospace;
  padding: 8px 12px;
  background-color: var(--n-color-target);
  border-radius: 6px;
  border: 1px solid var(--n-border-color);
}

/* 备份还原区域 */
.backup-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.database-path {
  display: flex;
  align-items: center;
  gap: 8px;
}

.backup-actions {
  display: flex;
  gap: 12px;
}

/* 关于程序区域 */
.about-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 16px;
}

.app-icon {
  margin-bottom: 8px;
}

.app-info {
  margin-bottom: 8px;
}

.app-name {
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 4px 0;
}

.app-version {
  font-size: 14px;
  color: var(--n-text-color-3);
  margin: 0;
}

.app-actions {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.app-footer {
  font-size: 12px;
  color: var(--n-text-color-3);
  line-height: 1.6;
}

.app-footer p {
  margin: 4px 0;
}

/* 响应式布局 */
@media (max-width: 1024px) {
  .settings-row {
    grid-template-columns: 1fr;
  }
}
</style>
