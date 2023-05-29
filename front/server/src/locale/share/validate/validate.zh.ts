/// file: validate.zh.ts
const locale: import("./validate.locale").ValidateLocale = {
  "required": "此字段为必填项",
  
  "number": "此字段必须为有效数字",
  "email": "此字段必须为有效的电子邮件地址",
  
  "min": "此字段必须为@min或更大",
  "max": "此字段必须为@max或更小",
  
  "minlen": "此字段必须包含@minlen个字符或更多",
  "maxlen": "此字段必须包含@maxlen个字符或更少",

  "new_password": {
    "minlen": "新密码必须包含@minlen个字符或更多",
    "maxlen": "新密码必须包含@maxlen个字符或更少"
  },

  "confirmation_password": "密码确认不匹配",
  
  "phone": {
    "tel": "此字段必须为有效的国际电话号码",
    "whatsapp": "此字段必须为有效的国际WhatsApp号码",
  },

  "url": {
    "valid": "此字段必须为有效的URL",
    "protocol": "此字段必须以http://或https://开头",
  },

  "email_registered": "电子邮件已经注册",
  
  "twitter_url": "此字段必须为有效的Twitter URL，导航至您的Twitter页面并从那里复制整个URL",
  "facebook_url": "此字段必须为有效的Facebook URL，导航至您的Facebook页面并从那里复制整个URL",
  "instagram_url": "此字段必须为有效的Instagram URL，导航至您的Instagram页面并从那里复制整个URL",
  "youtube_url": "此字段必须为有效的Youtube URL，导航至您的Youtube页面并从那里复制整个URL",
  "twitch_url": "此字段必须为有效的Twitch URL，导航至您的Twitch页面并从那里复制整个URL",
  "google_play_url": "此字段必须为有效的Google Play URL，导航至您的应用在Google Play的页面并从那里复制整个URL",
  "app_store_url": "此字段必须为有效的App Store URL，导航至您的应用在App Store的页面并从那里复制整个URL",
}

export default locale;