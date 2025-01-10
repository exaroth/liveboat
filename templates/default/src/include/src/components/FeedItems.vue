<script setup>
import { ref, watchEffect, shallowRef } from 'vue'
import { useFiltersStore } from '../stores/filters'
import { RouterLink } from 'vue-router'
import IconMusic from './icons/IconMusic.vue'
import IconMovie from './icons/IconMovie.vue'
import { useEmbedStore } from '../stores/embed'
import { useAudioStore } from '../stores/audio'

const fStore = useFiltersStore()
const embedStore = useEmbedStore()
const audioStore = useAudioStore()

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
  expand: {
    type: Boolean,
    required: true,
  },
  expandedArticles: {
    type: Array,
    required: true,
  },
})

const feedItems = shallowRef([])
const filteredFeedItems = shallowRef([])
const initialized = ref(false)
const emit = defineEmits(['expand-article', 'unexpand-article'])

fStore.$subscribe((state) => {
  filterFeedItems(state.payload)
})

// Utility
// ===============
const _dateOpts = {
  month: 'short',
  weekday: 'short',
  day: '2-digit',
}
const formatDate = new Intl.DateTimeFormat('en-US', _dateOpts).format

const _checkSameDate = (d1, d2) => {
  return (
    d1.getDate() === d2.getDate() &&
    d1.getMonth() === d2.getMonth() &&
    d1.getFullYear() === d2.getFullYear()
  )
}

const truncate = (v) => {
  const newline = v.indexOf('\n')
  return newline > 0 ? v.slice(0, newline) : v
}
// ===============


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
      guid: feedItem.guid,
      content: feedItem.content,
      author: feedItem.author,
      enclosureUrl: feedItem.enclosureUrl,
    })
  }
  return result
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

// Feed/Article expansion
// ======================
const showExpandedArticle = (articleId) => {
  return props.expandedArticles.indexOf(articleId) > -1
}
const handleExpandedArticle = (articleId) => {
  emit('expand-article', articleId)
}
const handleUnexpandedArticle = (articleId) => {
  emit('unexpand-article', articleId)
}
const dispatchExpandItems = () => {
  return {
    feedId: props.feed.id,
    articleIds: feedItems.value.map((i) => i.guid),
  }
}
// ======================

// Embed functionality
// ===================
const showEmbedModal = (feedItem) => {
  if (audioStore.audioPlayerVisible) {
    audioStore.hideAudioPlayer()
  }
  embedStore.setEmbedUrl(feedItem)
  embedStore.showEmbedModal()
}
const showAudioPlayer = (feedItem) => {
  if (embedStore.showModal) {
    embedStore.hideEmbedModal()
  }
  audioStore.setAudioData(feedItem)
  audioStore.showAudioPlayer()
}
// ===================

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
        <span v-if="feed.isQuery" class="feed-query-indicator"></span>
        <span class="item-count">({{ feed.itemCount }})</span></router-link
      >
      <button
        @click="$emit('expand-feed', dispatchExpandItems())"
        class="expand-button"
        title="Expand"
        v-if="!props.expand && !props.archived"
      >
        &#8675;
      </button>
      <button
        @click="$emit('unexpand-feed')"
        class="expand-button"
        title="Unexpand"
        v-if="props.expand && !props.archived"
      >
        &#8673;
      </button>
    </div>
    <div class="feed-item-group" v-for="(items, dateStr) in filteredFeedItems" :key="dateStr">
      <span class="feed-group-date" v-if="dateStr">{{ dateStr }}</span>
      <TransitionGroup name="items" tag="ul">
        <li v-for="(feedItem, index) in items" :key="index" class="feed-item">
          <ArticleItem
            :feedItem="feedItem"
            :expand="showExpandedArticle(feedItem.guid)"
            @expand-article="handleExpandedArticle(feedItem.guid)"
            @unexpand-article="handleUnexpandedArticle(feedItem.guid)"
          />

          <span class="feed-item-link">
            <a
              v-if="embedStore.isEmbeddable(feedItem)"
              @click="showEmbedModal(feedItem)"
              target="_blank"
            >
              {{ truncate(feedItem.title) }}<span class="feed-item-type"><IconMovie /></span
            ></a>
            <a
              v-else-if="audioStore.isAudioLink(feedItem)"
              @click="showAudioPlayer(feedItem)"
              target="_blank"
            >
              {{ truncate(feedItem.title) }}<span class="feed-item-type"><IconMusic /></span>
            </a>
            <a v-else :href="feedItem.url" target="_blank">{{ truncate(feedItem.title) }}</a>
            <button
              @click="handleExpandedArticle(feedItem.guid)"
              class="expand-button article-expand"
              title="Expand"
              v-if="!showExpandedArticle(feedItem.guid)"
            >
              &#8675;
            </button>
            <button
              @click="handleUnexpandedArticle(feedItem.guid)"
              class="expand-button article-expand"
              title="Unexpand"
              v-if="showExpandedArticle(feedItem.guid)"
            >
              &#8673;
            </button>
          </span>
          <span class="feed-item-author" v-if="feedItem.author"> by {{ feedItem.author }}</span>
          <span class="feed-item-domain">({{ feedItem.domain }})</span>
          <div
            :class="{ 'feed-item-details': true, expanded: showExpandedArticle(feedItem.guid) }"
            v-if="showExpandedArticle(feedItem.guid)"
          >
            <span class="feed-item-date"
              ><span class="feed-item-details-desc">Date: </span
              >{{ feedItem.date.toUTCString() }}</span
            >
            <br />

            <span class="feed-item-url"
              ><span class="feed-item-details-desc">URL: </span>{{ feedItem.url }}</span
            ><br />
            <span class="feed-item-contents" v-if="feedItem.content"
              ><span class="feed-item-details-desc">Content: </span>
              <span v-html="feedItem.content"></span
            ></span>
          </div>
        </li>
      </TransitionGroup>
    </div>
  </div>
</template>

<style scoped>
.feed-query-indicator {
  display: inline-block;
  height: 18px;
  width: 18px;
  background-color: var(--color-custom);
  position: relative;
  top: 1px;
  margin: 0 4px;
  border-radius: 50%;
}
.feed-query-indicator::after {
  content: 'Q';
  display: block;
  text-align: center;
  transform: translateY(-10%);
  font-weight: bold;
  font-size: 0.8rem;
}
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
.feed-group-date {
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

.article-expand {
  opacity: 0.7;
  top: 4px;
}
.feed-item-details {
  opacity: 0.8;
  outline: 1px solid rgb(from var(--color-highlight) r g b / 60%);
  padding: 20px;
  border-radius: 10px;
  overflow: hidden;
  display: none;
}
.feed-item-details-desc {
  color: var(--color-highlight);
}
.feed-item-details.expanded {
  display: block;
}
.feed-item-domain {
  opacity: 0.4;
  font-size: 0.72rem;
  margin: 0px 0px 0px 4px;
}

.feed-item-link a {
  padding: 6px 0;
  cursor: pointer;
}

.feed-title svg {
  width: 20px;
  height: 20px;
  position: relative;
  top: 4px;
  left: 4px;
}
.feed-item-author {
  font-size: 12px;
  color: var(--color-highlight);
  opacity: 0.7;
}

.feed-item-type svg {
  width: 18px;
  height: 18px;
  top: 4px;
  position: relative;
  margin-left: 4px;
  opacity: 0.7;
}

.expand-button {
  display: inline-block;
  position: relative;
  top: 1px;
  cursor: pointer;
  border: none;
  background: transparent;
  opacity: 0.8;
  color: var(--color-text);
  font-size: 1.2rem;
}
.expand-button:hover {
  opacity: 1;
}

@media (min-width: 1150px) {
  .feed-group-date {
    text-align: right;
    position: absolute;
    left: -94px;
  }
}
</style>
