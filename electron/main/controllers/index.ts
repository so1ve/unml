import type { Controller } from "../types";

const controllers = import.meta.glob<{ default: Controller }>("./*.ts", {
  eager: true,
});

const registerControllers: Controller = (win) => {
  for (const controller of Object.values(controllers)) {
    controller.default(win);
  }
};

export default registerControllers;
