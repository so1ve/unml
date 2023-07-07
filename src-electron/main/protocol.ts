import { RESOURCE_PROTOCOL } from "@unml/constants";
import { isParentDirectory, normalizePath } from "@unml/utils";
import { app, net, protocol } from "electron";
import { normalize } from "pathe";

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

function checkIsPathAllowed(path: string) {
  const installationDir = normalize(app.getAppPath());

  return isParentDirectory(installationDir, path);
}
