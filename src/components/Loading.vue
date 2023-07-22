<script setup lang="ts">
const props = defineProps<{
	loading: boolean;
	text?: string;
}>();
</script>

<template>
	<VOverlay items-center justify-center :model-value="props.loading">
		<div flex flex-col gap-4 items-center justify-center>
			<VProgressCircular indeterminate size="64" />
			{{ props.text }}
		</div>
	</VOverlay>
</template>
