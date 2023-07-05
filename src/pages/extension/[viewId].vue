<script setup lang="ts">
import type { View } from "@unml/schema";

const route = useRoute();
const { callNodeCommand } = useClient();

const views = await callNodeCommand<View[]>("ui:getViews");
const viewId = computed(() => route.params.viewId);
const view = computed(() => views.find((v) => v.id === viewId.value));
</script>

<template>
  <div v-if="!view">View {{ viewId }} not found</div>
  <IframeView v-else :view="view" />
</template>
