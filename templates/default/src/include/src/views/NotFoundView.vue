<script setup>
import IndexHeader from '@/components/IndexHeader.vue'
import IconNotFound from '@/components/icons/IconNotFound.vue'
</script>

<template>
  <main>
    <IndexHeader />
    <div id="not-found-container">
      <IconNotFound />
      <h1>Page not Found</h1>
    </div>
  </main>
</template>

<style scoped>
#not-found-container {
  width: 1024px;
  position: absolute;
  top: 50%;
  -ms-transform: translateY(-50%);
  transform: translateY(-50%);
}
#not-found-container h1 {
  width: 100%;
  text-align: center;
}
#not-found-container svg {
  width: 100px;
  height: 100px;
  display: block;
  margin: auto;
  stroke: #c7cfcc;
}
@media (max-width: 1024px) {
  #not-found-container {
    width: 100%
  }
}
</style>
