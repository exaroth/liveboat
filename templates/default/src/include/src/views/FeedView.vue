<script setup>
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import { useFeedsStore } from '@/stores/feeds'
import { useEmbedStore } from '../stores/embed'
import { useAudioStore } from '../stores/audio'
import AudioPlayer from '@/components/AudioPlayer.vue'
import EmbedModal from '@/components/EmbedModal.vue'
import FeedItems from '@/components/FeedItems.vue'
import IndexHeader from '@/components/IndexHeader.vue'

const route = useRoute()
const feedsStore = useFeedsStore()
const audioStore = useAudioStore()
const embedStore = useEmbedStore()
const expandedArticles = ref([])

const feed = feedsStore.getFeedById(route.params.feedId)
const handleArticleExpand = (articleId) => {
  expandedArticles.value.push(articleId)
}

const handleArticleUnexpand = (articleId) => {
  expandedArticles.value = expandedArticles.value.filter((i) => {
    return i !== articleId
  })
}
</script>

<template>
  <IndexHeader></IndexHeader>
  <FeedItems
    :filtered="false"
    :archived="true"
    :feed="feed"
    :expand="false"
    :expandedArticles="expandedArticles"
    @expand-article="handleArticleExpand"
    @unexpand-article="handleArticleUnexpand"
  ></FeedItems>
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
