import { defineStore } from 'pinia'

const LOCAL_STORAGE_KEY = 'liveboat-default-filters'

const getDefaultFilters = () => ({
  itemCount: 20,
  daysBackCount: 1,
  filterByDays: false,
  firehose: false,
  searchTerm: '',
})

const getFilters = () => {
  let result = getDefaultFilters()
  const savedFilters = localStorage.getItem(LOCAL_STORAGE_KEY)
  if (savedFilters) {
    result = { ...result, ...JSON.parse(savedFilters) }
  }
  return result
}

export const useFiltersStore = defineStore('filters', {
  state: () => getFilters(),
  getters: {
    filters: (state) => state,
  },
  actions: {
    saveStore() {
      let d = {
        itemCount: this.itemCount,
        daysBackCount: this.daysBackCount,
        filterByDays: this.filterByDays,
        firehose: this.firehose,
      }
      localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify(d))
    },
  },
  computed: {
    itemCount: (state) => state.itemCount,
    daysBackCount: (state) => state.daysBackCount,
    filterByDays: (state) => state.filterByDays,
    searchTerm: (state) => state.searchTerm,
    firehose: (state) => state.firehose,
  },
})
