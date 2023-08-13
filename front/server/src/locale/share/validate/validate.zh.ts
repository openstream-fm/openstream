/// file: validate.zh.ts
const locale: import("./validate.locale").ValidateLocale = {
  "required": "此字段为必填项",
  
  "number": "此字段必须为有效数字",
  "email": "此字段必须为有效电子邮件地址",
  
  "min": "此字段必须为@min或更多",
  "max": "此字段必须为@max或更少",
  
  "minlen": "此字段必须至少包含@minlen个字符",
  "maxlen": "此字段必须最多包含@maxlen个字符",

  "new_password": {
    "minlen": "新密码必须至少包含@minlen个字符",
    "maxlen": "新密码必须最多包含@maxlen个字符"
  },

  "confirmation_password": "确认密码不匹配",
  
  "phone": {
    "tel": "此字段必须为有效国际电话号码",
    "whatsapp": "此字段必须为有效国际WhatsApp号码",
  },

  "url": {
    "valid": "此字段必须为有效URL",
    "protocol": "此字段必须以http://或https://开头",
  },

  "email_registered": "此电子邮件已注册",

  "twitter_url": "此字段必须为有效的Twitter URL，请访问您的Twitter页面并从那里复制完整的URL",
  "facebook_url": "此字段必须为有效的Facebook URL，请访问您的Facebook页面并从那里复制完整的URL",
  "instagram_url": "此字段必须为有效的Instagram URL，请访问您的Instagram页面并从那里复制完整的URL",
  "threads_url": "此字段必须为有效的Threads URL，请访问您的Threads页面并从那里复制完整的URL",
  "youtube_url": "此字段必须为有效的Youtube URL，请访问您的Youtube页面并从那里复制完整的URL",
  "twitch_url": "此字段必须为有效的Twitch URL，请访问您的Twitch页面并从那里复制完整的URL",
  "tiktok_url": "此字段必须为有效的TikTok URL，请访问您的TikTok页面并从那里复制完整的URL",
  "spotify_url": "此字段必须为有效的Spotify URL，请访问您的Spotify页面并从那里复制完整的URL",
  "google_play_url": "此字段必须为有效的Google Play URL，请访问您在Google Play上的应用程序页面并从那里复制完整的URL",
  "app_store_url": "此字段必须为有效的App Store URL，请访问您在App Store上的应用程序页面并从那里复制完整的URL",
}

export default locale;