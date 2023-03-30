import type { Extension } from "../types";

const extension: Extension = async () => {
  const unml = useUnml();

  const tabs = [] as any[];

  await unml.callHook("ui:tabs", tabs);

  console.log(tabs);
};

export default extension;
