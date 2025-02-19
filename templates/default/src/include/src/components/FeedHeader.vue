<script setup>
import { shallowRef, onMounted, onUnmounted } from 'vue'
import { RouterLink } from 'vue-router'
import { useMinimizeStore } from '../stores/minimize'
import { useFeedItemsStore } from '../stores/feedItems'
import { useNavStore } from '../stores/nav'
import IconMinimize from './icons/IconMinimize.vue'
import IconMaximize from './icons/IconMaximize.vue'
import IconTop from './icons/IconTop.vue'

const fItemsStore = useFeedItemsStore()
const navStore = useNavStore()
const minimizeStore = useMinimizeStore()
const { getFeedItems } = fItemsStore

const feedHeaderRef = shallowRef(null)

const props = defineProps({
  feed: {
    type: Object,
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
  feedIndex: {
    type: Number,
    required: false,
  },
})

const emit = defineEmits(['expand-feed'])

const retrieveItemData = async () => {
  return await getFeedItems(props.feed.id, props.archived)
}

const dispatchExpandItems = async () => {
  const items = await retrieveItemData()
  return {
    feedId: props.feed.id,
    articleIds: items.map((i) => i.guid),
  }
}

const getNavData = () => {
  return {
    ref: feedHeaderRef,
    title: props.feed.displayTitle,
    index: props.feedIndex,
    minimized: minimizeStore.showFeedMinimized(props.feed.id),
  }
}

const expandFeed = () => {
  const y =
    feedHeaderRef.value.getBoundingClientRect().top + window.scrollY - window.innerHeight / 4 + 30
  window.scroll({
    top: y,
  })
  emit('expand-feed', dispatchExpandItems())
}

onMounted(() => {
  if (!props.firehose && !props.archived) {
    navStore.addFeed(getNavData())
  }
})

onUnmounted(() => {
  navStore.deleteFeed(props.feedIndex)
})
</script>

<template>
  <div class="feed-title" ref="feedHeaderRef">
    <router-link :to="{ name: 'feedView', params: { feedId: feed.id } }" v-if="!props.firehose"
      ><span v-html="feed.displayTitle || feed.title" />
      <span v-if="feed.isQuery" class="feed-query-indicator"></span>
      <span class="item-count">({{ feed.itemCount }})</span>
      <span id="feed-header-tags" v-for="(tag, index) in props.feed.tags" :key="index">
        <span class="feed-header-tag">#{{ tag }}</span>
      </span>
    </router-link>
    <a v-else href="#">{{ feed.displayTitle }}</a>
    <span class="feed-buttons">
      <button
        @click="minimizeStore.addMinimizedFeed(feed.id)"
        class="minimize-button"
        title="Minimize"
        v-if="!minimizeStore.showFeedMinimized(feed.id) && !props.archived && !props.firehose"
      >
        <IconMinimize />
      </button>
      <button
        @click="minimizeStore.removeMinimizedFeed(feed.id)"
        class="minimize-button"
        title="Maximize"
        v-if="minimizeStore.showFeedMinimized(feed.id) && !props.archived && !props.firehose"
      >
        <IconMaximize />
      </button>
      <button
        @click="expandFeed()"
        class="expand-button feed-expand-button"
        title="Expand"
        v-if="
          !props.expand &&
          !props.archived &&
          !props.firehose &&
          !minimizeStore.showFeedMinimized(feed.id)
        "
      >
        <IconTop />
      </button>
      <button
        @click="$emit('unexpand-feed')"
        class="expand-button feed-unexpand-button"
        title="Unexpand"
        v-if="props.expand && !props.archived && !props.firehose"
      >
        <IconTop />
      </button>
    </span>
  </div>
</template>

<style scoped>
.feed-title {
  padding: 0 0 0 4%;
  margin: 0px 0px 14px 0px;
  width: 100%;
  border-bottom: 2px solid var(--color-accent);
  position: relative;
}

.feed-title a {
  display: inline-block;
  background-color: var(--color-accent);
  padding: 2px 20px 0px 20px;
  border-radius: 3px 3px 0px 0px;
  max-width: 60%;
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
  position: relative;
  top: 7px;
}

.expand-button,
.minimize-button {
  display: inline-block;
  position: relative;
  cursor: pointer;
  border: none;
  background: transparent;
  opacity: 0.8;
  color: var(--color-text);
  font-size: 1.2rem;
}
.minimize-button svg {
  width: 20px;
  height: 20px;
}
.minimize-button {
  top: 1px;
  margin-left: 4px;
}

.feed-expand-button,
.feed-unexpand-button {
  color: var(--color-custom);
  opacity: 0.8;
  top: 2px;
}
.feed-expand-button svg {
  transform: rotate(180deg);
}
.feed-query-indicator {
  display: inline-block;
  height: 18px;
  width: 18px;
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
  color: var(--color-custom);
}
.item-count {
  opacity: 0.6;
  margin-left: 4px;
}

#feed-header-tags {
  margin-left: 6px;
  opacity: 0.7;
}
.feed-header-tag {
  color: var(--color-text);
  background-color: var(--color-background);
  font-size: 0.8em;
  padding: 2px 10px;
  font-weight: bold;
  line-height: 10px;
  border-radius: 25%;
  margin-left: 4px;
}
@media (max-width: 500px) {
  #feed-header-tags {
    display: none;
  }
}
</style>
