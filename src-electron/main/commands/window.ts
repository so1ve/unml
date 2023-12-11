import { exposeNodeCommand } from "@unml/kit";

import type { CommandRegister } from "../types";

export const registerCommands: CommandRegister = ({ win }) => {
	exposeNodeCommand("window:minimize", () => win.minimize());
	exposeNodeCommand("window:maximize", () => win.maximize());
	exposeNodeCommand("window:close", () => win.close());
};
