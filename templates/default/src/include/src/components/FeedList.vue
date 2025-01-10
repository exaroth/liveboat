<script setup>
import { ref } from 'vue'
import FeedItems from './FeedItems.vue'
import AudioPlayer from './AudioPlayer.vue'
import FilterBox from './FilterBox.vue'
import EmbedModal from './EmbedModal.vue'
import { useFeedsStore } from '@/stores/feeds'
import { useEmbedStore } from '@/stores/embed'
import { useAudioStore } from '@/stores/audio'
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
})

const expandFeed = ref(null)
const embedStore = useEmbedStore()
const audioStore = useAudioStore()
const feedsStore = useFeedsStore()
const { feeds } = storeToRefs(feedsStore)

const handleFeedExpand = (id) => {
  expandFeed.value = id
}

const showExpandedFeed = (feed) => {
  if (expandFeed.value == null) {
    return false
  }
  return feed.id === expandFeed.value
}
</script>

<template>
  <FilterBox />
  <div class="feed-list-wrapper" v-for="feed in feeds" :key="feed.id">
    <Transition>
      <FeedItems
        :feed="feed"
        :filtered="props.filtered"
        :archived="props.archived"
        :expand="showExpandedFeed(feed)"
        @expand-feed="handleFeedExpand"
      >
      </FeedItems>
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
</style>
