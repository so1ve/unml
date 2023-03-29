<script setup lang="ts">

</script>

<template>
  <VSystemBar
    class="bg-u-black!"
    density="compact"
    :elevation="0"
    window
  >
    <div class="h-full titlebar w-full" />
    <span class="flex flex-grow-0 flex-shrink h-full p-0">
      <span
        class="cursor-pointer flex items-center px-3 py-1"
        variant="plain"
        @click="minimize"
      >
        <span class="i-material-symbols:minimize-rounded" />
      </span>
      <span
        class="cursor-pointer flex items-center px-3 py-1"
        variant="plain"
        @click="close"
      >
        <span class="i-material-symbols:close-rounded" />
      </span>
    </span>
  </VSystemBar>
</template>

<style scoped>
.titlebar {
  -webkit-user-select: none;
  -webkit-app-region: drag;
}
</style>
