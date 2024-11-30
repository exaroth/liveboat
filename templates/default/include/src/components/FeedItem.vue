<script setup>
import { ref, watchEffect } from 'vue'
import { useFiltersStore } from '../stores/filters'
import { RouterLink } from 'vue-router'

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
  if (state.filterByDays) {
    filteredFeedItems.value = aggregateItems(_updateItemsWithDate(state.daysBackCount))
  } else {
    filteredFeedItems.value = aggregateItems(_updateItemsWithCount(state.itemCount))
  }
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
    let url = new URL(feedItem.url)
    result.push({
      title: feedItem.title,
      url: feedItem.url,
      date: date,
      domain: url.hostname,
    })
  }
  return result
}

const truncate = (v) => {
  const newline = v.indexOf('\n')
  return newline > 0 ? v.slice(0, newline) : v
}

const feedHasItems = () => {
  return Object.keys(filteredFeedItems.value).length !== 0
}

watchEffect(async () => {
  if (!initialized.value) {
    const url = `/feeds/${props.feed.id}.json`
    let data = await (await fetch(url)).json()
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
      <router-link :to="{ name: 'feedView', params: { feedId: feed.id } }" v-if="feed.title">{{
        feed.displayTitle || feed.title
      }}</router-link>
    </div>
    <div class="feed-item-group" v-for="(items, dateStr) in filteredFeedItems" :key="dateStr">
      <span class="feed-item-date" v-if="dateStr">{{ dateStr }}</span>
      <ul v-for="(feedItem, index) in items" :key="index">
        <li class="feed-item">
          <span class="feed-item-link">
            <a :href="feedItem.url" target="_blank">{{ truncate(feedItem.title) }}</a>
          </span>
          <span class="feed-item-domain">({{ feedItem.domain }})</span>
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.feed-item {
  margin: 0px 0px 8px 0px;
  width: 100%;
}
.feed-item-domain {
  opacity: 0.4;
  font-size: 0.72rem;
  margin: 0px 0px 0px 4px;
}
.feed-wrapper {
  padding: 0px 0px 12px 0px;
}

.feed-title {
  padding: 0px 0px 0px 50px;
  margin: 0px 0px 10px 0px;
  width: 100%;
  border-bottom: 2px solid #3c5e8b;
}

.feed-title a {
  display: inline-block;
  background-color: #3c5e8b;
  padding: 2px 20px 0px 20px;
  border-radius: 3px 3px 0px 0px;
}
.feed-item-group {
  position: relative;
}
.feed-item-date {
  width: 90px;
  text-align: right;
  position: absolute;
  left: -90px;
  color: #73bed3;
}
</style>
