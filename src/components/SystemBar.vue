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
    class="draggable bg-u-black!"
    density="compact"
    :elevation="0"
    window
  >
    {{ `${2}` }}
    <div class="flex gap-1">
      <VBtn class="non-draggable" size="small" variant="text" @click="minimize">
        <span class="i-material-symbols:minimize-rounded text-lg" />
      </VBtn>
      <VBtn class="non-draggable" size="small" variant="text" @click="close">
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
