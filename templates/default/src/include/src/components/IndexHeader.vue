<script setup>
import { ref, onMounted } from 'vue'

import { useEmbedStore } from '../stores/embed'
import { useFeedItemsStore } from '../stores/feedItems'
import { useThemeStore } from '../stores/theme'

import IconGithub from './icons/IconGithub.vue'
import IconHeart from './icons/IconHeart.vue'
import IconTop from './icons/IconTop.vue'
import IconRss from './icons/IconRss.vue'
import IconOPML from './icons/IconOPML.vue'
import IconNav from './icons/IconNav.vue'
import IconLiveboat from './icons/IconLiveboat.vue'
import IconRefresh from './icons/IconRefresh.vue'

const buildTime = ref(new Date(window.buildTime * 1000))
const pageTitle = window.pageTitle
const sitePath = window.sitePath
const showScrollToTop = ref(false)
const showRefresh = ref(false)
const templateVersion = ref(window.templateVersion)

const embedStore = useEmbedStore()
const themeStore = useThemeStore()
const { resetFeedItems } = useFeedItemsStore()

const props = defineProps({
  feedList: {
    type: Boolean,
    required: true,
  },
})

const subHeaderText = ref(window.subHeaderText || null)
const repoUrl = ref(window.repoUrl || null)

const setScrollToTop = () => {
  let vh = Math.max(document.documentElement.clientHeight || 0, window.innerHeight || 0)
  let offset = window.pageYOffset
  showScrollToTop.value = offset > vh
}

const scrollToTop = () => {
  window.scrollTo(0, 0)
}
const getBuildTime = async () => {
  let basePath = 'build_time.txt'
  let pathPrefix = window.sitePath || '/'
  if (!pathPrefix.endsWith('/')) {
    pathPrefix = pathPrefix + '/'
  }
  let url = pathPrefix + basePath
  let response = await fetch(url)
  return parseInt(await response.text())
}

const emit = defineEmits(['toggle-nav', 'reload-feeds'])

setInterval(() => {
  setScrollToTop()
}, 300)

let bTimeInterval = setInterval(async () => {
  let bTime = await getBuildTime()
  const newBTime = new Date(bTime * 1000)
  if (newBTime > buildTime.value) {
    if (window.autoreload === 1) {
      resetFeedItems()
      buildTime.value = newBTime
    } else {
      showRefresh.value = true
      clearInterval(bTimeInterval)
    }
  }
}, 10 * 1000)

const toggleNav = () => {
  emit('toggle-nav')
}

const selectedTheme = ref(null)
const selectTheme = () => {
  themeStore.selectTheme(selectedTheme.value)
}

const refreshPage = () => {
  window.location.reload()
}

onMounted(() => {
  selectedTheme.value = themeStore.themeName
})
</script>

<template>
  <div class="header-crumbs">
    <span>
      <h5>
        Page generated with <IconHeart /> by
        <a href="https://github.com/exaroth/liveboat" target="_blank">Liveboat</a>
      </h5>
      <br />
      <h5>Updated on {{ buildTime.toUTCString() }}</h5>
      <br />
      <h5>Template ver. {{ templateVersion }}</h5>
    </span>
  </div>
  <div :class="{ 'header-container': true, 'header-container-archive': !props.feedList }">
    <div class="header-title">
      <h2>
        <IconLiveboat />
        <a :href="sitePath" v-html="pageTitle" />
      </h2>
      <h3 v-if="subHeaderText" v-html="subHeaderText" id="header-subtitle" />
      <div id="icons-aggro">
        <a
          id="icon-github"
          :href="repoUrl || 'https://github.com/exaroth/liveboat'"
          target="_blank"
        >
          <IconGithub />
        </a>
        <a id="icon-rss" href="rss.xml" target="_blank"><IconRss /></a>
        <a id="icon-opml" href="opml.xml" target="_blank"><IconOPML /></a>
      </div>
    </div>
  </div>
  <div id="theme-selector">
    <select v-model="selectedTheme" @change="selectTheme()">
      <option v-for="(themeO, theme) in themeStore.availableThemes" :value="theme" :key="theme">
        {{ themeO.txt }}
      </option>
    </select>
  </div>
  <div id="side-buttons-wrapper">
    <div v-if="!embedStore.showModal" id="side-buttons">
      <a id="side-button-top" title="Scroll to top" v-if="showScrollToTop" @click="scrollToTop()"
        ><IconTop
      /></a>
      <a
        id="side-button-refresh"
        title="New feeds available"
        v-if="showRefresh"
        @click="refreshPage()"
        ><IconRefresh
      /></a>
      <a id="side-button-nav" v-if="props.feedList" title="Show navigation" @click="toggleNav()"
        ><IconNav
      /></a>
    </div>
  </div>
</template>

<style scoped>
#theme-selector {
  position: relative;
  top: 4px;
}
#theme-selector select {
  width: 140px;
  -webkit-appearance: none !important;
  -moz-appearance:none !important;
  appearance: none !important;
  border: 1px solid rgb(from var(--color-text) r g b / 20%);
  padding: 4px 8px;;
  outline: none;
  background-color: var(--color-background);
  color: var(--color-text);
}

#theme-selector::after {
  position: absolute;
  content: "\2304";
  top: -6px;
  left: 230px;
  color: rgb(from var(--color-text) r g b / 30%);
  font-size: 1.4em;
  pointer-events: none;
}

#header-subtitle {
  display: box;
  margin: 10px 0;
  font-weight: 600;
}
.header-container {
  width: 100%;
  height: 80px;
  margin: 10px 0px 14px 0px;
  position: relative;
}
.header-container-archive {
  margin: 10px 0px 50px 0px;
}
.header-title h2 {
  font-size: 1.8rem;
}
.header-title a {
  background-color: transparent;
}
.header-title h5 {
  opacity: 0.6;
}
.header-title svg {
  position: relative;
  top: 4px;
  width: 28px;
  height: 28px;
  margin-right: 4px;
  fill: var(--color-text);
}
.header-crumbs {
  position: absolute;
  right: 20px;
  top: 10px;
  text-align: right;
  line-height: 16px;
}
.header-crumbs h5 {
  opacity: 0.5;
  display: inline-block;
}
.header-crumbs h5 svg {
  width: 10px;
  height: 10px;
  padding: 0;
  bottom: 0;
}

.header-crumbs svg:hover {
  opacity: 1;
}

#side-buttons-wrapper {
  position: fixed;
  right: 45px;
  z-index: 9;
  top: 40%;
  transform: translateY(-40%);
}

#side-buttons {
  position: sticky;
  width: 38px;
}

#side-buttons a {
  float: right;
  width: 38px;
  opacity: 0.6;
  cursor: pointer;
  margin-bottom: 20px;
}

#side-buttons a:hover {
  opacity: 1;
  background-color: none;
  background: none;
}
#side-buttons svg {
  width: 38px;
  height: 38px;
}

#icons-aggro {
  position: relative;
  left: 0;
  width: 200px;
  z-index: 1;
}

#icon-rss,
#icon-opml,
#icon-github {
  fill: var(--color-text);
  float: left;
  margin-right: 10px;
  display: block;
}

#icon-github svg {
  width: 22px;
  height: 22px;
  top: 7px;
}

#icon-rss svg {
  width: 20px;
  height: 20px;
  top: 8px;
  left: 2px;
}

@media (max-width: 640px) {
  .header-container {
    margin: 0;
  }
  .header-title h5 {
    display: none;
  }
  .header-title h2 {
    font-size: 1.4rem;
  }

  .header-crumbs {
    display: none;
  }
}

@media (max-width: 1640px) {
  #side-buttons-wrapper {
    transform: none;
    right: 20px;
    top: 20px;
  }
}

@media (min-width: 1901px) {
  #side-button-nav {
    display: none;
  }
}
@media (max-width: 810px) {
  #side-button-nav {
    display: none;
  }
  #icons-aggro {
    margin-top: 20px;
    width: 100%;
  }
  .header-container-archive {
    margin: 10px 0px 50px 0px;
  }
  #theme-selector {
    top: 20px;
  }
}
</style>
