import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useNavStore = defineStore('nav', {
  state: () => ({
    feeds: ref([]),
    activeFeed: 0,
  }),
  actions: {
    addFeed(feedData) {
      for (let f of this.feeds) {
        if (f.index === feedData.index) {
          return
        }
      }
      this.feeds.push(feedData)
      this.feeds.sort((a, b) => {
        return a.index - b.index
      })
    },
    updateFeed(feedData) {
      const idx = this.feeds.findIndex((f) => (f.index = feedData.index))
      if (idx === -1) {
        return
      }
      this.feeds[idx] = feedData
    },
    setActiveFeed(feedIndex) {
      this.activeFeed = feedIndex
    },
    deleteFeed(feedIndex) {
      this.feeds = this.feeds.filter((f) => f.index != feedIndex)
    },
  },
  computed: {
    feeds: (state) => state.feeds,
    activeFeed: (state) => state.activeFeed,
  },
})
