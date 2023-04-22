import fsg from "node:fs/promises";

import fg from "fast-glob";

const main = async () => {
  const stubbedDistFiles = await fg("packages/*/dist/*.mjs");
  for (const stubbedDistFile of stubbedDistFiles) {
    const fileContent = await fsg.readFile(stubbedDistFile, "utf-8");
    const newFileContent = fileContent.replace(
      /^import jiti from ".+";/,
      'import jiti from "jiti";'
    );
    await fsg.writeFile(stubbedDistFile, newFileContent);
  }
};

main();
