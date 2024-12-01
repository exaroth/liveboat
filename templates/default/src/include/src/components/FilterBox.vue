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
        searchTerm: val.toLowerCase().trim(),
      })
    }
  }, 600)
})

</script>

<template>
  <div class="filter-container">
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
    <span id="filter-search">
      <input placeholder="Search" :value="searchFeedsTerm" @input="(event) => (searchFeedsTerm = event.target.value)" />
    </span>
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

.filter-box button,
#filter-search input {
  width: 90px;
  background-color: #3c5e8b;
  border: none;
  line-height: 26px;
  outline: none;
  border-radius: 3px;
  font-weight: bold;
  color: #c7cfcc;
  outline: none;
}
.filter-box button,
#filter-search span,
#filter-search button{
  cursor: pointer;
}

.filter-box button.selected {
  background-color: transparent;
  outline: 1px solid #c7cfcc;
  border: none;
}

.filter-box button:hover {
  opacity: 0.9;
}

#filter-search {
  float: right;
}
#filter-search input {
  width: 164px;
  height: 29px;
  background-color: transparent;
  outline: 1px solid #c7cfcc;
  font-weight: normal;
  padding: 0px 0px 0px 6px;
}
@media (max-width: 640px) {
  .filter-container {
    margin: 0px;
    padding: 10px 0px 20px 0px;
  }
  #filter-search {
    float: none;
    position: relative;
  }
  #filter-search input {
    width: 100%;
    margin-top: 20px;
  }
  .filter-box button {
    width: 75px;
  }
}
</style>
