/// This file is auto generated from its Rust definition, do not edit manually


/** Access token header used in API endpoints */
export const ACCESS_TOKEN_HEADER = "x-access-token";

/** access token autoremove validity
 *  remove access tokens that are not used in the last X time
 *  this is only for Login, Register or AdminAsUser access tokens
 *  Cli or Api(not AdminAsUser) access tokens does not auto-expire */
export const ACCESS_TOKEN_NOT_USED_AUTOREMOVE_SECS = 604800;

/** time in seconds for which an account invitation can be accepted (or rejected) */
export const ACCOUNT_INVITATION_VALIDITY_SECS = 604800;

/** limit of authotization, or other sensible api endpoints requests from the same ip */
export const API_IP_LIMIT = 60;

/** limit restart interval of API_IP_LIMIT */
export const API_IP_LIMIT_DURATION_SECS = 60;

/** audio file byte rate */
export const AUDIO_FILE_BYTERATE = 16000;

/** audio file chunk size in bytes */
export const AUDIO_FILE_CHUNK_SIZE = 256000;

export const DEPLOYMENT_HEALTH_CHECK_INTERVAL_SECS = 1;

/** time in seconds for which an email verification code is valid */
export const EMAIL_VERIFICATION_CODE_LEN = 6;

/** time in seconds for which an email verification code is valid */
export const EMAIL_VERIFICATION_VALIDITY_SECS = 3600;

/** delay of which if external relay produced no data, it will be cancelled */
export const EXTERNAL_RELAY_NO_DATA_SHUTDOWN_SECS = 10;

/** delay of which if external doesn't produce first data chunk, it will be cancelled */
export const EXTERNAL_RELAY_NO_DATA_START_SHUTDOWN_SECS = 30;

export const EXTERNAL_RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS = 60;

/** Internal forwarded ip header used when openstream servers are connecting with each other */
export const FORWARD_IP_HEADER = "x-openstream-forwarded-ip";

/** Access token header used by payments servers implementations */
export const PAYMENTS_ACCESS_TOKEN_HEADER = "x-access-token";

/** delay to shutdown a playlist media session when it run out of listeners */
export const PLAYLIST_NO_LISTENERS_SHUTDOWN_DELAY_SECS = 10;

/** External ip header used for connecting from a reverse proxy */
export const REAL_IP_HEADER = "x-real-ip";

/** delay to shutdown a relay session when it run out of listeners */
export const RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS = 10;

/** we need to update this value after making code changes to the station pictures logic or sizes
 *  changing to this value will make startup check and recreation of outdated images
 *  and invalidation of service workers station pictures caches */
export const STATION_PICTURES_VERSION = 5.0;

/** stream bust len in elements (use with STREAM_CHUNK_SIZE) */
export const STREAM_BURST_LENGTH = 12;

/** stream tokio broadcaster stream channel capacity */
export const STREAM_CHANNEL_CAPACITY = 16;

/** stream chunk size */
export const STREAM_CHUNK_SIZE = 16000;

/** limit of concurrent stream connections from the same ip */
export const STREAM_IP_CONNECTIONS_LIMIT = 8;

/** stream kbps */
export const STREAM_KBITRATE = 128;

/** time in seconds for which an user recovery token code is valid */
export const TOKEN_USER_RECOVERY_VALIDITY_SECS = 3600;

/** station's transfer save interval in milliseconds */
export const TRANSFER_SAVE_INTERVAL_MILLIS = 5000;

export const VALIDATE_ACCOUNT_NAME_MAX_LEN = 60;

export const VALIDATE_ACCOUNT_NAME_MIN_LEN = 1;

export const VALIDATE_ADMIN_FIRST_NAME_MAX_LEN = 100;

export const VALIDATE_ADMIN_LAST_NAME_MAX_LEN = 100;

export const VALIDATE_ADMIN_PASSWORD_MAX_LEN = 60;

export const VALIDATE_ADMIN_PASSWORD_MIN_LEN = 8;

export const VALIDATE_STATION_DESC_MAX_LEN = 4000;

export const VALIDATE_STATION_DESC_MIN_LEN = 1;

export const VALIDATE_STATION_EMAIL_MAX_LEN = 100;

export const VALIDATE_STATION_EXTERNAL_RELAY_URL_MAX_LEN = 200;

export const VALIDATE_STATION_FREQUENCY_MAX = 100000.0;

export const VALIDATE_STATION_FREQUENCY_MIN = 0.0;

export const VALIDATE_STATION_NAME_MAX_LEN = 60;

export const VALIDATE_STATION_NAME_MIN_LEN = 1;

export const VALIDATE_STATION_PHONE_MAX_LEN = 60;

export const VALIDATE_STATION_SLOGAN_MAX_LEN = 100;

export const VALIDATE_STATION_SLOGAN_MIN_LEN = 1;

export const VALIDATE_STATION_URLS_MAX_LEN = 150;

export const VALIDATE_STATION_WHATSAPP_MAX_LEN = 60;

export const VALIDATE_USER_EMAIL_MAX_LEN = 80;

export const VALIDATE_USER_FIRST_NAME_MAX_LEN = 100;

export const VALIDATE_USER_LAST_NAME_MAX_LEN = 100;

export const VALIDATE_USER_PASSWORD_MAX_LEN = 60;

export const VALIDATE_USER_PASSWORD_MIN_LEN = 8;

export const VALIDATE_USER_PHONE_MAX_LEN = 40;