const locale: import("./validate.locale").ValidateLocale = {
  "required": "Este campo es requerido",
  
  "number": "Este campo debe ser un número válido",
  "email": "Este campo debe ser una dirección de email válida",
  
  "min": "Este campo debe ser @min or más",
  "max": "Este campo debe ser @max or menos",
  
  "minlen": "Este campo debe tener @minlen caractéres o más",
  "maxlen": "Este campo debe tener @maxlen caractéres o menos",

  "new_password": {
    "minlen": "La nueva contraseña debe tener @minlen caractéres o más",
    "maxlen": "La nueva contraseña debe tener @maxlen caractéres o menos"
  },

  "confirmation_password": "La confirmación de la contraseña no coincide",
  
  "phone": {
    "tel": "Este campo debe ser un número de teléfono internacional válido",
    "whatsapp": "Este campo debe ser un número de WhatsApp internacional válido",
  },

  "url": {
    "valid": "Este campo debe ser una URL válida",
    "protocol": "Este campo debe empezar por http:// o https://",
  },

  "email_registered": "Este email ya está registrado",

  "css_color": "Este campo debe ser un color CSS válido, por ejemplo: #ffffff o rgba(0,0,0,0)",

  "twitter_url": "Este campo debe ser una URL de Twitter válida, navega hasta tu página de Twitter y copia la URL completa desde allí",
  "facebook_url": "Este campo debe ser una URL de Facebook válida, navega hasta tu página de Facebook y copia la URL completa desde allí",
  "instagram_url": "Este campo debe ser una URL de Instagram válida, navega hasta tu página de Instagram y copia la URL completa desde allí",
  "threads_url": "Este campo debe ser una URL de Threads válida, navega hasta tu página de Threads y copia la URL completa desde allí",
  "youtube_url": "Este campo debe ser una URL de Youtube válida, navega hasta tu página de Youtube y copia la URL completa desde allí",
  "twitch_url": "Este campo debe ser una URL de Twitch válida, navega hasta tu página de Twitch y copia la URL completa desde allí",
  "tiktok_url": "Este campo debe ser una URL de TikTok válida, navega hasta tu página de TikTok y copia la URL completa desde allí",
  "spotify_url": "Este campo debe ser una URL de Spotify válida, navega hasta tu página de Spotify y copia la URL completa desde allí",
  "radiocut_url": "Este campo debe ser una URL de RadioCut válida, navega hasta tu página de RadioCut y copia la URL completa desde allí",
  "google_play_url": "Este campo debe ser una URL de Google Play válida, navega hasta la página de tu aplicación en Google Play y copia la URL completa desde allí",
  "app_store_url": "Este campo debe ser una URL de App Store válida, navega hasta la página de tu aplicación en App Store y copia la URL completa desde allí",
}

export default locale;