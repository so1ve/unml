export interface View {
  id: string;
  path: string;
}

export interface Icon {
  type: "html" | "iconify";
  icon: string;
}

export interface Tab {
  id: string;
  view: string;
  icon: Icon;
}
