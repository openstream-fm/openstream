import { Router, json } from "express";
import { ApiError } from "./error";
export const api = (config) => {
    let api = Router();
    api.use(json());
    let pages = Router();
    api.use("/pages", pages);
    api.use((e, req, res, next) => {
        const error = ApiError.from(e);
        res.status(e.status).json(error);
    });
    return api;
};
