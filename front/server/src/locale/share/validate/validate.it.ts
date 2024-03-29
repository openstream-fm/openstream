/// file: validate.it.ts
const locale: import("./validate.locale.js").ValidateLocale = {
  "required": "Questo campo è obbligatorio",
  
  "number": "Questo campo deve essere un numero valido",
  "email": "Questo campo deve essere un indirizzo email valido",
  
  "min": "Questo campo deve essere @min o più",
  "max": "Questo campo deve essere @max o meno",
  
  "minlen": "Questo campo deve avere @minlen caratteri o più",
  "maxlen": "Questo campo deve avere @maxlen caratteri o meno",

  "new_password": {
    "minlen": "La nuova password deve avere @minlen caratteri o più",
    "maxlen": "La nuova password deve avere @maxlen caratteri o meno"
  },

  "confirmation_password": "La conferma della password non corrisponde",
  
  "phone": {
    "tel": "Questo campo deve essere un numero di telefono internazionale valido",
    "whatsapp": "Questo campo deve essere un numero di WhatsApp internazionale valido",
  },

  "url": {
    "valid": "Questo campo deve essere un URL valido",
    "protocol": "Questo campo deve iniziare con http:// o https://",
  },

  "email_registered": "Questo email è già registrato",

  "css_color": "Questo campo deve essere un colore CSS valido, ad esempio: #ffffff o rgba(0,0,0,0)",

  "twitter_url": "Questo campo deve essere un URL di Twitter valido, vai alla tua pagina Twitter e copia l'URL completo da lì",
  "facebook_url": "Questo campo deve essere un URL di Facebook valido, vai alla tua pagina Facebook e copia l'URL completo da lì",
  "instagram_url": "Questo campo deve essere un URL di Instagram valido, vai alla tua pagina Instagram e copia l'URL completo da lì",
  "threads_url": "Questo campo deve essere un URL di Threads valido, vai alla tua pagina Threads e copia l'URL completo da lì",
  "youtube_url": "Questo campo deve essere un URL di Youtube valido, vai alla tua pagina Youtube e copia l'URL completo da lì",
  "twitch_url": "Questo campo deve essere un URL di Twitch valido, vai alla tua pagina Twitch e copia l'URL completo da lì",
  "tiktok_url": "Questo campo deve essere un URL di TikTok valido, vai alla tua pagina TikTok e copia l'URL completo da lì",
  "spotify_url": "Questo campo deve essere un URL di Spotify valido, vai alla tua pagina Spotify e copia l'URL completo da lì",
  "radiocut_url": "Questo campo deve essere un URL di RadioCut valido, vai alla tua pagina RadioCut e copia l'URL completo da lì",
  "google_play_url": "Questo campo deve essere un URL di Google Play valido, vai alla pagina della tua app su Google Play e copia l'URL completo da lì",
  "app_store_url": "Questo campo deve essere un URL di App Store valido, vai alla pagina della tua app su App Store e copia l'URL completo da lì",
}

export default locale;