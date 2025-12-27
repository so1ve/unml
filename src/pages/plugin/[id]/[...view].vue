<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";

const route = useRoute("/plugin/[id]/[...view]");
const pluginId = computed(() => route.params.id);
const viewPath = computed(() => route.params.view);

const pluginUrl = computed(
  () => `unml-plugin://${pluginId.value}/${viewPath.value ?? "index.html"}`,
);
</script>

<template>
  <div class="w-full h-full flex flex-col">
    <div class="p-2 bg-gray-100 border-b">Plugin Container: {{ pluginId }}</div>
    <iframe
      class="flex-1 w-full border-none"
      sandbox="allow-scripts allow-same-origin"
      :src="pluginUrl"
    />
  </div>
</template>
