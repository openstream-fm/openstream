import type { BaseConnection } from "../defs/db/BaseConnection"
import type { ConnectionState } from "../defs/db/ConnectionState"

export type Connection  = BaseConnection & ConnectionState;