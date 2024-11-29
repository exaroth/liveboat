import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useFeedsStore = defineStore('feeds', () => {
  var feeds = ref([])
  window.queryFeeds.forEach((f) => {
    feeds.value.push(f)
  })
  window.feeds.forEach((f) => {
    feeds.value.push(f)
  })

  function getFeedById(id) {
    let match = feeds.value.filter((f) => {
      return f.id === id
    })
    if (match.length === 0) {
      return null
    }
    if (match.length > 1) {
      console.error('Found more than 1 feed matching criteria ', match)
    }
    return match[0]
  }

  return { feeds, getFeedById }
})
