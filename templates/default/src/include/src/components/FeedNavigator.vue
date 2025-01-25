<script>
import { mapStores } from 'pinia'
import { useNavStore } from '../stores/nav'

export default {
  name: 'FeedNavigator',
  props: {
    show: {
      type: Boolean,
      required: true,
    },
  },
  created() {
    window.addEventListener('scroll', () => {
      this.implicitFeedSelection = null
      this.navAutoScrollDisabled = false
    })
  },
  mounted() {
    setInterval(() => {
      this.updateHighlight()
      this.updateScrollIcons()
    }, 300)
    let navCInterval = setInterval(() => {
      const navC = this.$refs.navContainer
      if (navC != null) {
        navC.addEventListener('scroll', () => {
          this.navAutoScrollDisabled = true
        })
        clearInterval(navCInterval)
      }
    }, 100)
  },
  data() {
    return {
      navAutoScrollDisabled: false,
      implicitFeedSelection: null,
      navListScrolledDown: null,
      navListScrolledUp: null,
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
      const body = document.body
      const docEl = document.documentElement
      const scrollTop = window.pageYOffset || docEl.scrollTop || body.scrollTop
      const clientTop = docEl.clientTop || body.clientTop || 0

      if (scrollTop === 0) {
        this.setActiveFeed(this.navStore.feeds[0].index, true)
        return
      }

      if (scrollTop + window.innerHeight === body.offsetHeight) {
        this.setActiveFeed(this.navStore.feeds[this.navStore.feeds.length - 1].index, true)
        return
      }

      for (const feed of this.navStore.feeds) {
        if (feed.ref == null) {
          continue
        }
        const y = feed.ref.getBoundingClientRect().top + scrollTop - clientTop
        if (y > scrollTop + window.innerHeight) {
          continue
        }
        const yTarget = scrollTop + window.innerHeight / 2

        const nextF = this.navStore.getNextFeed(feed.index)
        if (!nextF || !nextF.ref) {
          this.setActiveFeed(feed.index, false)
          return
        }
        const nextY = nextF.ref.getBoundingClientRect().top + scrollTop - clientTop
        if (y <= yTarget && nextY > yTarget) {
          this.setActiveFeed(feed.index, false)
          return
        }
      }
    },
    updateScrollIcons() {
      const navC = this.$refs.navContainer
      if (navC == null) {
        return
      }
      this.navListScrolledDown = navC.scrollTop > 0
      this.navListScrolledUp = Math.abs(navC.scrollHeight - navC.scrollTop - navC.clientHeight) > 1
    },
    setActiveFeed(feedIndex, implicit) {
      this.navStore.setActiveFeed(feedIndex)
      this.recomputeNavScroll(feedIndex)
      if (implicit) {
        this.implicitFeedSelection = feedIndex
      }
    },
    recomputeNavScroll(fIndex) {
      if (this.implicitFeedSelection != null) {
        return
      }
      if (this.navAutoScrollDisabled) {
        return
      }
      const navC = this.$refs.navContainer
      let navE = this.$refs['navigatorLink-' + fIndex]
      if (!navE || navE.length === 0) {
        return
      }
      navE = navE[0]
      const offset = navC.clientHeight * 0.25
      if (navE.offsetTop > navC.scrollTop + navC.clientHeight) {
        navC.scroll({
          top: navE.offsetTop - navC.clientHeight + offset,
        })
        return
      }
      if (navE.offsetTop < navC.scrollTop) {
        navC.scroll({
          top: Math.max(navE.offsetTop - offset, 0),
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
      const y = ref.getBoundingClientRect().top + window.scrollY - window.innerHeight / 2 + 30
      window.scroll({
        top: y,
      })
      this.setActiveFeed(index, true)
    },
  },
}
</script>

<template>
  <div id="feed-navigator-overlay" :class="{ 'navigator-visible': show }">
    <div id="feed-navigator" v-if="navStore.feeds.length > 0">
      <h3 id="nav-header-title">Feed List</h3>
      <div
        id="nav-container"
        ref="navContainer"
        :class="{
          'scrolled-down': this.navListScrolledDown,
          'scrolled-up': this.navListScrolledUp,
        }"
      >
        <span class="nav-scroll-indicator" id="nav-scroll-top"><IconTop /></span>
        <ul id="navigator-links" v-for="f in navStore.feeds" :key="f.index">
          <li
            :class="{ 'navigator-link': true, 'navigator-link-active': getActiveFeed(f.index) }"
            :ref="'navigatorLink-' + f.index"
          >
            <a @click="goToFeed(f.ref, f.index)" v-html="f.title" />
          </li>
        </ul>
      </div>
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

#nav-container.scrolled-down::before,
#nav-container.scrolled-up::after {
  position: fixed;
  color: var(--color-highlight);
  left: -14px;
  font-size: 2em;
}

#nav-container.scrolled-down::before {
  content: '\2303';
  top: 14px;
}

#nav-container.scrolled-up::after {
  content: '\2304';
  bottom: 0;
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
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
  padding-left: 10px;
  position: relative;
  border-left: 1px solid rgb(from var(--color-accent) r g b / 60%);
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
    left: 40px;
    right: auto;
  }

  #feed-navigator-overlay.navigator-visible {
    display: block;
  }

  #feed-navigator-overlay {
    overscroll-behavior: contain;
    display: none;
    right: 0;
    top: 0;
    position: fixed;
    background-color: rgb(from var(--color-background) r g b / 95%);
    height: 100vh;
    width: 600px;
    z-index: 8;
    border-left: 1px solid var(--color-accent);
  }

  #nav-container {
    height: 80vh !important;
    width: 100%;
    max-width: 80%;
  }
  .navigator-link {
    font-size: 1.2em;
  }
}
@media (max-width: 810px) {
  #feed-navigator-overlay {
    width: 100vw !important;
    border: none;
  }
}
</style>
