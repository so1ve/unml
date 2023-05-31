import type { HookRegister, HookRegisterContext } from "../types";

export async function loadHooks(ctx: HookRegisterContext) {
  const registers = Object.values(
    import.meta.glob<{ registerHooks: HookRegister }>("./*.ts", {
      eager: true,
    }),
  ).map((register) => register.registerHooks);
  for (const register of registers) {
    await register(ctx);
  }
}
