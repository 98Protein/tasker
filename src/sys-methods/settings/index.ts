import { invoke } from '@tauri-apps/api/core'

export interface GeneralSettings {
  auto_start: boolean
  minimize_to_tray: boolean
}

export interface UpdaterSettings {
  last_check_time: string | null
  auto_update: boolean
}

export interface AppSettings {
  general: GeneralSettings
  updater: UpdaterSettings
}

export async function loadSettings(): Promise<AppSettings> {
  return await invoke<AppSettings>('load_settings')
}

export async function saveSettings(settings: AppSettings): Promise<void> {
  await invoke('save_settings', { settings })
}

export async function getMinimizeToTray(): Promise<boolean> {
  return await invoke<boolean>('get_minimize_to_tray')
}
