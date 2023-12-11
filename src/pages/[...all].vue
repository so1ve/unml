<script setup lang="ts">
const route = useRoute();
const { t } = useI18n();
</script>

<template>
	{{ t("common.not-found", { page: route.path }) }}
</template>
