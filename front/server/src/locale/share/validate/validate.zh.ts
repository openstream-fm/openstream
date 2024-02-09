/// file: validate.zh.ts
const locale: import("./validate.locale.js").ValidateLocale = {
  "required": "此字段是必需的",
  
  "number": "此字段必须是有效的数字",
  "email": "此字段必须是有效的电子邮件地址",
  
  "min": "此字段必须是 @min 或更多",
  "max": "此字段必须是 @max 或更少",
  
  "minlen": "此字段必须有 @minlen 个字符或更多",
  "maxlen": "此字段必须有 @maxlen 个字符或更少",

  "new_password": {
    "minlen": "新密码必须有 @minlen 个字符或更多",
    "maxlen": "新密码必须有 @maxlen 个字符或更少"
  },

  "confirmation_password": "密码确认不匹配",
  
  "phone": {
    "tel": "此字段必须是有效的国际电话号码",
    "whatsapp": "此字段必须是有效的WhatsApp国际号码",
  },

  "url": {
    "valid": "此字段必须是有效的URL",
    "protocol": "此字段必须以 http:// 或 https:// 开头",
  },

  "email_registered": "此电子邮件已注册",

  "css_color": "此字段必须是有效的CSS颜色，例如：#ffffff 或 rgba(0,0,0,0)",

  "twitter_url": "此字段必须是有效的Twitter URL，导航到你的Twitter页面并从那里复制完整的URL",
  "facebook_url": "此字段必须是有效的Facebook URL，导航到你的Facebook页面并从那里复制完整的URL",
  "instagram_url": "此字段必须是有效的Instagram URL，导航到你的Instagram页面并从那里复制完整的URL",
  "threads_url": "此字段必须是有效的Threads URL，导航到你的Threads页面并从那里复制完整的URL",
  "youtube_url": "此字段必须是有效的Youtube URL，导航到你的Youtube页面并从那里复制完整的URL",
  "twitch_url": "此字段必须是有效的Twitch URL，导航到你的Twitch页面并从那里复制完整的URL",
  "tiktok_url": "此字段必须是有效的TikTok URL，导航到你的TikTok页面并从那里复制完整的URL",
  "spotify_url": "此字段必须是有效的Spotify URL，导航到你的Spotify页面并从那里复制完整的URL",
  "radiocut_url": "此字段必须是有效的RadioCut URL，导航到你的RadioCut页面并从那里复制完整的URL",
  "google_play_url": "此字段必须是有效的Google Play URL，导航到你在Google Play的应用页面并从那里复制完整的URL",
  "app_store_url": "此字段必须是有效的App Store URL，导航到你在App Store的应用页面并从那里复制完整的URL",
}

export default locale;