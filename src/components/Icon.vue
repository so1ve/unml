<script setup lang="ts">
import { Icon } from "@iconify/vue";

const props = defineProps<{
  type: "html" | "iconify";
  icon: string;
}>();
</script>

<template>
  <span v-if="props.type === 'html'" v-html="props.icon" />
  <Icon v-else-if="props.type === 'iconify'" :icon="props.icon" text-2xl />
</template>
