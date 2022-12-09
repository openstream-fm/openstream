import { BaseAccessToken } from "../defs/db/BaseAccessToken"
import { AccessTokenScope } from "../defs/db/AccessTokenScope"
import { AccessTokenGeneratedBy } from "../defs/db/AccessTokenGeneratedBy";

export type AccessToken = BaseAccessToken & AccessTokenScope & AccessTokenGeneratedBy;