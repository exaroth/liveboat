<script setup>
import FeedList from '../components/FeedList.vue'
import IndexHeader from '@/components/IndexHeader.vue'
</script>

<template>
  <main>
    <IndexHeader/>
    <FeedList :filtered="true" />
  </main>
</template>
