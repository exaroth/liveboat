<script setup>
import { useRoute } from 'vue-router'
import { useFeedsStore } from '@/stores/feeds';
import FeedItem from '@/components/FeedItem.vue';

const route = useRoute()
const feedsStore = useFeedsStore();

const feed = feedsStore.getFeedById(route.params.feedId);
</script>

<template>
  <FeedItem :filtered="false" :feed="feed"></FeedItem>
</template>
