<script setup lang="ts">
// const unml = useUnml();

function minimize() {
  rpc["window:minimize"]();
}
function close() {
  rpc["window:close"]();
}
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
        class="non-draggable"
        size="x-small"
        variant="text"
        @click="minimize"
      >
        <span class="i-material-symbols:minimize-rounded text-lg" />
      </VBtn>
      <VBtn class="non-draggable" size="x-small" variant="text" @click="close">
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
