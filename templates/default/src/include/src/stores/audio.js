import { defineStore } from 'pinia'

const supportedExtensions = ['mp3', 'ogg', 'wav']

export const useAudioStore = defineStore('audio', {
  state: () => ({
    audioPlayerVisible: false,
    audioUrl: null,
    feedName: null,
    feedUrl: null,
    linkName: null,
    linkUrl: null,
  }),
  actions: {
    isAudioLink(feedItem) {
      const url = feedItem.enclosureUrl
      if (!url) {
        return false
      }
      const parts = URL.parse(url).pathname.split('/')
      if (parts.length == 0) {
        return false
      }
      const fparts = parts[parts.length - 1].split('.')
      const ext = fparts[fparts.length - 1]
      if (ext.length > 0) {
        return supportedExtensions.indexOf(ext) > -1
      }
      return false
    },

    setAudioData(feedItem) {
      if (!feedItem.enclosureUrl) {
        console.error('invalid audio feed item: ', feedItem)
        return
      }
      this.audioUrl = feedItem.enclosureUrl
      this.linkName = feedItem.title
      this.linkUrl = feedItem.url
      // this.feedName = feedName
      // this.feedLink = feedLink
    },
    showAudioPlayer() {
      this.audioPlayerVisible = true
    },
    hideAudioPlayer() {
      this.audioPlayerVisible = false
    },
  },
  computed: {
    audioPlayerVisible: (state) => state.audioPlayerVisible,
    audioUrl: (state) => state.audioUrl,
    feedName: (state) => state.feedName,
    feedUrl: (state) => state.feedUrl,
    linkName: (state) => state.linkName,
    linkUrl: (state) => state.linkUrl,
  },
})
