const locale: import("./validate.locale").ValidateLocale = {
  "required": "Ce champ est requis",
  
  "number": "Ce champ doit être un nombre valide",
  "email": "Ce champ doit être une adresse e-mail valide",
  
  "min": "Ce champ doit être @min ou plus",
  "max": "Ce champ doit être @max ou moins",
  
  "minlen": "Ce champ doit avoir @minlen caractères ou plus",
  "maxlen": "Ce champ doit avoir @maxlen caractères ou moins",

  "new_password": {
    "minlen": "Le nouveau mot de passe doit avoir @minlen caractères ou plus",
    "maxlen": "Le nouveau mot de passe doit avoir @maxlen caractères ou moins"
  },

  "confirmation_password": "La confirmation du mot de passe ne correspond pas",
  
  "phone": {
    "tel": "Ce champ doit être un numéro de téléphone international valide",
    "whatsapp": "Ce champ doit être un numéro de WhatsApp international valide",
  },

  "url": {
    "valid": "Ce champ doit être une URL valide",
    "protocol": "Ce champ doit commencer par http:// ou https://",
  },

  "email_registered": "Cet e-mail est déjà enregistré",
  
  "twitter_url": "Ce champ doit être une URL Twitter valide, accédez à votre page Twitter et copiez l'URL complète à partir de là",
  "facebook_url": "Ce champ doit être une URL Facebook valide, accédez à votre page Facebook et copiez l'URL complète à partir de là",
  "instagram_url": "Ce champ doit être une URL Instagram valide, accédez à votre page Instagram et copiez l'URL complète à partir de là",
  "youtube_url": "Ce champ doit être une URL Youtube valide, accédez à votre page Youtube et copiez l'URL complète à partir de là",
  "twitch_url": "Ce champ doit être une URL Twitch valide, accédez à votre page Twitch et copiez l'URL complète à partir de là",
  "google_play_url": "Ce champ doit être une URL Google Play valide, accédez à la page de votre application sur Google Play et copiez l'URL complète à partir de là",
  "app_store_url": "Ce champ doit être une URL App Store valide, accédez à la page de votre application sur l'App Store et copiez l'URL complète à partir de là",
}

export default locale;