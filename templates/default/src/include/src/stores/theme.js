import { defineStore } from 'pinia'

const LOCAL_STORAGE_KEY = 'liveboat-default-theme'
const AVAILABLE_THEMES = {
  default: { cl: null, txt: 'Default Theme' },
  mono: { cl: 'mono-theme', txt: 'Mono' },
  plain: { cl: 'plain-theme', txt: 'Plain' },
  soldark: { cl: 'soldark-theme', txt: 'Solarized Dark' },
  sollight: { cl: 'sollight-theme', txt: 'Solarized Light' },
  tokyo: { cl: 'tokyo-theme', txt: 'Tokyo Night' },
  seabreeze: { cl: 'seabreeze-theme', txt: 'Seabreeze' },
}

const getTheme = () => {
  const savedTheme = localStorage.getItem(LOCAL_STORAGE_KEY) || 'default'
  const theme = AVAILABLE_THEMES[savedTheme]
  document.body.classList.add(theme.cl)
  return {
    themeName: savedTheme,
    theme: theme,
    availableThemes: AVAILABLE_THEMES,
  }
}

export const useThemeStore = defineStore('theme', {
  state: () => getTheme(),
  actions: {
    _saveTheme(theme) {
      localStorage.setItem(LOCAL_STORAGE_KEY, theme)
    },
    selectTheme(theme) {
      this.theme = AVAILABLE_THEMES[theme]
      document.body.setAttribute('class', '')
      document.body.classList.add(this.theme.cl)
      this._saveTheme(theme)
    },
  },
  computed: {
    themeName: (state) => state.themeName,
    theme: (state) => state.theme,
    availableThemes: () => AVAILABLE_THEMES,
  },
})
