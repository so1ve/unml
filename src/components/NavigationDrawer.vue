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
      <VListItem to="/">
        <Icon icon="material-symbols:home-rounded" type="iconify" />
      </VListItem>
    </VList>
    <VDivider />
    <!-- TODO: Loading -->
    <VList density="compact" nav>
      <VListItem
        v-for="tab in tabs"
        :key="tab.id"
        :to="'path' in tab ? tab.path : `/extension/custom/${tab.view}`"
      >
        <Icon v-bind="tab.icon" />
      </VListItem>
    </VList>
  </VNavigationDrawer>
</template>
