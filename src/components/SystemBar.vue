<script setup lang="ts">
const { callNodeCommand } = useClient();

async function minimize() {
  await callNodeCommand("window:minimize");
}
async function close() {
  await callNodeCommand("window:close");
}
</script>

<template>
  <VSystemBar
    bg="u-black!"
    class="draggable"
    density="compact"
    :elevation="0"
    window
  >
    <div flex mt--2 mx--2>
      <VBtn
        class="non-draggable"
        hover:bg="gray-300/50"
        size="x-small"
        u-systembar-control
        variant="text"
        @click="minimize"
      >
        <span i-material-symbols:minimize-rounded text-xs />
      </VBtn>
      <VBtn
        class="non-draggable"
        hover:bg="red-600"
        size="x-small"
        u-systembar-control
        variant="text"
        @click="close"
      >
        <span i-material-symbols:close-rounded text-xs />
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
