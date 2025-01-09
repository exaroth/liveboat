<script setup>
import { useEmbedStore } from '../stores/embed'
import { useAudioStore } from '../stores/audio'
import IconMusic from './icons/IconMusic.vue'
import IconMovie from './icons/IconMovie.vue'

const truncate = (v) => {
  const newline = v.indexOf('\n')
  return newline > 0 ? v.slice(0, newline) : v
}

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

const embedStore = useEmbedStore()
const audioStore = useAudioStore()

const props = defineProps({
  feedItem: {
    type: Object,
    required: true,
  },
})
</script>

<template>
  <span class="feed-item-link">
    <a v-if="embedStore.isEmbeddable(props.feedItem)" @click="showEmbedModal(props.feedItem)" target="_blank">
      {{ truncate(props.feedItem.title) }}<span class="feed-item-type"><IconMovie /></span
    ></a>
    <a
      v-else-if="audioStore.isAudioLink(props.feedItem)"
      @click="showAudioPlayer(props.feedItem)"
      target="_blank"
    >
      {{ truncate(props.feedItem.title) }}<span class="feed-item-type"><IconMusic /></span>
    </a>
    <a v-else :href="props.feedItem.url" target="_blank">{{ truncate(props.feedItem.title) }}</a>
  </span>
  <span class="feed-item-author" v-if="props.feedItem.author"> by {{ props.feedItem.author }}</span>
  <span class="feed-item-domain">({{ props.feedItem.domain }})</span>
</template>

<style scoped>

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
</style>
