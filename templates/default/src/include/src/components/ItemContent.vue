<script setup>
import { ref, onMounted } from 'vue'
const props = defineProps({
  content: {
    type: String,
    required: true,
  },
  contentLength: {
    type: Number,
    required: true,
  },
})

const hideArticleContentOverlay = () => {
	cRef.value.classList.remove("feed-item-contents-truncated")

}

const cRef = ref(null)
onMounted(() => {
  const box = cRef.value.getBoundingClientRect()
  if (box.height > 400) {
    cRef.value.classList.add("feed-item-contents-truncated")
  }
})
</script>

<template>
  <span :class="{ 'feed-item-contents': true }" ref="itemContents">
    <span class="feed-item-contents-ellipsis">---</span><br />
    <span ref="cRef" @click="hideArticleContentOverlay()"> <span v-html="props.content"></span><br /> </span>
  </span>
</template>

<style scoped>
.feed-item-contents {
  display: block;
}
.feed-item-contents:hover {
  background: transparent;
}
.feed-item-contents :deep(p) {
  margin-bottom: 12px;
}
.feed-item-contents-truncated {
  height: 300px;
  overflow: hidden;
  display: block;
  cursor: pointer;
  position: relative;
}
.feed-item-contents-truncated::after {
  content: "^";
	width: 100%;
	height: 100%;
  text-align: center;
  position: absolute;
  bottom: 0;
	transform: rotate(180deg);
  font-size: 24px;
	font-weight: bold;
	color: var(--color-custom);
	background-image: linear-gradient(to bottom, var(--color-background), transparent 20%);
}

.feed-item-contents-ellipsis {
  color: var(--color-custom);
}
</style>
