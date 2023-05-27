import { SettingsManager } from 'tauri-settings'

interface SettingsSchema {
  theme?: 'dark' | 'light'
  username?: string
  password?: string
}

const settingsManager = new SettingsManager<SettingsSchema>({}, { fileName: 'settings' })

export default settingsManager
