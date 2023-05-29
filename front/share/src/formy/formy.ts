import { MaybePromise } from "./util";

export type FormyValidation = () => MaybePromise<string | null>;
export type ValidationItem = { fn: FormyValidation, parent_element: Element };

export type FormyContext = {
  add: (item: ValidationItem) => () => void
}

export const FORMY_KEY = Symbol.for("openstream.formy.context");