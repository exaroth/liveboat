import { ref } from 'vue'
import { defineStore } from 'pinia'
import { useFeedsStore } from '@/stores/feeds'
import { storeToRefs } from 'pinia'

const resolveFeedPath = (feedId, archived) => {
  let basePath = `feeds/${feedId}`
  if (archived) {
    basePath = basePath + '_archive'
  }
  let pathPrefix = window.sitePath || '/'
  if (!pathPrefix.endsWith('/')) {
    pathPrefix = pathPrefix + '/'
  }
  let feedUrl = `${pathPrefix}${basePath}.json?bt=${window.buildTime}`
  return feedUrl
}

const processFeedItems = (feedItems) => {
  var result = []
  for (let feedItem of feedItems) {
    let date = new Date(feedItem.date * 1000)
    let url
    try {
      url = new URL(feedItem.url)
    } catch {
      console.log('Could not fetch URL for article: ', feedItem)
      continue
    }
    result.push({
      title: feedItem.title,
      url: feedItem.url,
      date: date,
      domain: url.hostname,
      guid: feedItem.guid,
      content: feedItem.content,
      contentLength: feedItem.contentLength,
      commentsUrl: feedItem.commentsUrl,
      author: feedItem.author,
      enclosureUrl: feedItem.enclosureUrl,
    })
  }
  result.sort((a, b) => {
    return b.date - a.date
  })
  return result
}
const fetchFeedItems = async (feedIds, archived) => {
  const promises = feedIds.map((f) => {
    return fetch(resolveFeedPath(f, archived))
  })
  const responses = await Promise.all(promises)
  let result = {}
  for (const resp of responses) {
    const data = await resp.json()
    result[data.id] = processFeedItems(data.items)
  }
  return result
}

export const useFeedItemsStore = defineStore('feedItems', () => {
  var feedItems = {}
  const feedsStore = useFeedsStore()
  const { feeds } = storeToRefs(feedsStore)
  const feedReloadTrigger = ref(false)

  async function getFeedItems(feedId, archived) {
    let feedIds = feedId != null ? [feedId] : feeds.value.map((f) => f.id)
    let result = []
    if (!archived) {
      for (const [idx, fId] of feedIds.entries()) {
        if (feedItems[fId] != null) {
          result = result.concat(feedItems[fId])
          feedIds.splice(idx, 1)
        }
      }
    }

    if (feedIds.length > 0) {
      const data = await fetchFeedItems(feedIds, archived)
      if (!archived) {
        feedItems = { ...feedItems, ...data }
      }
      for (const k in data) {
        result = result.concat(data[k])
      }
    }
    // Always resort firehose articles for now
    if (feedId == null) {
      // cleanup duplicates
      let temp = {}
      for (let i of result) {
        temp[i.guid] = i
      }
      result = Object.values(temp)
      result.sort((a, b) => {
        return b.date - a.date
      })
    }
    return result
  }

  function resetFeedItems() {
    feedItems = {}
    feedReloadTrigger.value = !feedReloadTrigger.value
  }

  return { getFeedItems, resetFeedItems, feedReloadTrigger }
})
