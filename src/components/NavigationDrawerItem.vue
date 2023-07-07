<script setup lang="ts">
const props = defineProps<{
  to: string;
  icon: string;
}>();
const route = useRoute();
const currentPath = computed(() => route.path);
</script>

<template>
  <VListItem
    :active="props.to === currentPath"
    active-class="v-list-item--active text-green"
    :to="props.to"
  >
    <Icon :icon="props.icon" type="iconify" />
  </VListItem>
</template>
