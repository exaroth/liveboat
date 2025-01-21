<script>
import { mapStores } from 'pinia'
import { useNavStore } from '../stores/nav'

export default {
  name: 'FeedNavigator',
  created() {
    window.addEventListener('scroll', () => {
      this.implicitFeedSelection = null
    })
  },
  mounted() {
    setInterval(() => {
      this.updateHighlight()
    }, 300)
  },
  data() {
    return {
      implicitFeedSelection: null,
    }
  },
  computed: {
    ...mapStores(useNavStore),
  },
  methods: {
    updateHighlight() {
      if (this.navStore.feeds.length === 0) {
        return
      }
      for (const feed of this.navStore.feeds) {
        if (feed.minimized) {
          continue
        }
        if (feed.ref == null) {
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
          if (!nextF || !nextF.ref) {
            continue
          }
          const nextY = nextF.ref.getBoundingClientRect().top + scrollTop - clientTop
          if (nextY > yTarget && feed.index !== this.navStore.activeFeed) {
            this.navStore.setActiveFeed(feed.index)
            this.recomputeNavScroll(feed.index)
          }
        }
      }
    },
    recomputeNavScroll(fIndex) {
      const navC = this.$refs.navContainer
      let navE = this.$refs['navigatorLink-' + fIndex]
      if (!navE || navE.length === 0) {
        return
      }
      navE = navE[0]
      if (navE.offsetTop > navC.scrollTop + navC.clientHeight) {
        navC.scroll({
          top: navE.offsetTop - navC.clientHeight + 40,
        })
        return
      }
      if (navE.offsetTop < navC.scrollTop) {
        navC.scroll({
          top: Math.max(navE.offsetTop - 40, 0),
        })
        return
      }
    },
    getActiveFeed(feedIndex) {
      if (this.implicitFeedSelection != null) {
        return this.implicitFeedSelection === feedIndex
      }
      return this.navStore.activeFeed === feedIndex
    },
    goToFeed(ref, index) {
      const y = ref.getBoundingClientRect().top + window.scrollY - window.innerHeight / 3
      this.implicitFeedSelection = index
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
    <div id="nav-container" ref="navContainer">
      <ul id="navigator-links" v-for="f in navStore.feeds" :key="f.index">
        <li
          :class="{ 'navigator-link': true, 'navigator-link-active': getActiveFeed(f.index) }"
          v-if="!f.minimized"
          :ref="'navigatorLink-' + f.index"
        >
          <a @click="goToFeed(f.ref, f.index)" v-html="f.title" />
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
#feed-navigator {
  position: fixed;
  top: 50%;
  transform: translateY(-50%);
  right: 140px;
}

#nav-container {
  height: 60vh;
  width: 320px;
  position: relative;
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
