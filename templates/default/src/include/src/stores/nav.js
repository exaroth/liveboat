import { defineStore } from 'pinia'

export const useNavStore = defineStore('nav', {
  state: () => ({
    feeds: [],
    activeFeed: 0,
  }),
  actions: {
    addFeed(feedData) {
      this.feeds.push(feedData)
      this.feeds.sort((a, b) => {
        return a.index - b.index
      })
    },
    updateFeed(feedData) {
      this.feeds[feedData.index] = feedData
    },
    setActiveFeed(feedIndex) {
      this.activeFeed = feedIndex
    },
  },
  computed: {
    feeds: (state) => state.feeds,
    activeFeed: (state) => state.activeFeed,
  },
})
