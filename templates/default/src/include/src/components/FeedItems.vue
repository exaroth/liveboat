<script setup>
import { ref, watchEffect, shallowRef, onMounted } from 'vue'
import { useEmbedStore } from '../stores/embed'
import { useAudioStore } from '../stores/audio'
import { useFiltersStore } from '../stores/filters'
import { useFeedItemsStore } from '../stores/feedItems'
import { useMinimizeStore } from '../stores/minimize'
import IconMusic from './icons/IconMusic.vue'
import IconMovie from './icons/IconMovie.vue'
import IconExpand from './icons/IconExpand.vue'
import ItemContent from './ItemContent.vue'
import FeedHeader from './FeedHeader.vue'

const fStore = useFiltersStore()
const embedStore = useEmbedStore()
const audioStore = useAudioStore()
const minimizeStore = useMinimizeStore()
const fItemsStore = useFeedItemsStore()

const { getFeedItems } = fItemsStore

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
  firehose: {
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

const filteredFeedItems = shallowRef([])
const initialized = ref(false)
const emit = defineEmits(['expand-article', 'unexpand-article', 'expand-feed', 'unexpand-feed'])
const itemDetails = ref(null)

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

const filterFeedItems = async (state) => {
  const items = await retrieveItemData()
  if (state.searchTerm) {
    filteredFeedItems.value = aggregateItems(_filterByTerm(items, state.searchTerm))
    return
  }
  if (state.firehose) {
    filteredFeedItems.value = aggregateItems(items)
    return
  }
  if (state.filterByDays) {
    filteredFeedItems.value = aggregateItems(_updateItemsWithDate(items, state.daysBackCount))
    return
  }
  filteredFeedItems.value = aggregateItems(_updateItemsWithCount(items, state.itemCount))
}

const _filterByTerm = (items, term) => {
  let title = (props.feed.displayTitle || props.feed.title).toLowerCase().split(' ')
  let checker = (arr, target) => target.every((v) => arr.some((vv) => vv.includes(v)))
  return items.filter((f) => {
    let fTitle = f.title.toLowerCase().split(' ')
    fTitle.push(f.author.toLowerCase())
    return checker(fTitle.concat(title), term.split(' '))
  })
}
const _updateItemsWithDate = (items, daysBack) => {
  let d = new Date()
  d.setDate(d.getDate() - daysBack)
  return items.filter((f) => {
    return f.date > d
  })
}

const _updateItemsWithCount = (items, numItems) => {
  return items.slice(0, numItems)
}

const feedHasItems = () => {
  return Object.keys(filteredFeedItems.value).length !== 0
}

// Feed/Article expansion
// ======================
const showExpandedArticle = (article) => {
  return props.expandedArticles.indexOf(article.guid) > -1 && article.contentLength > 0
}
const handleExpandedArticle = (articleId) => {
  emit('expand-article', articleId)
}
const handleUnexpandedArticle = (articleId) => {
  emit('unexpand-article', articleId)
}
const handleFeedExpand = (pr) => emit('expand-feed', pr)
const handleFeedUnexpand = (pr) => emit('unexpand-feed', pr)
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
const retrieveItemData = async () => {
  return await getFeedItems(props.feed.id, props.archived)
}

// ===================
watchEffect(async () => {
  if (!initialized.value) {
    await retrieveItemData()
  }
  initialized.value = true
  if (props.filtered) {
    filterFeedItems(fStore)
  } else {
    filteredFeedItems.value = aggregateItems(await retrieveItemData())
  }
})

const updateArticleHighlighting = () => {
  if (!itemDetails.value || itemDetails.value.length === 0) {
    return
  }
  const body = document.body
  const docEl = document.documentElement

  const scrollTop = window.pageYOffset || docEl.scrollTop || body.scrollTop
  const clientTop = docEl.clientTop || body.clientTop || 0
  const center = scrollTop + window.innerHeight / 2
  let visibleDetails = []
  for (let detail of itemDetails.value) {
    const box = detail.getBoundingClientRect()
    const boxTop = box.top + scrollTop - clientTop
    const boxCenter = box.top + scrollTop - clientTop + box.height / 2
    // filter out invisible details
    if (boxTop + box.height < scrollTop || scrollTop + window.innerHeight < boxTop) {
      detail.classList.remove('detail-highlight')
    } else {
      detail.setAttribute('offsetCenter', Math.abs(center - boxCenter).toString())
      visibleDetails.push(detail)
    }
  }
  if (visibleDetails.length === 0) {
    return
  }
  visibleDetails.sort((a, b) => {
    return parseFloat(a.getAttribute('offsetCenter')) - parseFloat(b.getAttribute('offsetCenter'))
  })

  const first = visibleDetails.shift()
  first.classList.add('detail-highlight')
  for (let d of visibleDetails) {
    d.classList.remove('detail-highlight')
  }
}

onMounted(() => {
  setInterval(() => {
    updateArticleHighlighting()
  }, 400)
})
</script>

<template>
  <div class="feed-wrapper" v-if="feedHasItems()">
    <FeedHeader
      :feed="feed"
      :archived="props.archived"
      :expand="props.expand"
      :firehose="props.firehose"
      @expand-feed="handleFeedExpand"
      @unexpand-feed="handleFeedUnexpand"
    />
    <div v-if="!minimizeStore.showFeedMinimized(feed.id)">
      <div class="feed-item-group" v-for="(items, dateStr) in filteredFeedItems" :key="dateStr">
        <span class="feed-group-date" v-if="dateStr">{{ dateStr }}</span>
        <TransitionGroup name="items" tag="ul">
          <li v-for="(feedItem, index) in items" :key="index" class="feed-item">
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
                v-if="!showExpandedArticle(feedItem) && feedItem.contentLength > 0"
              >
                <IconExpand />
              </button>
              <button
                @click="handleUnexpandedArticle(feedItem.guid)"
                class="expand-button article-expand article-unexpand"
                title="Unexpand"
                v-if="showExpandedArticle(feedItem) && feedItem.contentLength > 0"
              >
                <IconExpand />
              </button>
            </span>
            <span class="feed-item-author" v-if="feedItem.author"> by {{ feedItem.author }}</span>
            <span class="feed-item-domain">({{ feedItem.domain }})</span>
            <div
              :class="{ 'feed-item-details': true, expanded: showExpandedArticle(feedItem) }"
              v-if="showExpandedArticle(feedItem)"
              ref="itemDetails"
            >
              <ItemContent :content="feedItem.content" :contentLength="feedItem.contentLength" />
            </div>
          </li>
        </TransitionGroup>
      </div>
    </div>
  </div>
</template>

<style scoped>
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

.article-expand {
  opacity: 0.7;
  top: 4px;
}
.feed-item-details {
  opacity: 0.8;
  padding: 10px 60px;
  overflow: hidden;
  display: none;
}

.feed-item-details.expanded {
  display: block;
}
.feed-item-details.detail-highlight {
  opacity: 1;
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
.article-expand {
  top: 1px;
}
.article-expand svg {
  width: 18px;
  height: 18px;
  position: relative;
  top: 4px;
  color: var(--color-text);
  opacity: 0.8;
}
.article-unexpand svg {
  transform: rotate(90deg);
}

@media (min-width: 1150px) {
  .feed-group-date {
    text-align: right;
    position: absolute;
    left: -94px;
  }
}
@media (max-width: 500px) {
  .feed-item-details {
    padding: 6px 10px;
  }
}
</style>
