import type { Controller } from "../types";

import windowController from "./window";

const registerControllers: Controller = (win) => {
  windowController(win);
};

export default registerControllers;
