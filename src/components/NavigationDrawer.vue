<script setup lang="ts">
// const customTabs = await rpc.getCustomTabs();
</script>

<template>
  <VNavigationDrawer
    class="bg-u-black!"
    disable-resize-watcher
    disable-route-watcher
    :elevation="0"
    permanent
    rail
  >
    <!--
      <VListItem
      nav
      prepend-avatar="https://randomuser.me/api/portraits/women/75.jpg"
      /> 
    -->
    <VDivider />
    <VList
      density="compact"
      nav
    >
      <!--
        <VListItem v-for="tab in customTabs" :key="tab.id">
        <span :class="tab.icon" />
        </VListItem> 
      -->
    </VList>
  </VNavigationDrawer>
</template>