<script setup lang="ts">
import HelloWorld from "./components/HelloWorld.vue";

console.log("[App.vue]", `Hello world from Electron ${process.versions.electron}!`);
</script>

<template>
  <div>
    <a href="https://www.electronjs.org/" target="_blank">
      <img alt="Electron logo" class="electron logo" src="./assets/electron.svg">
    </a>
    <a href="https://vitejs.dev/" target="_blank">
      <img alt="Vite logo" class="logo" src="/vite.svg">
    </a>
    <a href="https://vuejs.org/" target="_blank">
      <img alt="Vue logo" class="logo vue" src="./assets/vue.svg">
    </a>
  </div>
  <HelloWorld msg="Electron + Vite + Vue" />
  <div class="flex-center">
    Place static files into the <code>/public</code> folder
    <img alt="Node logo" src="/node.svg" style="width:5em;">
  </div>
</template>

<style>
.flex-center {
  display: flex;
  align-items: center;
  justify-content: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}

.logo.electron:hover {
  filter: drop-shadow(0 0 2em #9FEAF9);
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
