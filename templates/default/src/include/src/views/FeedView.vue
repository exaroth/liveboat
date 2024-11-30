<script setup>
import { useRoute } from 'vue-router'
import { useFeedsStore } from '@/stores/feeds';
import FeedItem from '@/components/FeedItem.vue';
import IndexHeader from '@/components/IndexHeader.vue';

const route = useRoute()
const feedsStore = useFeedsStore();

const feed = feedsStore.getFeedById(route.params.feedId);

</script>

<template>
  <IndexHeader></IndexHeader>
  <FeedItem :filtered="false" :feed="feed"></FeedItem>
</template>
