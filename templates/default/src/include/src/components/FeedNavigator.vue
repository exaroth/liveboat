<script>
import { mapStores } from 'pinia'
import { useNavStore } from '../stores/nav'

export default {
  name: 'FeedNavigator',
  mounted() {
    setInterval(() => {
      for (const feed of this.navStore.feeds) {
        if (feed.minimized) {
          continue
        }
        const body = document.body
        const docEl = document.documentElement
        const scrollTop = window.pageYOffset || docEl.scrollTop || body.scrollTop
        const clientTop = docEl.clientTop || body.clientTop || 0
        const y = feed.ref.getBoundingClientRect().top + scrollTop - clientTop
        if (y > scrollTop + window.innerHeight) {
          return
        }
        const yTarget = scrollTop + window.innerHeight / 2
        if (y <= yTarget) {
          const nextF = this.navStore.feeds[feed.index + 1]
          if (!nextF) {
            return
          }
          const nextY = nextF.ref.getBoundingClientRect().top + scrollTop - clientTop
          if (nextY > yTarget && feed.index !== this.navStore.activeFeed) {
            this.navStore.setActiveFeed(feed.index)
          }
        }
      }
    }, 300)
  },
  computed: {
    ...mapStores(useNavStore),
  },
  methods: {
    getActiveFeed(feedIndex) {
      return this.navStore.activeFeed == feedIndex
    },
    goToFeed(ref) {
      const y = ref.getBoundingClientRect().top + window.scrollY - window.innerHeight / 2
      window.scroll({
        top: y,
      })
    },
  },
}
</script>

<template>
  <div id="feed-navigator">
    <h3 id="nav-header-title">Feed List</h3>
    <ul id="navigator-links" v-for="f in navStore.feeds" :key="f.index">
      <li
        :class="{ 'navigator-link': true, 'navigator-link-active': getActiveFeed(f.index) }"
        v-if="!f.minimized"
      >
        <a @click="goToFeed(f.ref)" v-html="f.title" />
      </li>
    </ul>
  </div>
</template>

<style scoped>
#feed-navigator {
  position: fixed;
  top: 50%;
  transform: translateY(-50%);
  width: 320px;
  height: 60vh;
  right: 140px;
  overflow: scroll;
  -ms-overflow-style: none;
  scrollbar-width: none;
}
#nav-header-title {
  margin-bottom: 6px;
  padding-left: 10px;
}

#feed-navigator::-webkit-scrollbar {
  display: none;
}

.navigator-link {
  opacity: 0.7;
  cursor: pointer;
  width: 100%;
  text-overflow: ellipsis;
  padding-left: 10px;
  position: relative;
  border-left: 0.6px solid rgb(from var(--color-accent) r g b / 60%);
}

.navigator-link.navigator-link-active {
  opacity: 1;
}

.navigator-link.navigator-link-active::before {
  display: block;
  content: '';
  left: -5px;
  width: 4px;
  height: 100%;
  background-color: var(--color-highlight);
  position: absolute;
}

@media (max-width: 1900px) {
  #feed-navigator {
    position: relative;
    left: 0;
    top: 0;
  }
}
</style>
