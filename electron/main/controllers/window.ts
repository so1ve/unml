// import { useUnml } from "@unml/kit";

import type { Controller } from "../types";

import { useUnml, useUnmlServer } from "@unml/kit";

const controller: Controller = (win) => {
  const unmlServer = useUnmlServer();
  const unml = useUnml();

  const show = () => {
    win.show();
  };
  const minimize = () => {
    win.minimize();
  };
  const close = () => {
    win.close();
  };
  unmlServer.extendServerRpc("window", {
    show,
    minimize,
    close,
  });
  unml.hook("window:minimize", minimize);
  unml.hook("window:show", show);
  unml.hook("window:close", close);
};

export default controller;
