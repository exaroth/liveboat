import { defineStore } from 'pinia'

const LOCAL_STORAGE_KEY = 'liveboat-default-minimize'

const getMinimizedFeeds = () => {
  const savedMinimizedFeeds = localStorage.getItem(LOCAL_STORAGE_KEY)
  if (savedMinimizedFeeds) {
    return JSON.parse(savedMinimizedFeeds)
  }
  return {
    minimizedFeeds: [],
  }
}

export const useMinimizeStore = defineStore('minimize', {
  state: () => getMinimizedFeeds(),
  actions: {
    _saveMinimizedFeeds() {
      localStorage.setItem(
        LOCAL_STORAGE_KEY,
        JSON.stringify({
          minimizedFeeds: this.minimizedFeeds,
        }),
      )
    },
    addMinimizedFeed(feedId) {
      this.minimizedFeeds.push(feedId)
      this._saveMinimizedFeeds()
    },
    removeMinimizedFeed(feedId) {
      this.minimizedFeeds = this.minimizedFeeds.filter((i) => {
        return i !== feedId
      })
      this._saveMinimizedFeeds()
    },
    showFeedMinimized(feedId) {
      return this.minimizedFeeds.indexOf(feedId) > -1
    },
  },
  computed: {
    minimizedFeeds: (state) => state.minimizedFeeds,
  },
})
