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
    <VDivider />
    <VList density="compact" nav>
      <VListItem v-for="tab in tabs" :key="tab.id">
        <Icon :icon="tab.icon" />
      </VListItem>
    </VList>
  </VNavigationDrawer>
</template>
