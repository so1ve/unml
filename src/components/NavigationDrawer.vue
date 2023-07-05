<script setup lang="ts">
import type { Tab } from "@unml/schema";

const { callNodeCommand } = useClient();
const tabs = await callNodeCommand<Tab[]>("ui:getTabs");
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
    <VList density="compact" nav>
      <VListItem to="/">
        <Icon icon="material-symbols:home-rounded" type="iconify"></Icon>
      </VListItem>
    </VList>
    <VDivider />
    <VList density="compact" nav>
      <VListItem v-for="tab in tabs" :key="tab.id" :to="`/plugin/${tab.id}`">
        <Icon v-bind="tab.icon" />
      </VListItem>
    </VList>
  </VNavigationDrawer>
</template>
