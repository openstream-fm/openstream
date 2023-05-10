export const EMAIL = /^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$/i
export const TWITTER = /^https:\/\/twitter\.com\/.+/;
export const FACEBOOK = /^https:\/\/www\.facebook\.com\/.+/;
export const INSTAGRAM = /^https:\/\/www\.instagram\.com\/.+/;
export const YOUTUBE = /^https:\/\/www\.youtube\.com\/.+/;
export const TWITCH = /^https:\/\/twitch\.tv\/.+/;
export const GOOGLE_PLAY = /^https:\/\/play\.google\.com\/.+/;
export const APP_STORE = /^https:\/\/apps\.apple\.com\/.+/;


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
        return "This field is required";
      }
    } else {
      if(typeof v !== "number") {
        return `This field must be a number, received ${typeof v}`
      }

      if(Number.isNaN(v)) {
        return `This field must a valid number`;
      }

      if(min != null && v < min) {
        return `This field must be ${min} or greater`
      }

      if(max != null && v > max) {
        return `This field must be ${max} or less`
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
        return "This field is required"
    
      }
    } else {
      if(typeof v !== "string") {
        return `This field must a string, received ${typeof v}`;
      }

      if(minlen != null && v.length < minlen) {
        return `This field must have ${minlen} characters or more`
      }
  
      if(maxlen != null && v.length > maxlen) {
        return `This field must have ${maxlen} characters or less`
      } 
    }
    
    return null;
  }
}

export const _email = ({
  required = false,
}: {
  required?: boolean
} = {}) => {
  return (v: string | null | undefined) => {
    if(is_empty_string(v)) {
      if(required) {
        return "This field is required";
      } 
    } else {
      if(typeof v !== "string") {
        return `This field must be a string, received: ${typeof v}`;
      }
      
      if(!is_valid_email(v)) {
        return "This field must be a valid email address";
      }
    }   

    return null;    
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
        return "This field is required"
      }  
    } else {
      if(typeof v !== "string") {
        return `This field must be a string, received: ${typeof v}`;
      }

      if(v.length < minlen) {
        return `New password must have ${minlen} characters or more`
      }

      if(v.length > maxlen) {
        return `New password must have ${minlen} characters or less`
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
        return "Cofirmation password is required"
      }
    } else {
      
      if(typeof confirm_password !== "string") {
        return `This field must be a string, received: ${typeof confirm_password}`
      }

      if(password !== confirm_password) {
        return "Confirmation password does not match"
      }
    }
    
    return null;
  }
}

export const _phone = ({
  required = false,
  whatsapp = false,
}: {
  required?: boolean,
  whatsapp?: boolean,
} = {}) => {
  return (v: string | null | undefined) => {
    if(is_empty_string(v)) {
      if(required) {
        return "This field is required"
      }
    } else {
      if(typeof v !== "string") {
        return `This field must be a string, received: ${typeof v}`
      }

      const helper = v.replace(/[\+\(\)\.\-\ \+]/g, "");
      if(!/[0-9]+/.test(helper) || helper.length < 9 || helper.length > 40) {
        if(whatsapp) {
          return `This field must be a valid international WhatsApp number`;
        } else {
          return `This field must be a valid international phone number`;
        }
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
        return `This field is required`;
      }
    } else {
      if(typeof v !== "string") {
        return `This field must of a string, received ${typeof v}`;
      }

      let url: URL;
      try {
        url = new URL(v);
      } catch(e) {
        return "This field must be a valid URL"
      }

      if(url.protocol !== "https:" && url.protocol !== "http:") {
        return "This field must start with http:// or https://";
      }

      if(maxlen != null && v.length > maxlen) {
        return `This field must have ${maxlen} characters or less`;
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
  message: string,
  required?: boolean,
  maxlen?: number | null,
}) => {
  return (v: string | null | undefined) => {
    if(is_empty_string(v)) {
      if(required) {
        return `This field is required`
      }
    } else {
      if(typeof v !== "string") {
        return `This field must be a string, received: ${typeof v}`
      }

      if(!regex.test(v)) {
        return message;
      }

      if(maxlen != null && v.length > maxlen) {
        `This field must have ${maxlen} characters or less`;
      }
    }
    
    if((v != null && v !== "") && !regex.test(v)) {
      return message;
    }

    return null;
  }
}

const Pattern = (regex: RegExp, message: string) => {
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

export const _twitter_url = Pattern(TWITTER, "This field must be a valid Twitter URL, navigate to your Twitter page and copy the entire URL from there");
export const _facebook_url = Pattern(FACEBOOK, "This field must be a valid Facebook URL, navigate to your Facebook page and copy the entire URL from there");
export const _instagram_url = Pattern(INSTAGRAM, "This field must be a valid Instagram URL, navigate to your Instagram page and copy the entire URL from there");
export const _youtube_url = Pattern(YOUTUBE, "This field must be a valid Youtube URL, navigate to your Youtube page and copy the entire URL from there");
export const _twitch_url = Pattern(TWITCH, "This field must be a valid Twitch URL, navigate to your Twitch page and copy the entire URL from there");
export const _google_play_url = Pattern(GOOGLE_PLAY, "This field must be a valid Google Play URL, navigate to your app's page at Google Play and copy the entire URL from there");
export const _app_store_url = Pattern(APP_STORE, "This field must be a valid App Store URL, navigate to your app's page at App Store and copy the entire URL from there");