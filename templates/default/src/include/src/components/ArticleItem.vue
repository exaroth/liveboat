<script setup>
import { useEmbedStore } from '../stores/embed'
import { useAudioStore } from '../stores/audio'
import IconMusic from './icons/IconMusic.vue'
import IconMovie from './icons/IconMovie.vue'
import IconExpand from './icons/IconExpand.vue'
import IconUnexpand from './icons/IconUnexpand.vue'

const props = defineProps({
  feedItem: {
    type: Object,
    required: true,
  },
  expand: {
    type: Boolean,
    required: true,
  },
})

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
const showAudioPlayer = () => {
  if (embedStore.showModal) {
    embedStore.hideEmbedModal()
  }
  audioStore.setAudioData(props.feedItem)
  audioStore.showAudioPlayer()
}

const embedStore = useEmbedStore()
const audioStore = useAudioStore()
</script>

<template>
  <span class="feed-item-link">
    <a
      v-if="embedStore.isEmbeddable(props.feedItem)"
      @click="showEmbedModal(props.feedItem)"
      target="_blank"
    >
      {{ truncate(props.feedItem.title) }}<span class="feed-item-type"><IconMovie /></span
    ></a>
    <a
      v-else-if="audioStore.isAudioLink(props.feedItem)"
      @click="showAudioPlayer()"
      target="_blank"
    >
      {{ truncate(props.feedItem.title) }}<span class="feed-item-type"><IconMusic /></span>
    </a>
    <a v-else :href="props.feedItem.url" target="_blank">{{ truncate(props.feedItem.title) }}</a>
    <button
      @click="$emit('expand-article')"
      class="expand-button article-expand"
      title="Expand"
      v-if="!props.expand"
    >
      <IconExpand />
    </button>
    <button
      @click="$emit('unexpand-article')"
      class="expand-button article-expand"
      title="Unexpand"
      v-if="props.expand"
    >
      <IconUnexpand />
    </button>
  </span>
  <span class="feed-item-author" v-if="props.feedItem.author"> by {{ props.feedItem.author }}</span>
  <span class="feed-item-domain">({{ props.feedItem.domain }})</span>
  <div :class="{ 'feed-item-details': true, expanded: expand }" v-if="props.expand">
    <span class="feed-item-date"
      ><span class="feed-item-details-desc">Date: </span
      >{{ props.feedItem.date.toUTCString() }}</span
    ><br />
    <span class="feed-item-url"
      ><span class="feed-item-details-desc">URL: </span>{{ props.feedItem.url }}</span
    ><br />
    <span class="feed-item-contents" v-if="props.feedItem.content"
      ><span class="feed-item-details-desc">Content: </span>
      <span v-html="props.feedItem.content"></span
    ></span>
  </div>
</template>

<style scoped>
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
</style>
