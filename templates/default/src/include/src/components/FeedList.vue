<script setup>
  import { ref } from 'vue'
import FeedItem from './FeedItem.vue'
import AudioPlayer from './AudioPlayer.vue'
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
  archived: {
    type: Boolean,
    required: true,
  },
})

const embedStore = useEmbedStore()
const feedsStore = useFeedsStore()
const depro = ref("https://www.kimlarocca.com/fur-elise.mp3")
const { feeds } = storeToRefs(feedsStore)
</script>

<template>
  <AudioPlayer
      :title="'title'"
      :url="'https://www.deprofundis.com'"
      :feedTitle="'feed'"
      :feedLink="'kek'"
      :file="depro"
  />
  <FilterBox />
  <div class="feed-list-wrapper" v-for="feed in feeds" :key="feed.id">
    <Transition>
      <FeedItem :feed="feed" :filtered="props.filtered" :archived="props.archived"> </FeedItem>
    </Transition>
  </div>
  <EmbedModal
    v-if="embedStore.showModal"
    :embedCode="embedStore.modalEmbedCode"
    :fallbackUrl="embedStore.fallbackUrl"
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
