<script setup>
import { ref, watch } from 'vue'
import { useFiltersStore } from '../stores/filters'

const fStore = useFiltersStore()

const setTimeLimit = (limit) => {
  fStore.$patch({
    daysBackCount: limit,
    filterByDays: true,
    firehose: false,
  })
}
const setItemLimit = (limit) => {
  fStore.$patch({
    itemCount: limit,
    filterByDays: false,
    firehose: false,
  })
}
const setFirehose = () => {
  fStore.$patch({
    firehose: true,
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
      ><button :class="{ selected: filters.firehose === true }" @click="setFirehose()">
        Firehose
      </button></span
    >
    <span class="filter-box"
      ><button
        :class="{
          selected:
            filters.daysBackCount === 1 && filters.filterByDays === true && !filters.firehose,
        }"
        @click="setTimeLimit(1)"
      >
        Last day
      </button></span
    >
    <span class="filter-box"
      ><button
        :class="{
          selected: filters.itemCount === 50 && filters.filterByDays === false && !filters.firehose,
        }"
        @click="setItemLimit(50)"
      >
        Last 50
      </button></span
    >
    <span class="filter-box"
      ><button
        :class="{
          selected: filters.itemCount === 20 && filters.filterByDays === false && !filters.firehose,
        }"
        @click="setItemLimit(20)"
      >
        Last 20
      </button></span
    >
    <span id="filter-search">
      <input
        placeholder="Search"
        :value="searchFeedsTerm"
        @input="(event) => (searchFeedsTerm = event.target.value)"
      />
    </span>
  </div>
</template>

<style scoped>
.filter-container {
  width: 100%;
  padding: 20px 0px 20px 0px;
  margin: 0px 0px 24px 0px;
  position: relative;
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
  background-color: var(--color-accent);
  border: none;
  line-height: 26px;
  outline: none;
  border-radius: 3px;
  font-weight: bold;
  color: var(--color-text);
  outline: none;
}

.filter-box button,
#filter-search span,
#filter-search button {
  cursor: pointer;
}

.filter-box button.selected {
  background-color: transparent;
  outline: 1px solid var(--color-text);
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
  outline: 1px solid var(--color-text);
  font-weight: normal;
  padding: 0px 0px 0px 6px;
}

@media (max-width: 640px) {
  .filter-container {
    margin: 0 0 30px 0;
    position: relative;
    width: 100%;
    height: 110px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  #filter-search {
    float: none;
    position: relative;
  }
  #filter-search input {
    width: 100%;
    margin-top: 20px;
  }

  #filter-search {
    float: none;
    position: absolute;
    width: 100%;
    left: 50%;
    transform: translate(-50%);
    bottom: -10px;
  }

  .filter-box {
    display: inline-block;
  }

  .filter-box button {
    width: 70px;
  }
}
</style>
