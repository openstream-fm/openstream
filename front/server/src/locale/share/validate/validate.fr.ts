/// file: validate.fr.ts
const locale: import("./validate.locale").ValidateLocale = {
  "required": "Ce champ est requis",
  
  "number": "Ce champ doit être un nombre valide",
  "email": "Ce champ doit être une adresse email valide",
  
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

  "email_registered": "Cet email est déjà enregistré",

  "css_color": "Ce champ doit être une couleur CSS valide, par exemple : #ffffff ou rgba(0,0,0,0)",

  "twitter_url": "Ce champ doit être une URL Twitter valide, naviguez jusqu'à votre page Twitter et copiez l'URL complète à partir de là",
  "facebook_url": "Ce champ doit être une URL Facebook valide, naviguez jusqu'à votre page Facebook et copiez l'URL complète à partir de là",
  "instagram_url": "Ce champ doit être une URL Instagram valide, naviguez jusqu'à votre page Instagram et copiez l'URL complète à partir de là",
  "threads_url": "Ce champ doit être une URL Threads valide, naviguez jusqu'à votre page Threads et copiez l'URL complète à partir de là",
  "youtube_url": "Ce champ doit être une URL Youtube valide, naviguez jusqu'à votre page Youtube et copiez l'URL complète à partir de là",
  "twitch_url": "Ce champ doit être une URL Twitch valide, naviguez jusqu'à votre page Twitch et copiez l'URL complète à partir de là",
  "tiktok_url": "Ce champ doit être une URL TikTok valide, naviguez jusqu'à votre page TikTok et copiez l'URL complète à partir de là",
  "spotify_url": "Ce champ doit être une URL Spotify valide, naviguez jusqu'à votre page Spotify et copiez l'URL complète à partir de là",
  "radiocut_url": "Ce champ doit être une URL RadioCut valide, naviguez jusqu'à votre page RadioCut et copiez l'URL complète à partir de là",
  "google_play_url": "Ce champ doit être une URL Google Play valide, naviguez jusqu'à la page de votre application sur Google Play et copiez l'URL complète à partir de là",
  "app_store_url": "Ce champ doit être une URL App Store valide, naviguez jusqu'à la page de votre application sur App Store et copiez l'URL complète à partir de là",
}

export default locale;