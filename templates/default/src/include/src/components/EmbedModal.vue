<script setup>
import { useEmbedStore } from '../stores/embed'
import IconClose from './icons/IconClose.vue'
import IconMinimize from './icons/IconMinimize.vue'
import IconMaximize from './icons/IconMaximize.vue'

const embedStore = useEmbedStore()

window.addEventListener('keydown', (e) => {
  if (e.key == 'Escape') {
    embedStore.hideEmbedModal()
  }
})

const props = defineProps({
  embedCode: {
    type: String,
    required: true,
  },
  fallbackUrl: {
    type: String,
    required: true,
  },
})
</script>

<template>
  <div id="embed-modal-wrapper" :class="{ minimized: embedStore.minimized }">
    <div id="embed-modal-overlay" @click="embedStore.hideEmbedModal()"></div>
    <div id="modal-container" v-if="props.embedCode">
      <p class="modal-text-bkp">Loading...</p>
      <div v-html="props.embedCode"></div>
    </div>
    <div v-else>
      <a :href="props.fallbackUrl" class="modal-text-bkp">{{ props.fallbackUrl }}</a>
    </div>
    <div id="modal-buttons">
      <a id="modal-close" @click="embedStore.hideEmbedModal()"><IconClose /></a>
      <a id="modal-minimize" v-if="!embedStore.minimized" @click="embedStore.minimizeModal($event)"
        ><IconMinimize
      /></a>
      <a id="modal-maximize" v-if="embedStore.minimized" @click="embedStore.maximizeModal()"
        ><IconMaximize
      /></a>
    </div>
  </div>
</template>

<style scoped>
#embed-modal-wrapper.minimized #embed-modal-overlay {
  display: none;
}
#embed-modal-wrapper.minimized #modal-container {
  top: auto;
  bottom: 0;
  left: 0;
  transform: none;
}
#embed-modal-wrapper.minimized #modal-container :deep(iframe) {
  width: 360px;
  height: 240px;
}
#embed-modal-wrapper.minimized #modal-buttons {
  top: auto;
  bottom: 260px;
  left: 0;
  width: 360px;
  height: 20px;
  margin-right: 20px;
}
#embed-modal-wrapper.minimized #modal-buttons a {
  width: 28px;
  height: 28px;
  margin-left: 8px;
}
#embed-modal-overlay {
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.5);
  position: fixed;
  left: 0;
  top: 0;
  z-index: 998;
}
#modal-container {
  position: fixed;
  z-index: 999;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}
#modal-container :deep(iframe) {
  border: none;
  width: 70vw;
  outline: none;
  height: 90vh;
}
.modal-text-bkp {
  position: absolute;
  display: block;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  font-size: 1.3rem;
  z-index: -1;
}
#modal-buttons {
  width: 40px;
  position: fixed;
  top: 50px;
  right: 50px;
  z-index: 999;
}
#modal-buttons a {
  display: block;
  float: right;
  opacity: 0.6;
  width: 40px;
  height: 40px;
  cursor: pointer;
  margin-bottom: 10px;
}

@media (max-width: 1280px) {
  #modal-container :deep(iframe) {
    border: none;
    outline: none;
    width: 100vw;
    height: 100vh;
  }
  #modal-buttons a {
    top: 8px;
    right: 8px;
    opacity: 1;
    background-color: var(--color-background);
  }
}
@media (max-width: 1880px) {
  #modal-minimize,
  #modal-maximize {
    display: none !important;
  }
}
</style>
