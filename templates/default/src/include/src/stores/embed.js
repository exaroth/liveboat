import { defineStore } from 'pinia'

const embedConfigs = {
  youtube: {
    matches: [/www.youtube.com\/v\/(?<id>[a-zA-Z0-9_-]+)/],
    getEmbedCode: function (url) {
      for (const re of this.matches) {
        let tokens = url.match(re)
        if (tokens.length < 2) {
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
    isEmbeddable(url) {
      for (const embedCfg of Object.values(this.configs)) {
        for (let match of embedCfg.matches) {
          if (match.test(url)) {
            return true
          }
          return false
        }
      }
    },
    setEmbedUrl(url) {
      console.log('------> ', url)
      for (const embedCfg of Object.values(this.configs)) {
        let code = embedCfg.getEmbedCode(url)
        console.log('code', code)
        if (code != null) {
          this.modalEmbedCode = code
        }
      }
    },
    setEmbedFallbackUrl(url) {
      this.fallbackUrl = url
    },
    showEmbedModal() {
      this.showModal = true
    },
    hideEmbedModal() {
      this.showModal = false
    },
  },
  computed: {
    showModal: (state) => state.showModal,
    modalEmbedCode: (state) => state.modalEmbedCode,
    fallbackUrl: (state) => state.fallbackUrl,
  },
})