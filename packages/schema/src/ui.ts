export interface View {
  id: string;
  path: string;
}

export interface Icon {
  type: "html" | "iconify";
  value: string;
}

export interface Tab {
  id: string;
  view: string;
  icon: Icon;
}
