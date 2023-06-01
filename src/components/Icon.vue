<script setup lang="ts">
import { Icon } from "@iconify/vue";

const props = defineProps<{
  icon: string;
}>();
</script>

<template>
  <Icon class="text-2xl" :icon="props.icon" />
</template>
