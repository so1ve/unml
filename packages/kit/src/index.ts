import { COMMAND_CLIENT_CALL } from "@unml/constants";
import type { CommandFn, Tab, Unml, View } from "@unml/schema";
import { getContext } from "unctx";

export * from "./helpers";

export const unmlCtx = getContext<Unml>("unml");

function useUnml() {
	const instance = unmlCtx.tryUse();
	if (!instance) {
		throw new Error("UNML instance is unavailable!");
	}

	return instance;
}

export const addHook: Unml["hook"] = (...args) => useUnml().hook(...args);
export const addHooks: Unml["addHooks"] = (...args) =>
	useUnml().addHooks(...args);
export const callHook: Unml["callHook"] = (...args) =>
	useUnml().callHook(...args);

export function addView(view: View) {
	addHook("ui:views", (views) => {
		views.push(view);
	});
}

export function addTab(tab: Tab) {
	addHook("ui:tabs", (tabs) => {
		tabs.push(tab);
	});
}

export function exposeNodeCommand(name: string, fn: CommandFn) {
	const unml = useUnml();
	if (unml.commands.has(name)) {
		throw new Error(`Command "${name}" is already exposed!`);
	}
	unml.commands.set(name, fn);
}

export async function callNodeCommand(name: string, ...args: any[]) {
	const unml = useUnml();
	const fn = unml.commands.get(name);
	if (!fn) {
		throw new Error(`Command "${name}" is not exposed!`);
	}

	return await fn(...args);
}

export const callClientCommand = async (name: string, ...args: any[]) =>
	await callNodeCommand(COMMAND_CLIENT_CALL, name, ...args);
