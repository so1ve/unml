import { useUnml } from "@unml/kit";

import type { Controller } from "../types";

const controller: Controller = () => {
  const unml = useUnml();

  unml.hook("ui:tabs", (tabs) => {
    tabs.push({
      name: "Extensions",
      path: "/extensions",
    });
  });
};

export default controller;
