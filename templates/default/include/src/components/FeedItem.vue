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

fStore.$subscribe((state) => {
  filterFeedItems(state.payload)
})

const feedItems = ref([])
const filteredFeedItems = ref([])
const initialized = ref(false)

const filterFeedItems = (state) => {
  if (state.filterByDays) {
    _updateItemsWithDate(state.daysBackCount)
  } else {
    _updateItemsWithCount(state.itemCount)
  }
}

const _updateItemsWithDate = (daysBack) => {
  filteredFeedItems.value = feedItems.value.slice(0, 10)
  let d = new Date()
  d.setDate(d.getDate() - daysBack)
  filteredFeedItems.value = feedItems.value.filter((f) => {
    return f.date > d
  })
}

const _updateItemsWithCount = (numItems) => {
  filteredFeedItems.value = feedItems.value.slice(0, numItems)
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
      dateStr: date.toLocaleDateString(),
      domain: url.hostname,
    })
  }
  return result
}

const truncate = (v) => {
  const newline = v.indexOf('\n')
  return newline > 0 ? v.slice(0, newline) : v
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
    filteredFeedItems.value = feedItems.value
  }
})
</script>

<template>
  <div class="feed-wrapper">
    <h3 class="feed-title" v-if="feed.title"></h3>
    <router-link :to="{ name: 'feedView', params: {feedId: feed.id }}" v-if="feed.title">{{ feed.displayTitle || feed.title }}</router-link>
    <ul v-if="filteredFeedItems.length > 0">
      <li v-for="(feedItem, index) in filteredFeedItems" :key="index">
        <span class="feed-item-date">{{ feedItem.dateStr }}</span>
        <span class="feed-item-link">
          <a :href="feedItem.url" target="_blank">{{ truncate(feedItem.title) }}</a>
        </span>
        <span class="feed-item-domain">({{ feedItem.domain }})</span>
      </li>
    </ul>
  </div>
</template>
