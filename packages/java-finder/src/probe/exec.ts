import { execFile } from "node:child_process";

export const execFileText = (
  file: string,
  args: string[],
  options: { timeoutMs: number; windowsHide?: boolean },
): Promise<string> =>
  new Promise((resolve, reject) => {
    execFile(
      file,
      args,
      {
        timeout: options.timeoutMs,
        windowsHide: options.windowsHide ?? true,
        encoding: "utf8",
        maxBuffer: 1024 * 1024,
      },
      (error, stdout, stderr) => {
        const combined = `${stdout ?? ""}${stderr ?? ""}`;
        if (
          error && // Still return output for parsing; caller may decide.
          // But if there's no output, treat as failure.
          !combined.trim()
        ) {
          reject(error);

          return;
        }
        resolve(combined);
      },
    );
  });
