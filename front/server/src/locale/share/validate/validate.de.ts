/// file: validate.de.ts
const locale: import("./validate.locale").ValidateLocale = {
  "required": "Dieses Feld ist erforderlich",
  
  "number": "Dieses Feld muss eine gültige Nummer sein",
  "email": "Dieses Feld muss eine gültige E-Mail-Adresse sein",
  
  "min": "Dieses Feld muss @min oder mehr sein",
  "max": "Dieses Feld muss @max oder weniger sein",
  
  "minlen": "Dieses Feld muss @minlen oder mehr Zeichen haben",
  "maxlen": "Dieses Feld muss @maxlen oder weniger Zeichen haben",

  "new_password": {
    "minlen": "Das neue Passwort muss @minlen oder mehr Zeichen haben",
    "maxlen": "Das neue Passwort muss @maxlen oder weniger Zeichen haben"
  },

  "confirmation_password": "Die Passwortbestätigung stimmt nicht überein",
  
  "phone": {
    "tel": "Dieses Feld muss eine gültige internationale Telefonnummer sein",
    "whatsapp": "Dieses Feld muss eine gültige internationale WhatsApp-Nummer sein",
  },

  "url": {
    "valid": "Dieses Feld muss eine gültige URL sein",
    "protocol": "Dieses Feld muss mit http:// oder https:// beginnen",
  },

  "email_registered": "Diese E-Mail ist bereits registriert",

  "twitter_url": "Dieses Feld muss eine gültige Twitter-URL sein. Gehe zu deiner Twitter-Seite und kopiere die vollständige URL von dort",
  "facebook_url": "Dieses Feld muss eine gültige Facebook-URL sein. Gehe zu deiner Facebook-Seite und kopiere die vollständige URL von dort",
  "instagram_url": "Dieses Feld muss eine gültige Instagram-URL sein. Gehe zu deiner Instagram-Seite und kopiere die vollständige URL von dort",
  "threads_url": "Dieses Feld muss eine gültige Threads-URL sein. Gehe zu deiner Threads-Seite und kopiere die vollständige URL von dort",
  "youtube_url": "Dieses Feld muss eine gültige YouTube-URL sein. Gehe zu deiner YouTube-Seite und kopiere die vollständige URL von dort",
  "twitch_url": "Dieses Feld muss eine gültige Twitch-URL sein. Gehe zu deiner Twitch-Seite und kopiere die vollständige URL von dort",
  "tiktok_url": "Dieses Feld muss eine gültige TikTok-URL sein. Gehe zu deiner TikTok-Seite und kopiere die vollständige URL von dort",
  "spotify_url": "Dieses Feld muss eine gültige Spotify-URL sein. Gehe zu deiner Spotify-Seite und kopiere die vollständige URL von dort",
  "google_play_url": "Dieses Feld muss eine gültige Google Play-URL sein. Gehe zur Seite deiner App im Google Play Store und kopiere die vollständige URL von dort",
  "app_store_url": "Dieses Feld muss eine gültige App Store-URL sein. Gehe zur Seite deiner App im App Store und kopiere die vollständige URL von dort",
}

export default locale;