import { Config } from "./config";
import { Request, Response, NextFunction, Router, json } from "express";
import { ApiError } from "./error";

export const api = (config: Config) => {

  let api = Router();
  api.use(json())

  let pages = Router();

  api.use("/pages", pages);

  api.use((e: any, req: Request, res: Response, next: NextFunction) => {
    const error = ApiError.from(e);
    res.status(e.status).json(error);
  })

  return api;

}