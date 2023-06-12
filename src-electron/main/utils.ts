import { withTrailingSlash } from "ufo";

export function isParentDirectory(dir: string, file: string): boolean {
  dir = withTrailingSlash(dir);

  // TODO: case sensitive filesystem
  return file.startsWith(dir);
}
