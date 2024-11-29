import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export const useFiltersStore = defineStore('filters', {
  state: () => ({
      itemCount: 0,
      daysBackCount: 1,
      filterByDays: true,

  }),
  computed: {
    itemCount: (state) => state.itemCount,
    daysBackCount: (state) => state.daysBackCount,
    filterByDays: (state) => state.filterByDays,
  },
})
