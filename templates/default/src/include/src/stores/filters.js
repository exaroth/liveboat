import { defineStore } from 'pinia'

const getDefaultFilters = () => ({
    itemCount: 0,
    daysBackCount: 1,
    filterByDays: true,
})
const getSettings = () => {
  const settings = localStorage.getItem("filters")
  return settings ? JSON.parse(settings) : getDefaultFilters()
}

export const useFiltersStore = defineStore('filters', {
  state: () => getSettings(),
  getters: {
    filters: (state) => state
  },
  actions: {
    saveStore() {
      let d = {
        itemCount: this.itemCount,
        daysBackCount: this.daysBackCount,
        filterByDays: this.filterByDays,
      }
      localStorage.setItem("filters", JSON.stringify(d))
    }
  },
  computed: {
    itemCount: (state) => state.itemCount,
    daysBackCount: (state) => state.daysBackCount,
    filterByDays: (state) => state.filterByDays,
  },
})
