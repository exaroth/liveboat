<script setup>
import { ref, watchEffect } from 'vue'
import { useFiltersStore } from '../stores/filters'
import { RouterLink } from 'vue-router'
import ArticleItem from './ArticleItem.vue'

const fStore = useFiltersStore()

const props = defineProps({
  feed: {
    type: Object,
    required: true,
  },
  filtered: {
    type: Boolean,
    required: true,
  },
  archived: {
    type: Boolean,
    required: true,
  },
})

const _dateOpts = {
  month: 'short',
  weekday: 'short',
  day: '2-digit',
}
const formatDate = new Intl.DateTimeFormat('en-US', _dateOpts).format

fStore.$subscribe((state) => {
  filterFeedItems(state.payload)
})

const feedItems = ref([])
const filteredFeedItems = ref([])
const initialized = ref(false)

const filterFeedItems = (state) => {
  if (state.searchTerm) {
    filteredFeedItems.value = aggregateItems(_filterByTerm(state.searchTerm))
  } else if (state.filterByDays) {
    filteredFeedItems.value = aggregateItems(_updateItemsWithDate(state.daysBackCount))
  } else {
    filteredFeedItems.value = aggregateItems(_updateItemsWithCount(state.itemCount))
  }
}

const _filterByTerm = (term) => {
  let title = (props.feed.displayTitle || props.feed.title).toLowerCase().split(' ')
  let checker = (arr, target) => target.every((v) => arr.some((vv) => vv.includes(v)))
  return feedItems.value.filter((f) => {
    let fTitle = f.title.toLowerCase().split(' ')
    fTitle.push(f.author.toLowerCase())
    return checker(fTitle.concat(title), term.split(' '))
  })
}
const _updateItemsWithDate = (daysBack) => {
  let d = new Date()
  d.setDate(d.getDate() - daysBack)
  return feedItems.value.filter((f) => {
    return f.date > d
  })
}

const _updateItemsWithCount = (numItems) => {
  return feedItems.value.slice(0, numItems)
}

const _checkSameDate = (d1, d2) => {
  return (
    d1.getDate() === d2.getDate() &&
    d1.getMonth() === d2.getMonth() &&
    d1.getFullYear() === d2.getFullYear()
  )
}

const aggregateItems = (items) => {
  let result = {}
  let now = new Date()
  for (let item of items) {
    let d = ''
    if (!_checkSameDate(item.date, now)) {
      d = formatDate(item.date)
    }
    if (!(d in result)) {
      result[d] = []
    }
    result[d].push(item)
  }
  return result
}

const processFeedItems = (feedItems) => {
  feedItems.sort((a, b) => {
    return b.date - a.date
  })

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
      author: feedItem.author,
      enclosureUrl: feedItem.enclosureUrl,
    })
  }
  return result
}

const feedHasItems = () => {
  return Object.keys(filteredFeedItems.value).length !== 0
}

const resolveFeedPath = (feedId) => {
  let basePath = `feeds/${feedId}`
  if (props.archived) {
    basePath = basePath + '_archive'
  }
  let pathPrefix = window.sitePath || '/'
  if (!pathPrefix.endsWith('/')) {
    pathPrefix = pathPrefix + '/'
  }
  let feedUrl = `${pathPrefix}${basePath}.json?bt=${window.buildTime}`
  return feedUrl
}

watchEffect(async () => {
  if (!initialized.value) {
    const url = resolveFeedPath(props.feed.id)
    let data
    try {
      data = await (await fetch(url)).json()
    } catch {
      console.log('Could not fetch feed data for feed ', url)
      return
    }
    feedItems.value = processFeedItems(data.items)
  }
  initialized.value = true
  if (props.filtered) {
    filterFeedItems(fStore)
  } else {
    filteredFeedItems.value = aggregateItems(feedItems.value)
  }
})
</script>

<template>
  <div class="feed-wrapper" v-if="feedHasItems()">
    <div class="feed-title">
      <router-link :to="{ name: 'feedView', params: { feedId: feed.id } }" v-if="feed.title"
        >{{ feed.displayTitle || feed.title }}
        <span v-if="feed.isQuery">Q</span>
        <span class="item-count">({{ feed.itemCount }})</span></router-link
      >
    </div>
    <div class="feed-item-group" v-for="(items, dateStr) in filteredFeedItems" :key="dateStr">
      <span class="feed-item-date" v-if="dateStr">{{ dateStr }}</span>
      <TransitionGroup name="items" tag="ul">
        <li v-for="(feedItem, index) in items" :key="index" class="feed-item">
          <ArticleItem :feedItem="feedItem" />
        </li>
      </TransitionGroup>
    </div>
  </div>
</template>

<style scoped>
.item-count {
  opacity: 0.6;
  margin-left: 4px;
}
.feed-item {
  line-height: 34px;
  width: 100%;
}
.feed-wrapper {
  padding: 0px 0px 12px 0px;
}

.feed-item-group {
  position: relative;
  transition: visibility 2s;
}
.feed-item-date {
  width: 94px;
  color: var(--color-highlight);
  position: relative;
}
.feed-title {
  padding: 0px 0px 0px 50px;
  margin: 0px 0px 14px 0px;
  width: 100%;
  border-bottom: 2px solid var(--color-accent);
}

.feed-title a {
  display: inline-block;
  background-color: var(--color-accent);
  padding: 2px 20px 0px 20px;
  border-radius: 3px 3px 0px 0px;
}

@media (min-width: 1150px) {
  .feed-item-date {
    text-align: right;
    position: absolute;
    left: -94px;
  }
}

</style>
