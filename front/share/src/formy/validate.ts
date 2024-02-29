import { VALIDATE_STATION_SLUG_MAX_LEN, VALIDATE_STATION_SLUG_MIN_LEN } from "$server/defs/constants";
import { GET, unwrap } from "$share/client";
import { locale } from "$share/locale";
import { _get } from "$share/net.client";
import { get } from "svelte/store";

export const EMAIL = /^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$/i
export const TWITTER = /^https?:\/\/(www\.)?twitter\.com\/.+/;
export const FACEBOOK = /^https?:\/\/(www\.)?facebook\.com\/.+/;
export const INSTAGRAM = /^https?:\/\/(www\.)?instagram\.com\/.+/;
export const THREADS = /^https?:\/\/(www\.)?threads\.net\/.+/;
export const YOUTUBE = /^https?:\/\/(www\.)?youtube\.com\/.+/;
export const TWITCH = /^https?:\/\/(www\.)?twitch\.tv\/.+/;
export const TIKTOK = /^https?:\/\/(www\.)?tiktok\.com\/.+/;
export const SPOTIFY = /^https?:\/\/((open|www)\.)?spotify\.com\/.+/;
export const RADIOCUT = /^https?:\/\/(www\.)?radiocut\.fm\/.+/;
export const GOOGLE_PLAY = /^https?:\/\/play\.google\.com\/.+/;
export const APP_STORE = /^https?:\/\/apps\.apple\.com\/.+/;

export const SLUG_PATTERN = /^([a-zA-Z0-9\.\-\_]+)$/

export const is_valid_email = (str: string) => EMAIL.test(str);

export const is_empty_string = <T extends string | null | undefined>(v: string | null | undefined): (T extends (null | undefined | "") ? true : false) => {
  // @ts-ignore
  return v == null || v === "";
}

export const _number = ({
  required = false,
  min = null,
  max = null,
}: {
  required?: boolean
  min?: number | null
  max?: number | null
} = {}) => {
  return (v: number | null | undefined) => {
    if(v == null) {
      if(required) {
        return get(locale).validate.required;
      }
    } else {
      if(typeof v !== "number") {
        return `This field must be a number, received ${typeof v}`
      }

      if(Number.isNaN(v)) {
        return get(locale).validate.number;
      }

      if(min != null && v < min) {
        return get(locale).validate.min.replace("@min", String(min));
      }

      if(max != null && v > max) {
        return get(locale).validate.max.replace("@max", String(max));
      }
    }

    return null;
  }
}

export const _string = ({
  required = false,
  minlen = null,
  maxlen = null,
}: {
  required?: boolean
  minlen?: number | null
  maxlen?: number | null
} = {}) => {
  return (v: string | null | undefined) => {
    if(is_empty_string(v)) {
      if(required) {
        return get(locale).validate.required;
    
      }
    } else {
      if(typeof v !== "string") {
        return `This field must a string, received ${typeof v}`;
      }

      if(minlen != null && v.length < minlen) {
        return get(locale).validate.minlen.replace("@minlen", String(minlen))
      }
  
      if(maxlen != null && v.length > maxlen) {
        return get(locale).validate.maxlen.replace("@maxlen", String(maxlen))
      } 
    }
    
    return null;
  }
}

export const _email = ({
  required = false,
  maxlen = null,
}: {
  required?: boolean
  maxlen?: number | null
} = {}) => {
  return (v: string | null | undefined) => {
    if(is_empty_string(v)) {
      if(required) {
        return get(locale).validate.required;
      } 
    } else {
      if(typeof v !== "string") {
        return `This field must be a string, received: ${typeof v}`;
      }
      
      if(!is_valid_email(v)) {
        return get(locale).validate.email;
      }

      if(maxlen != null && v.length > maxlen) {
        return get(locale).validate.maxlen.replace("@maxlen", String(maxlen));
      }
    }   

    return null;    
  }
}


export const _new_user_email = ({
  maxlen = null,
}: {
  maxlen?: number | null
}) => {
  const email = _email({ required: true, maxlen });
  return async (v: string | null | undefined) => {
    let m = email(v);
    if(m != null) return m;

    try {
      const { exists } = await _get<import("$api/auth/user/email-exists/[email]/GET/Output").Output>(`/api/auth/user/email-exists/${String(v).trim().toLowerCase()}`)
      if(exists) {
        return get(locale).validate.email_registered;
      } else {
        return null;
      }
    } catch(e) {
      return null;
    }
  }
}

export const _new_password = ({
  required = true,
  minlen,
  maxlen
}: {
  required?: boolean,
  minlen: number,
  maxlen: number
}) => {
  return (v: string | null | undefined) => {
    if(is_empty_string(v)) {
      if(required) {
        return get(locale).validate.required;
      }  
    } else {
      if(typeof v !== "string") {
        return `This field must be a string, received: ${typeof v}`;
      }

      if(v.length < minlen) {
        return get(locale).validate.new_password.minlen.replace("@minlen", String(minlen));
      }

      if(v.length > maxlen) {
        return get(locale).validate.new_password.maxlen.replace("@maxlen", String(maxlen));
      }
    }

    return null;
  }
}

export const _confirmation_password = ({
  required = true
}: {
  required?: boolean
} = {}) => {
  return ({ password, confirm_password }: { password: string, confirm_password: string }) => {
    
    if(is_empty_string(confirm_password)) {
      if(required) {
        return get(locale).validate.required;
      }
    } else {
      
      if(typeof confirm_password !== "string") {
        return `This field must be a string, received: ${typeof confirm_password}`
      }

      if(password !== confirm_password) {
        return get(locale).validate.confirmation_password;
      }
    }
    
    return null;
  }
}

export const _phone = ({
  required = false,
  whatsapp = false,
  maxlen = null
}: {
  required?: boolean,
  whatsapp?: boolean,
  maxlen?: number | null,
} = {}) => {
  return (v: string | null | undefined) => {
    if(is_empty_string(v)) {
      if(required) {
        return get(locale).validate.required;
      }
    } else {
      if(typeof v !== "string") {
        return `This field must be a string, received: ${typeof v}`
      }

      const helper = v.replace(/[\+\(\)\.\-\ \+]/g, "");
      if(!/[0-9]+/.test(helper) || helper.length < 9 || helper.length > 40) {
        if(whatsapp) {
          return get(locale).validate.phone.whatsapp;
        } else {
          return get(locale).validate.phone.tel;
        }
      }

      if(maxlen != null && v.length > maxlen) {
        return get(locale).validate.maxlen.replace("@maxlen", String(maxlen));
      }
    }

    return null;
  }
}

export const _url = ({
  required = false,
  maxlen = null,
}: {
  required?: boolean,
  maxlen?: number | null,
}) => {
  return (v: string | null | undefined) => {
    if(is_empty_string(v)) {
      if(required) {
        return get(locale).validate.required;
      }
    } else {
      if(typeof v !== "string") {
        return `This field must of a string, received ${typeof v}`;
      }

      let url: URL;
      try {
        url = new URL(v);
      } catch(e) {
        return get(locale).validate.url.valid;
      }

      if(url.protocol !== "https:" && url.protocol !== "http:") {
        return get(locale).validate.url.protocol;
      }

      if(maxlen != null && v.length > maxlen) {
        return get(locale).validate.maxlen.replace("@maxlen", String(maxlen));
      }
    }
    
    return null;
  }
}

export const pattern = ({
  regex,
  message,
  required = false,
  maxlen = null
}: {
  regex: RegExp,
  message: () => string,
  required?: boolean,
  maxlen?: number | null,
}) => {
  return (v: string | null | undefined) => {
    if(is_empty_string(v)) {
      if(required) {
        return get(locale).validate.required;
      }
    } else {
      if(typeof v !== "string") {
        return `This field must be a string, received: ${typeof v}`
      }

      if(!regex.test(v)) {
        return message();
      }

      if(maxlen != null && v.length > maxlen) {
        get(locale).validate.maxlen.replace("@maxlen", String(maxlen));
      }
    }
    
    return null;
  }
}

const Pattern = (regex: RegExp, message: () => string) => {
  return ({
    required = false,
    maxlen = null,
  }: {
    required?: boolean,
    maxlen?: number | null,
  } = {}) => {  
    return pattern({ regex, message, required, maxlen })
  }
}

export const _twitter_url = Pattern(TWITTER, () => get(locale).validate.twitter_url);
export const _facebook_url = Pattern(FACEBOOK, () => get(locale).validate.facebook_url);
export const _instagram_url = Pattern(INSTAGRAM, () => get(locale).validate.instagram_url);
export const _threads_url = Pattern(THREADS, () => get(locale).validate.threads_url);
export const _youtube_url = Pattern(YOUTUBE, () => get(locale).validate.youtube_url);
export const _twitch_url = Pattern(TWITCH, () => get(locale).validate.twitch_url);
export const _tiktok_url = Pattern(TIKTOK, () => get(locale).validate.tiktok_url);
export const _spotify_url = Pattern(SPOTIFY, () => get(locale).validate.spotify_url); 
export const _radiocut_url = Pattern(RADIOCUT, () => get(locale).validate.radiocut_url); 
export const _google_play_url = Pattern(GOOGLE_PLAY, () => get(locale).validate.google_play_url);
export const _app_store_url = Pattern(APP_STORE, () => get(locale).validate.app_store_url);


export const _station_slug = ({
  station_id,
  required = false,
  maxlen = VALIDATE_STATION_SLUG_MAX_LEN,
  minlen = VALIDATE_STATION_SLUG_MIN_LEN,
}: {
  station_id?: string | null | undefined
  maxlen?: number | null,
  minlen?: number | null,
  required?: boolean,
} = {}) => {
  return async (v: string | null | undefined) => {
    
    {
      const message = _string({
        required,
        minlen,
        maxlen
      })(v);

      if(message != null) {
        return message;
      }
    }

    {
      const message = pattern({
        required,
        maxlen,
        regex: SLUG_PATTERN,
        // TODO: locale
        message: () => "The station slug can only contain letters, numbers, dashes, underscores and dots",
      })(v)

      if(message != null) {
        return message;
      }
    }

    try {
      const { is_available } = unwrap(await GET("/stations/is-slug-available", { params: { query: { slug: v!, station_id } } }));

      if(!is_available) {
        // TODO: locale
        return "This slug is already in use by another station";
      } else {
        return null;
      }
    } catch(e) {
      return null;
    } 
  }
}