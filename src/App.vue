<script setup lang="ts">
import {nextTick, provide, ref} from "vue";

const isRouteAlive = ref<boolean>(true);
const reload = () => {
  isRouteAlive.value = false
  nextTick(() => {
    isRouteAlive.value = true
  })
}

provide("reload", reload)

</script>

<template>
  <div class="app-container">
    <router-view v-if="isRouteAlive"/>
  </div>
</template>

<style scoped>
.app-container {
  position: relative;
  min-height: 100vh;
}

</style>
<style>
</style>