import { defineStore } from 'pinia'

const embedConfigs = {
  youtube: {
    matches: [
      /www.youtube.com\/v\/(?<id>[a-zA-Z0-9_-]+)/,
      /www.youtube.com\/watch\?v=(?<id>[a-zA-Z0-9_-]+)/
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
  state: () => ({
    configs: embedConfigs,
    showModal: false,
    modalEmbedCode: null,
    fallbackUrl: null,
  }),
  actions: {
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
      document.documentElement.style.overflow = 'hidden'
    },
    hideEmbedModal() {
      this.showModal = false
      document.documentElement.style.overflow = 'auto'
    },
  },
  computed: {
    showModal: (state) => state.showModal,
    modalEmbedCode: (state) => state.modalEmbedCode,
    fallbackUrl: (state) => state.fallbackUrl,
  },
})
