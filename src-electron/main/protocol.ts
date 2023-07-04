import os from "node:os";

import { RESOURCE_PROTOCOL } from "@unml/constants";
import { app, net, protocol } from "electron";
import { normalize } from "pathe";

import { isParentDirectory } from "./utils";

export function preInitProtocol() {
  protocol.registerSchemesAsPrivileged([
    {
      scheme: RESOURCE_PROTOCOL,
      privileges: {
        secure: true,
        standard: true,
        supportFetchAPI: true,
      },
    },
  ]);
}

export function initProtocol() {
  protocol.handle(RESOURCE_PROTOCOL, ({ url }) => {
    const filepath = normalizePath(url.slice(RESOURCE_PROTOCOL.length + 3));
    if (!checkIsPathAllowed(filepath)) {
      return new Response(
        `Cannot load ${filepath} because it is not under the installation directory.`,
        { status: 403 },
      );
    }

    return net.fetch(`file://${filepath}`);
  });
}

function normalizePath(path: string) {
  if (os.platform() !== "win32") {
    return path;
  }
  path = normalize(path);
  const splitted = path.split("/");
  const drive = splitted[0].toUpperCase();
  const rest = splitted.slice(1).join("/");

  return `${drive}:/${rest}`;
}

function checkIsPathAllowed(path: string) {
  const installationDir = normalize(app.getAppPath());

  return isParentDirectory(installationDir, path);
}
