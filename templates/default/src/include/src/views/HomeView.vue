<script setup>
import { ref } from 'vue'
import FeedList from '../components/FeedList.vue'
import IndexHeader from '@/components/IndexHeader.vue'

const navVisible = ref(false)

const handleNavToggle = () => {
  navVisible.value = !navVisible.value
}
</script>

<template>
  <main>
    <IndexHeader @toggle-nav="handleNavToggle" :feedList="true" />
    <FeedList :filtered="true" :archived="false" :showNav="navVisible" />
  </main>
</template>
