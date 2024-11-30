<script setup>
import { ref, watch } from 'vue'
import { useFiltersStore } from '../stores/filters'

const fStore = useFiltersStore()

const setTimeLimit = (limit) => {
  fStore.$patch({
    daysBackCount: limit,
    filterByDays: true,
  })
}
const setItemLimit = (limit) => {
  fStore.$patch({
    itemCount: limit,
    filterByDays: false,
  })
}

const filters = ref(fStore.filters)

fStore.$subscribe((state) => {
  filters.value = state.payload
  fStore.saveStore()
})

var searchDelayTimeout

const searchFeedsTerm = ref('')
watch(searchFeedsTerm, (val) => {
  clearTimeout(searchDelayTimeout)
  searchDelayTimeout = setTimeout(() => {
    if (searchFeedsTerm.value != null || searchFeedsTerm.value !== '') {
      fStore.$patch({
        searchTerm: val.toLowerCase(),
      })
    }
  }, 600)
})
</script>

<template>
  <div class="filter-container">
    <span id="filter-search">
      <input :value="searchFeedsTerm" @input="(event) => (searchFeedsTerm = event.target.value)" />
    </span>
    <span class="filter-box"
      ><button
        :class="{ selected: filters.daysBackCount === 7 && filters.filterByDays === true }"
        @click="setTimeLimit(7)"
      >
        Last week
      </button></span
    >
    <span class="filter-box"
      ><button
        :class="{ selected: filters.daysBackCount === 1 && filters.filterByDays === true }"
        @click="setTimeLimit(1)"
      >
        Last day
      </button></span
    >
    <span class="filter-box"
      ><button
        :class="{ selected: filters.itemCount === 50 && filters.filterByDays === false }"
        @click="setItemLimit(50)"
      >
        Last 50
      </button></span
    >
    <span class="filter-box"
      ><button
        :class="{ selected: filters.itemCount === 20 && filters.filterByDays === false }"
        @click="setItemLimit(20)"
      >
        Last 20
      </button></span
    >
  </div>
</template>

<style scoped>
.filter-container {
  width: 100%;
  padding: 20px 0px 20px 0px;
  margin: 0px 0px 24px 0px;
}

.filter-box {
  float: right;
  margin: 0px 0px 0px 12px;
  cursor: pointer;
  border-radius: 3px;
}

.filter-box button {
  width: 90px;
  cursor: pointer;
  background-color: #3c5e8b;
  border: none;
  line-height: 26px;
  outline: none;
  border-radius: 3px;
  font-weight: bold;
  color: #c7cfcc;
  outline: none;
}

.filter-box button.selected {
  background-color: transparent;
  outline: 1px solid #c7cfcc;
  border: none;
}

.filter-box button:hover {
  opacity: 0.9;
}
</style>
