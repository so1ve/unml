export function handleExtensions() {
	const client = useClient();
	window.__UNML_API__.onCallClientCommand(client.callClientCommand);
}
