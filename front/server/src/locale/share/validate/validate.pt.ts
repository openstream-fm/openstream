/// file: validate.pt.ts
const locale: import("./validate.locale").ValidateLocale = {
  "required": "Este campo é obrigatório",
  
  "number": "Este campo deve ser um número válido",
  "email": "Este campo deve ser um endereço de email válido",
  
  "min": "Este campo deve ser @min ou mais",
  "max": "Este campo deve ser @max ou menos",
  
  "minlen": "Este campo deve ter @minlen caracteres ou mais",
  "maxlen": "Este campo deve ter @maxlen caracteres ou menos",

  "new_password": {
    "minlen": "A nova senha deve ter @minlen caracteres ou mais",
    "maxlen": "A nova senha deve ter @maxlen caracteres ou menos"
  },

  "confirmation_password": "A confirmação da senha não coincide",
  
  "phone": {
    "tel": "Este campo deve ser um número de telefone internacional válido",
    "whatsapp": "Este campo deve ser um número de WhatsApp internacional válido",
  },

  "url": {
    "valid": "Este campo deve ser uma URL válida",
    "protocol": "Este campo deve começar com http:// ou https://",
  },

  "email_registered": "Este email já está registrado",

  "css_color": "Este campo deve ser uma cor CSS válida, por exemplo: #ffffff ou rgba(0,0,0,0)",

  "twitter_url": "Este campo deve ser uma URL do Twitter válida, navegue até a sua página do Twitter e copie a URL completa de lá",
  "facebook_url": "Este campo deve ser uma URL do Facebook válida, navegue até a sua página do Facebook e copie a URL completa de lá",
  "instagram_url": "Este campo deve ser uma URL do Instagram válida, navegue até a sua página do Instagram e copie a URL completa de lá",
  "threads_url": "Este campo deve ser uma URL do Threads válida, navegue até a sua página do Threads e copie a URL completa de lá",
  "youtube_url": "Este campo deve ser uma URL do Youtube válida, navegue até a sua página do Youtube e copie a URL completa de lá",
  "twitch_url": "Este campo deve ser uma URL do Twitch válida, navegue até a sua página do Twitch e copie a URL completa de lá",
  "tiktok_url": "Este campo deve ser uma URL do TikTok válida, navegue até a sua página do TikTok e copie a URL completa de lá",
  "spotify_url": "Este campo deve ser uma URL do Spotify válida, navegue até a sua página do Spotify e copie a URL completa de lá",
  "google_play_url": "Este campo deve ser uma URL do Google Play válida, navegue até a página do seu aplicativo no Google Play e copie a URL completa de lá",
  "app_store_url": "Este campo deve ser uma URL da App Store válida, navegue até a página do seu aplicativo na App Store e copie a URL completa de lá",
}

export default locale;