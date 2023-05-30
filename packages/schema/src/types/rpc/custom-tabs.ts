export interface CustomTab {
  /** The id of the tab, must be unique */
  id: string;
  /** Icon of the tab, support any Iconify icons, or a url to an image */
  icon?: string;
  /** Needs investigation Since we do not show the name now */
  // /**
  //  * Title of the tab
  //  */
  // title: string;
  /** View ID */
  viewId: string;
}