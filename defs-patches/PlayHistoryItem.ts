import type { BasePlayHistoryItem } from "../defs/db/BasePlayHistoryItem"
import type { PlayHistoryItemKind } from "../defs/db/PlayHistoryItemKind"

export type PlayHistoryItem  = BasePlayHistoryItem & PlayHistoryItemKind;