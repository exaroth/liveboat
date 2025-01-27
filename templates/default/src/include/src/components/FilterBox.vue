<script setup>
import { ref, watch } from 'vue'
import { useFiltersStore } from '../stores/filters'
import IconCloseRound from './icons/IconCloseRound.vue'
import IconSearch from './icons/IconSearch.vue'

const fStore = useFiltersStore()
const inputSearch = ref(null)

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

const clearSearch = () => {
  searchFeedsTerm.value = ''
}

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
      <span id="filter-search-help">
        <span>In order to filter search results by tag use <code>t:</code> operator<br/>
          Valid examples:<br/>
          <code>t:tag1 <\search-query\></code><br/>
          <code>t:tag1,tag2</code><br/>
          <code>t:tag1 t:tag2</code><br/>
          <code>t:tag1,"Tag with space"</code>
        </span>
      </span>
      <button
        id="filter-search-clear"
        alt="Clear"
        v-if="inputSearch && inputSearch.value.length > 0"
        @click="clearSearch()"
      >
        <IconCloseRound />
      </button>
      <span id="filter-search-icon"> <IconSearch /></span>
      <input
        placeholder="Search"
        :value="searchFeedsTerm"
        @input="(event) => (searchFeedsTerm = event.target.value)"
        ref="inputSearch"
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
  position: relative;
  float: right;
}

#filter-search input {
  width: 200px;
  height: 29px;
  background-color: transparent;
  outline: 1px solid rgb(from var(--color-text) r g b / 40%);
  font-weight: normal;
  padding: 0px 30px 0px 26px;
}

#filter-search input:focus {
  outline: 1px solid var(--color-custom);
}

#filter-search-clear,
#filter-search-icon {
  position: absolute;
  color: rgb(from var(--color-text) r g b / 70%);
  background-color: transparent;
  border: none;
  width: 18px;
  height: 18px;
  align-content: center;
}

#filter-search-clear {
  right: 10px;
  top: 4px;
}

#filter-search-icon {
  left: 8px;
  top: 2px
}

#filter-search-clear:hover {
  color: rgb(from var(--color-text) r g b / 80%);
}

#filter-search-help {
  position: absolute;
  left: -40px;
  top: 2px;
}
#filter-search-help::after {
  content: "?";
  display: inline-block;
  background-color: var(--color-accent);
  width: 24px;
  height: 24px;
  border-radius: 50%;
  text-align: center;
  font-weight: bold;

}
#filter-search-help:hover span {
  display: block;
  position: absolute;
  width: 280px;
  background-color: var(--color-background);
  border-radius: 2px;
  border: 1px solid var(--color-accent);
  z-index: 999;
  padding: 6px 12px;
  font-size: .9em;
}
#filter-search-help span {
  display: none;
}

@media (max-width: 640px) {
  #filter-search-clear {
    top: 24px;
  }

  #filter-search-icon {
    top: 22px;
  }
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
  #filter-search-help {
    display: none;
  }

  .filter-box {
    display: inline-block;
  }

  .filter-box button {
    width: 70px;
  }
}
</style>
