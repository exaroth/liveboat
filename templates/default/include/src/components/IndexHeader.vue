<script setup>
import IconGithub from './icons/IconGithub.vue'
import IconHeart from './icons/IconHeart.vue'

const buildTime = new Date(window.build_time * 1000)
const pageTitle = window.pageTitle
</script>

<template>
  <div class="header-crumbs">
    <span>
      <h5>Page generated with <IconHeart></IconHeart> using Liveboat</h5>
      <a href="git@github.com:exaroth/liveboat.git" target="_blank">
      <IconGithub id="github-link"></IconGithub>
      </a>
    </span>
  </div>
  <div class="header-container">
    <div class="header-title">
      <h2>{{ pageTitle }}</h2>
      <h5>Page last updated on {{ buildTime.toUTCString() }}</h5>
    </div>
  </div>
</template>

<style scoped>
.header-container {
  width: 100%;
  height: 80px;
  margin: 0px 0px 10px 0px;
  position: relative;
}
.header-title h2 {
  font-size: 1.8rem;
}
.header-title h5 {
  opacity: 0.6;
}
.header-crumbs {
  position: fixed;
  right: 20px;
  top: 10px;
  color:;
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
#github-link  {
  width: 28px;
  height: 28px;
  position: absolute;
  right: 0px;
  top: 36px;
  opacity: .9;
}

.header-crumbs svg:hover {
  opacity: 1;
}
</style>
