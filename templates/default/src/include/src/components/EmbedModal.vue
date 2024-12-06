<script setup>
import { useEmbedStore } from '../stores/embed'
import IconClose from './icons/IconClose.vue'

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
  <div id="embed-modal-overlay" @click="embedStore.hideEmbedModal()">
    <div v-if="props.embedCode" id="modal-container" v-html="props.embedCode"></div>
    <div v-else>
      <a :href="props.fallbackUrl"></a>
    </div>
    <a id="modal-close" @click="embedStore.hideEmbedModal()"><IconClose /></a>
  </div>
</template>

<style scoped>
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
  background: transparent;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}
#modal-container :deep(iframe) {
  border: none;
  outline: none;
  width: 70vw;
  height: 90vh;
}
#modal-close {
  display: block;
  position: absolute;
  width: 60px;
  height: 60px;
  top: 40px;
  right: 40px;
  cursor: pointer;
  z-index: 999;
}

@media (max-width: 1280px) {
  #modal-container :deep(iframe) {
    border: none;
    outline: none;
    width: 100vw;
    height: 100vh;
  }
  #modal-close {
    width: 40px;
    height: 40px;
    top: 36px;
    right: 20px;
  }
}
</style>
