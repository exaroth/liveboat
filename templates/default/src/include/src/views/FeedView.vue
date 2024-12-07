<script setup>
import { useRoute } from 'vue-router'
import { useFeedsStore } from '@/stores/feeds'
import { useEmbedStore } from '../stores/embed'
import EmbedModal from '@/components/EmbedModal.vue'
import FeedItem from '@/components/FeedItem.vue'
import IndexHeader from '@/components/IndexHeader.vue'

const route = useRoute()
const feedsStore = useFeedsStore()
const embedStore = useEmbedStore()

const feed = feedsStore.getFeedById(route.params.feedId)
</script>

<template>
  <IndexHeader></IndexHeader>
  <FeedItem :filtered="false" :feed="feed"></FeedItem>
  <EmbedModal v-if="embedStore.showModal" :embedCode="embedStore.modalEmbedCode" :fallbackUrl="embedStore.fallbackUrl"/>
</template>
