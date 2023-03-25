import { createContext } from "unctx";

import { Unml } from "./types";

const context = createContext<Unml>();

const withEnsureContext = <A extends any[], T extends (...args: A) => any>(fn: T) => (...args: A): ReturnType<T> => {
  if (!context.tryUse()) {
    context.set(new Unml());
  }
  return fn(...args);
};

export const useUnml = withEnsureContext(context.use);
export const tryUseUnml = withEnsureContext(context.tryUse);
