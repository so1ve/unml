<script setup lang="ts">
import { pathToResourceUrl } from "@unml/utils";
import type { View } from "@unml/schema";

const route = useRoute();
const { callNodeCommand } = useClient();

const views = await callNodeCommand<View[]>("ui:getViews");
const viewId = computed(() => route.params.viewId);
const view = computed(() => views.find((v) => v.id === viewId.value));
const iframeUrl = computed(() => {
  if (!view.value) {
    return "";
  }

  return pathToResourceUrl(view.value.path);
});
</script>

<template>
  <div v-if="!view">View {{ viewId }} not found</div>
  <iframe v-else :src="iframeUrl"></iframe>
</template>
