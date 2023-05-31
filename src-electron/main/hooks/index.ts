import type { HookRegister, HookRegisterContext } from "../types";

export async function loadHooks(ctx: HookRegisterContext) {
  const hooks = Object.values(
    import.meta.glob<{ registerHooks: HookRegister }>("./*.ts", {
      eager: true,
    }),
  ).map((hook) => hook.registerHooks);
  for (const hook of hooks) {
    await hook(ctx);
  }
}
