<script setup>
import { ref } from 'vue'
import FeedItems from './FeedItems.vue'
import AudioPlayer from './AudioPlayer.vue'
import FilterBox from './FilterBox.vue'
import FeedNavigator from './FeedNavigator.vue'
import EmbedModal from './EmbedModal.vue'
import IconNotFound from '@/components/icons/IconNotFound.vue'
import { useFeedsStore } from '@/stores/feeds'
import { useEmbedStore } from '@/stores/embed'
import { useAudioStore } from '@/stores/audio'
import { useFiltersStore } from '@/stores/filters'
import { storeToRefs } from 'pinia'

const props = defineProps({
  filtered: {
    type: Boolean,
    required: true,
  },
  archived: {
    type: Boolean,
    required: true,
  },
  showNav: {
    type: Boolean,
    required: true,
  },
})

const showLoadingSpinner = ref(true)
const totalItemCount = ref(0)
const expandedFeed = ref(null)
const expandedArticles = ref([])
const embedStore = useEmbedStore()
const audioStore = useAudioStore()
const feedsStore = useFeedsStore()
const filterStore = useFiltersStore()

const { feeds } = storeToRefs(feedsStore)

const handleFeedExpand = async (expandData) => {
  expandData.then((res) => {
    expandedFeed.value = res.feedId
    expandedArticles.value = res.articleIds
  })
}

const handleFeedUnexpand = () => {
  expandedFeed.value = null
  expandedArticles.value = []
}

const handleArticleExpand = (articleId) => {
  expandedArticles.value.push(articleId)
}

const handleArticleUnexpand = (articleId) => {
  expandedArticles.value = expandedArticles.value.filter((i) => {
    return i !== articleId
  })
}

const showExpandedFeed = (feed) => {
  if (expandedFeed.value == null) {
    return false
  }
  return feed.id === expandedFeed.value
}

const generateFirehoseFeed = () => {
  return {
    id: null,
    url: null,
    title: 'Firehose',
    displayTitle: 'Firehose',
    itemCount: 0,
    isQuery: false,
    tags: [],
  }
}

const handleLoadingFeed = () => {
  totalItemCount.value = 0
  showLoadingSpinner.value = true
}
const handleLoadedFeed = (numItems) => {
  totalItemCount.value += numItems
  showLoadingSpinner.value = false
}
</script>

<template>
  <div class="loading-spinner" v-if="showLoadingSpinner" />
  <div id="no-feeds-found-indicator" v-if="totalItemCount === 0 && !showLoadingSpinner">
    <IconNotFound />
    <h2>No feeds found</h2>
  </div>
  <FeedNavigator v-if="!props.archived" :show="showNav" />
  <FilterBox />
  <div v-if="filterStore.firehose">
    <FeedItems
      :feed="generateFirehoseFeed()"
      :filtered="props.filtered"
      :archived="props.archived"
      :firehose="true"
      :expandedArticles="expandedArticles"
      @expand-article="handleArticleExpand"
      @unexpand-article="handleArticleUnexpand"
      @feed-loading="handleLoadingFeed"
      @feed-loaded="handleLoadedFeed"
    />
  </div>
  <div class="feed-list-wrapper" v-else v-for="(feed, index) in feeds" :key="index">
    <Transition>
      <FeedItems
        :feed="feed"
        :filtered="props.filtered"
        :archived="props.archived"
        :expand="showExpandedFeed(feed)"
        :firehose="false"
        :expandedArticles="expandedArticles"
        :feedIndex="index"
        @expand-feed="handleFeedExpand"
        @unexpand-feed="handleFeedUnexpand"
        @expand-article="handleArticleExpand"
        @unexpand-article="handleArticleUnexpand"
        @feed-loading="handleLoadingFeed"
        @feed-loaded="handleLoadedFeed"
      />
    </Transition>
  </div>
  <EmbedModal
    v-if="embedStore.showModal"
    :embedCode="embedStore.modalEmbedCode"
    :fallbackUrl="embedStore.fallbackUrl"
  />
  <AudioPlayer
    v-if="audioStore.audioPlayerVisible"
    :title="audioStore.linkName"
    :url="audioStore.linkUrl"
    :feedTitle="audioStore.feedName"
    :feedLink="audioStore.feedUrl"
    :file="audioStore.audioUrl"
  />
</template>

<style scoped>
.v-enter-active,
.v-leave-active {
  transition: all 0.2s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

#no-feeds-found-indicator {
  position: absolute;
  left: 50%;
  top: 30%;
  transform: translateX(-50%);
}

#no-feeds-found-indicator svg {
  width: 60px;
  height: 60px;
  display: block;
  margin: auto;
  stroke: #c7cfcc;
}

@media (max-width: 1000px) {
  #no-feeds-found-indicator {
    top: auto;
    bottom: 10%;
  }
}

@media (max-height: 400px) {
  #no-feeds-found-indicator {
    display: none;
  }
}
</style>
