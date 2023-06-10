<script setup lang="ts">
import { Icon } from "@iconify/vue";

import type { Icon as IconType } from "@unml/schema";

const props = defineProps<{
  icon: IconType;
}>();
</script>

<template>
  <span v-if="props.icon.type === 'svg'" v-html="props.icon.value" />
  <Icon
    v-else-if="props.icon.type === 'iconify'"
    class="text-2xl"
    :icon="(props.icon.value as any)"
  />
</template>
