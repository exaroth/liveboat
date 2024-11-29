<script setup>
import { useFiltersStore } from '../stores/filters'

const fStore = useFiltersStore()

const setTimeLimit = (limit) => {
  console.log('time limit = ', limit)
  fStore.$patch({
    daysBackCount: limit,
    filterByDays: true,
  })
}
const setItemLimit = (limit) => {
  console.log('set item limit = ', limit)
  fStore.$patch({
    itemCount: limit,
    filterByDays: false,
  })
}
</script>

<template>
  <div class="filter-container">
    <span class="filter-box"><button @click="setTimeLimit(1)">Last day</button></span>
    <span class="filter-box"><button @click="setTimeLimit(7)">Last week</button></span>
    <span class="filter-box"><button @click="setItemLimit(20)">Last 20</button></span>
    <span class="filter-box"><button @click="setItemLimit(50)">Last 50</button></span>
  </div>
</template>
