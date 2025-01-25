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
    setActiveFeed(feedIndex) {
      this.activeFeed = feedIndex
    },
    getNextFeed(feedIndex) {
      const thisI = this.feeds.findIndex((f) => f.index == feedIndex)
      return this.feeds[thisI + 1] || null
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
