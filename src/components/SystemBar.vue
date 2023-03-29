<script setup lang="ts">
const unml = useUnml();

console.log(unml);
</script>

<template>
  <VSystemBar
    class="bg-u-black! draggable"
    density="compact"
    :elevation="0"
    window
  >
    <div>
      <VBtn
        class="ma-2 non-draggable"
        size="x-small"
        variant="text"
      >
        <span class="i-material-symbols:minimize-rounded text-lg" />
      </VBtn>
      <VBtn
        class="ma-2 non-draggable"
        size="x-small"
        variant="text"
      >
        <span class="i-material-symbols:close-rounded text-lg" />
      </VBtn>
    </div>
  </VSystemBar>
</template>

<style scoped>
.draggable {
  -webkit-user-select: none;
  -webkit-app-region: drag;
}

.non-draggable {
  -webkit-app-region: no-drag;
}
</style>
