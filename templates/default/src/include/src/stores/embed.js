import { defineStore } from 'pinia'

const LOCAL_STORAGE_KEY = 'liveboat-default-embed'

const getDefaultEmbedSettings = () => ({
  configs: embedConfigs,
  showModal: false,
  minimized: false,
  modalEmbedCode: null,
  fallbackUrl: null,
})

const getEmbedSettings = () => {
  let result = getDefaultEmbedSettings()
  const savedSettings = localStorage.getItem(LOCAL_STORAGE_KEY)
  if (savedSettings) {
    result = { ...result, ...JSON.parse(savedSettings) }
  }
  return result
}

const embedConfigs = {
  youtube: {
    matches: [
      /www.youtube.com\/v\/(?<id>[a-zA-Z0-9_-]+)/,
      /www.youtube.com\/watch\?v=(?<id>[a-zA-Z0-9_-]+)/,
    ],
    getEmbedCode: function (url) {
      for (const re of this.matches) {
        let tokens = url.match(re)
        if (!tokens || tokens.length < 2) {
          continue
        }
        let id = tokens[1]
        return `<iframe allow="fullscreen;" src="https://www.youtube.com/embed/${id}?autoplay=1"/>`
      }
      return null
    },
  },
}

export const useEmbedStore = defineStore('embed', {
  state: () => getEmbedSettings(),
  actions: {
    _updateOverflow() {
      if (this.showModal && !this.minimized) {
        document.documentElement.style.overflow = 'hidden'
        return
      }
      document.documentElement.style.overflow = 'auto'
    },
    isEmbeddable(feedItem) {
      let url = feedItem.enclosureUrl || feedItem.url
      for (const embedCfg of Object.values(this.configs)) {
        for (let match of embedCfg.matches) {
          if (match.test(url)) {
            return true
          }
        }
        return false
      }
    },
    setEmbedUrl(feedItem) {
      let url = feedItem.enclosureUrl || feedItem.url
      for (const embedCfg of Object.values(this.configs)) {
        let code = embedCfg.getEmbedCode(url)
        if (code != null) {
          this.modalEmbedCode = code
        }
        this.fallbackUrl = feedItem.url
      }
    },
    showEmbedModal() {
      this.showModal = true
      this._updateOverflow()
    },
    hideEmbedModal() {
      this.showModal = false
      this._updateOverflow()
    },
    minimizeModal(evt) {
      this.minimized = true
      localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify({ minimized: true }))
      this._updateOverflow()
      evt.preventDefault()
    },
    maximizeModal() {
      this.minimized = false
      localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify({ minimized: false }))
      this._updateOverflow()
    },
  },
  computed: {
    showModal: (state) => state.showModal,
    minimized: (state) => state.minimized,
    modalEmbedCode: (state) => state.modalEmbedCode,
    fallbackUrl: (state) => state.fallbackUrl,
  },
})
