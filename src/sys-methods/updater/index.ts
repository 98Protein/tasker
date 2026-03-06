import { invoke } from '@tauri-apps/api/core'

export async function checkForUpdates(): Promise<string | null> {
  return await invoke<string | null>('check_for_updates')
}

export async function downloadAndInstallUpdate(): Promise<void> {
  await invoke('download_and_install_update')
}

export async function getCurrentVersion(): Promise<string> {
  return await invoke<string>('get_current_version')
}
