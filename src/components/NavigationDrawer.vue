<script setup lang="ts">
const { tabs } = useTabs();
</script>

<template>
  <VNavigationDrawer
    bg="u-black!"
    disable-resize-watcher
    disable-route-watcher
    :elevation="0"
    permanent
    rail
  >
    <VList density="compact" nav>
      <NavigationDrawerItem icon="material-symbols:home-rounded" to="/" />
    </VList>
    <VDivider />
    <!-- TODO: Loading -->
    <VList density="compact" nav>
      <NavigationDrawerTabItem v-for="tab in tabs" :key="tab.id" :tab="tab" />
    </VList>
  </VNavigationDrawer>
</template>
