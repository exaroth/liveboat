<script setup>
import { ref } from 'vue'

import { useEmbedStore } from '../stores/embed'

import IconGithub from './icons/IconGithub.vue'
import IconHeart from './icons/IconHeart.vue'
import IconTop from './icons/IconTop.vue'
import IconRss from './icons/IconRss.vue'
import IconOPML from './icons/IconOPML.vue'
import IconLiveboat from './icons/IconLiveboat.vue'
import IconRefresh from './icons/IconRefresh.vue'

const buildTime = new Date(window.buildTime * 1000)
const pageTitle = window.pageTitle
const sitePath = window.sitePath
const showScrollToTop = ref(false)
const showRefresh = ref(false)

const embedStore = useEmbedStore()

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

setInterval(() => {
  setScrollToTop()
}, 300)

let bTimeInterval = setInterval(async () => {
  let bTime = await getBuildTime()
  if (new Date(bTime * 1000) > buildTime) {
    showRefresh.value = true
    clearInterval(bTimeInterval)
  }
}, 10 * 1000)

const refreshPage = () => {
  window.location.reload()
}
</script>

<template>
  <div class="header-crumbs">
    <span>
      <h5>Page generated with <IconHeart /> by Liveboat</h5>
      <a href="https://github.com/exaroth/liveboat" target="_blank">
        <IconGithub id="github-link"></IconGithub>
      </a>
    </span>
  </div>
  <div class="header-container">
    <div class="header-title">
      <h2>
        <IconLiveboat />
        <a :href="sitePath" v-html="pageTitle"/>
      </h2>
      <h5>Page last updated on {{ buildTime.toUTCString() }}</h5>
      <div id="icons-aggro">
        <a id="icon-rss" href="rss.xml" target="_blank"><IconRss /></a>
        <a id="icon-opml" href="opml.xml" target="_blank"><IconOPML /></a>
      </div>
    </div>
  </div>
  <div id="side-buttons-wrapper">
    <div v-if="!embedStore.showModal" id="side-buttons">
      <a title="Scroll to top" v-if="showScrollToTop" @click="scrollToTop()"><IconTop /></a>
      <a title="New feeds available" v-if="showRefresh" @click="refreshPage()"><IconRefresh /></a>
    </div>
  </div>
</template>

<style scoped>
.header-container {
  width: 100%;
  height: 80px;
  margin: 10px 0px 10px 0px;
  position: relative;
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
#github-link {
  width: 28px;
  height: 28px;
  position: absolute;
  right: 0px;
  top: 36px;
  opacity: 0.9;
}

.header-crumbs svg:hover {
  opacity: 1;
}

#side-buttons-wrapper {
  position: fixed;
  right: 60px;
  z-index: 997;
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
  position: absolute;
  right: 0;
  bottom: -12px;;
  width: 100px;
}

#icon-rss svg,
#icon-opml svg {
  display: inline-block;
  float: right;
  margin-left: 10px;
}
#icon-rss svg {
  width: 20px;
  height: 20px;
  top: 8px;
}

@media (max-width: 640px) {
  .header-crumbs {
    top: 2px;
    right: 12px;
  }
  .header-crumbs h5 {
    display: none;
  }
  #github-link {
    top: 12px;
  }
}


@media (max-width: 1640px) {
  #side-buttons-wrapper {
    right: 20px;
    top: 40px;
  }
}
</style>
