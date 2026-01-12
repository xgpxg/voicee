<script setup lang="ts">
import {nextTick, onMounted, provide, ref} from "vue";
import {call} from "./utils/commands.ts";

const isRouteAlive = ref(true);
const reload = () => {
  isRouteAlive.value = false
  nextTick(() => {
    isRouteAlive.value = true
  })
}

provide("reload", reload)

const init = async () => {
  await call('gen_unique_id', {})
}

onMounted(async () => {
  await init()
})
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