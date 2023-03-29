<template>
  <VNavigationDrawer
    class="bg-u-black!"
    disable-resize-watcher
    disable-route-watcher
    :elevation="0"
    permanent
    rail
  >
    <VListItem
      nav
      prepend-avatar="https://randomuser.me/api/portraits/women/75.jpg"
    />
    <VDivider />
    <VList
      density="compact"
      nav
    >
      <VListItem prepend-icon="mdi-view-dashboard" value="dashboard" />
      <VListItem prepend-icon="mdi-forum" value="messages" />
    </VList>
  </VNavigationDrawer>
</template>
