import type { Request, Response, NextFunction } from "express";

export const json = <T>(fn: (req: Request, res: Response) => Promise<T>) => {
  return async (req: Request, res: Response, next: NextFunction) => {
    let v: T;
    try {
      v = await fn(req, res);
    } catch(e) {
      return next(e);
    }

    res.json(v);
  }
}