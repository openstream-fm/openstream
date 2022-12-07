import type { BaseEvent } from "../defs/db/BaseEvent";
import type { EventVariant } from "../defs/db/EventVariant";

export type Event = BaseEvent & EventVariant;