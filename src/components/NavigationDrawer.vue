<script setup lang="ts">
const { tabs } = useTabs();
</script>

<template>
  <VNavigationDrawer
    bg="u-black!"
    :elevation="0"
    :expand-on-hover="false"
    mini-variant
    permanent
    rail
  >
    <VListItem nav py-3 rounded to="/user">
      <VAvatar>
        <img alt="John" src="https://cdn.vuetifyjs.com/images/john.jpg" />
      </VAvatar>
    </VListItem>
    <VDivider />
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
