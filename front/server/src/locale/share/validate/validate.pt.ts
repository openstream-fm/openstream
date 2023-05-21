// file: validate.pt.ts
const locale: import("./validate.locale").ValidateLocale = {
  "required": "Este campo é obrigatório",
  
  "number": "Este campo deve ser um número válido",
  "email": "Este campo deve ser um endereço de email válido",
  
  "min": "Este campo deve ser @min ou mais",
  "max": "Este campo deve ser @max ou menos",
  
  "minlen": "Este campo deve ter @minlen caracteres ou mais",
  "maxlen": "Este campo deve ter @maxlen caracteres ou menos",

  "new_password": {
    "minlen": "Nova senha deve ter @minlen caracteres ou mais",
    "maxlen": "Nova senha deve ter @maxlen caracteres ou menos"
  },

  "confirmation_password": "A confirmação de senha não coincide",
  
  "phone": {
    "tel": "Este campo deve ser um número de telefone internacional válido",
    "whatsapp": "Este campo deve ser um número de WhatsApp internacional válido",
  },

  "url": {
    "valid": "Este campo deve ser uma URL válida",
    "protocol": "Este campo deve começar com http:// ou https://",
  },

  "twitter_url": "Este campo deve ser uma URL válida do Twitter, navegue até sua página no Twitter e copie a URL completa de lá",
  "facebook_url": "Este campo deve ser uma URL válida do Facebook, navegue até sua página no Facebook e copie a URL completa de lá",
  "instagram_url": "Este campo deve ser uma URL válida do Instagram, navegue até sua página no Instagram e copie a URL completa de lá",
  "youtube_url": "Este campo deve ser uma URL válida do Youtube, navegue até sua página no Youtube e copie a URL completa de lá",
  "twitch_url": "Este campo deve ser uma URL válida do Twitch, navegue até sua página no Twitch e copie a URL completa de lá",
  "google_play_url": "Este campo deve ser uma URL válida do Google Play, navegue até a página do seu aplicativo no Google Play e copie a URL completa de lá",
  "app_store_url": "Este campo deve ser uma URL válida da App Store, navegue até a página do seu aplicativo na App Store e copie a URL completa de lá",
}

export default locale;