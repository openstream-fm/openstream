import type { Request, Response, NextFunction } from "express";

const next_once = (fn: NextFunction): NextFunction => {
  let called = false;
  let next: NextFunction = (...args) => {
    if(called) return;
    called = true;
    return fn(...args);
  }
  return next;
}

export const json = <Params, T>(fn: (req: Request<Params>, res: Response) => Promise<T>) => {
  return async (req: Request<Params>, res: Response, next: NextFunction) => {
    let v: T;
    try {
      v = await fn(req, res);
    } catch(e) {
      return next(e);
    }

    res.json(v);
  }
}

export const handler = <T>(fn: (req: Request, res: Response, next: NextFunction) => Promise<void>) => {
  return async (req: Request, res: Response, _next: NextFunction) => {
    const next = next_once(_next);
    try {
      await fn(req, res, next);
    } catch(e) {
      return next(e);
    }
  }
}