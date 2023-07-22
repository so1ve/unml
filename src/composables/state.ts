import type { Tab, View } from "@unml/schema";

const TAB_ROUTE_RE = /^\/extension\/(?!custom\/)/;

export function useTabs() {
	const { callNodeCommand } = useClient();
	const { state: customTabs, isLoading } = useAsyncState(
		callNodeCommand<Tab[]>("ui:getTabs"),
		[],
	);
	const router = useRouter();
	const builtinTabs = computed(() =>
		router
			.getRoutes()
			.filter((r) => r.name && TAB_ROUTE_RE.test(r.path))
			.sort((a, b) => (a.meta.order ?? 100) - (b.meta.order ?? 100))
			.map<Tab>((r) => ({
				id: r.name as string,
				icon: {
					type: "iconify",
					icon: r.meta.icon,
				},
				path: r.path,
			})),
	);

	const tabs = computed(() => [...builtinTabs.value, ...customTabs.value]);

	return { tabs, isLoading };
}

export function useViews() {
	const { callNodeCommand } = useClient();
	const { state: views, isLoading } = useAsyncState(
		callNodeCommand<View[]>("ui:getViews"),
		[],
	);

	return { views, isLoading };
}
