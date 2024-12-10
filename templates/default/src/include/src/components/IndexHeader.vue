<script setup>
import IconGithub from './icons/IconGithub.vue'
import IconHeart from './icons/IconHeart.vue'
import IconLiveboat from './icons/IconLiveboat.vue'

const buildTime = new Date(window.buildTime * 1000)
const pageTitle = window.pageTitle
const sitePath = window.sitePath
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

    <IconLiveboat/>
        <a :href="sitePath">{{ pageTitle }}</a></h2>
      <h5>Page last updated on {{ buildTime.toUTCString() }}</h5>
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
  fill: var(--color-text-primary);

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
</style>
