<script setup lang="ts">
import {nextTick, onMounted, provide, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {call} from "./utils/commands.ts";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", {name: name.value});
}

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