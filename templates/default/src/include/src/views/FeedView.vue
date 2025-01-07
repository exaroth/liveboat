<script setup>
import { useRoute } from 'vue-router'
import { useFeedsStore } from '@/stores/feeds'
import { useEmbedStore } from '../stores/embed'
import { useAudioStore } from '../stores/audio'
import AudioPlayer from '@/components/AudioPlayer.vue'
import EmbedModal from '@/components/EmbedModal.vue'
import FeedItem from '@/components/FeedItem.vue'
import IndexHeader from '@/components/IndexHeader.vue'

const route = useRoute()
const feedsStore = useFeedsStore()
const audioStore = useAudioStore()
const embedStore = useEmbedStore()

const feed = feedsStore.getFeedById(route.params.feedId)
</script>

<template>
  <IndexHeader></IndexHeader>
  <FeedItem :filtered="false" :archived="true" :feed="feed"></FeedItem>
  <EmbedModal v-if="embedStore.showModal" :embedCode="embedStore.modalEmbedCode" :fallbackUrl="embedStore.fallbackUrl"/>
  <AudioPlayer
    v-if="audioStore.audioPlayerVisible"
    :title="audioStore.linkName"
    :url="audioStore.linkUrl"
    :feedTitle="audioStore.feedName"
    :feedLink="audioStore.feedUrl"
    :file="audioStore.audioUrl"
  />
</template>
