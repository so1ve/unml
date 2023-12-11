<script setup lang="ts">
const route = useRoute();

const { t } = useI18n();
const { views, isLoading } = useViews();
const viewId = computed(() => route.params.viewId);
const view = computed(() => views.value.find((v) => v.id === viewId.value));
</script>

<template>
	<div v-if="!view && !isLoading">Custom view {{ viewId }} not found</div>
	<Loading
		v-else-if="!view && isLoading"
		:loading="isLoading"
		:text="t('pages.custom-view.loading', { page: viewId })"
	/>
	<IframeView v-else :view="view!" />
</template>
