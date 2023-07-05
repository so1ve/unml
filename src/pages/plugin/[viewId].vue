<script setup lang="ts">
import type { View } from "@unml/schema";
import { pathToResourceUrl } from "@unml/utils";

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
  <iframe
    v-else
    class="h-[calc(100vh_-_32px)] w-[calc(32px_+_100%)] -m-4"
    :src="iframeUrl"
  ></iframe>
</template>
