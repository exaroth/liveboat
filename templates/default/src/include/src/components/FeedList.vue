<script setup>
import FeedItem from './FeedItem.vue'
import FilterBox from './FilterBox.vue'
import EmbedModal from './EmbedModal.vue'
import { useFeedsStore } from '@/stores/feeds'
import { useEmbedStore } from '../stores/embed'
import { storeToRefs } from 'pinia'

const props = defineProps({
  filtered: {
    type: Boolean,
    required: true,
  },
})

const embedStore = useEmbedStore()
const feedsStore = useFeedsStore()
const { feeds } = storeToRefs(feedsStore)
</script>

<template>
  <FilterBox />
  <div class="feed-list-wrapper" v-for="feed in feeds" :key="feed.id">
    <FeedItem :feed="feed" :filtered="props.filtered"> </FeedItem>
  </div>
  <EmbedModal v-if="embedStore.showModal" :embedCode="embedStore.modalEmbedCode" :fallbackUrl="embedStore.fallbackUrl"/>
</template>
