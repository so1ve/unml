import type { CommandRegister, CommandRegisterContext } from "../types";

export async function loadCommands(ctx: CommandRegisterContext) {
	const registers = Object.values(
		import.meta.glob<{ registerCommands: CommandRegister }>("./*.ts", {
			eager: true,
		}),
	).map((register) => register.registerCommands);
	for (const register of registers) {
		await register(ctx);
	}
}
