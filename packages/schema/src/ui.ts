export interface View {
  id: string;
  path: string;
  persistent?: boolean;
}

export interface Icon {
  type: "html" | "iconify";
  icon: string;
}

interface TabBase {
  id: string;
  icon: Icon;
}
type TabView = TabBase & {
  view: string;
};
type TabPath = TabBase & {
  path: string;
};
export type Tab = TabView | TabPath;
