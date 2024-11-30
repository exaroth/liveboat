import { defineStore } from 'pinia'

const getDefaultFilters = () => ({
  itemCount: 0,
  daysBackCount: 1,
  filterByDays: true,
  searchTerm: '',
})

const getFilters = () => {
  let result = getDefaultFilters()
  const savedFilters = localStorage.getItem('filters')
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
      }
      localStorage.setItem('filters', JSON.stringify(d))
    },
  },
  computed: {
    itemCount: (state) => state.itemCount,
    daysBackCount: (state) => state.daysBackCount,
    filterByDays: (state) => state.filterByDays,
    searchTerm: (state) => state.searchTerm,
  },
})
