<script setup lang="ts">
import type { Tab } from "@unml/schema";

const { callNodeCommand } = useClient();
const { state: tabs } = useAsyncState(
	() => callNodeCommand<Tab[]>("ui:getTabs"),
	[],
);
</script>

<template>
	<VNavigationDrawer
		bg="u-black!"
		disable-resize-watcher
		disable-route-watcher
		:elevation="0"
		permanent
		rail
	>
		<VList density="compact" nav>
			<VListItem to="/">
				<Icon icon="material-symbols:home-rounded" type="iconify" />
			</VListItem>
		</VList>
		<VDivider />
		<!-- TODO: Loading -->
		<VList density="compact" nav>
			<VListItem v-for="tab in tabs" :key="tab.id" :to="`/extension/${tab.id}`">
				<Icon v-bind="tab.icon" />
			</VListItem>
		</VList>
	</VNavigationDrawer>
</template>
