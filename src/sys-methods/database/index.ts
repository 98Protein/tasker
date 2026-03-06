import { invoke } from '@tauri-apps/api/core'
import { save, open } from '@tauri-apps/plugin-dialog'

export async function getDatabasePath(): Promise<string> {
  return await invoke<string>('get_database_path')
}

export async function getDefaultBackupFilename(): Promise<string> {
  return await invoke<string>('get_default_backup_filename')
}

export async function backupDatabase(): Promise<string | null> {
  const defaultName = await getDefaultBackupFilename()

  const targetPath = await save({
    defaultPath: defaultName,
    filters: [{ name: 'SQLite Database', extensions: ['db'] }]
  })

  if (!targetPath) return null

  return await invoke<string>('backup_database', { targetPath })
}

export async function restoreDatabase(): Promise<boolean> {
  const sourcePath = await open({
    filters: [{ name: 'SQLite Database', extensions: ['db'] }],
    multiple: false
  })

  if (!sourcePath) return false

  await invoke('restore_database', { sourcePath })
  return true
}
