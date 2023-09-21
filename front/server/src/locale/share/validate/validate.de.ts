/// file: validate.de.ts
const locale: import("./validate.locale").ValidateLocale = {
  "required": "Dieses Feld ist erforderlich",
  
  "number": "Dieses Feld muss eine gültige Nummer sein",
  "email": "Dieses Feld muss eine gültige E-Mail-Adresse sein",
  
  "min": "Dieses Feld muss @min oder mehr sein",
  "max": "Dieses Feld muss @max oder weniger sein",
  
  "minlen": "Dieses Feld muss @minlen Zeichen oder mehr haben",
  "maxlen": "Dieses Feld muss @maxlen Zeichen oder weniger haben",

  "new_password": {
    "minlen": "Das neue Passwort muss @minlen Zeichen oder mehr haben",
    "maxlen": "Das neue Passwort muss @maxlen Zeichen oder weniger haben"
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

  "css_color": "Dieses Feld muss eine gültige CSS-Farbe sein, zum Beispiel: #ffffff oder rgba(0,0,0,0)",

  "twitter_url": "Dieses Feld muss eine gültige Twitter-URL sein, navigieren Sie zu Ihrer Twitter-Seite und kopieren Sie die vollständige URL von dort",
  "facebook_url": "Dieses Feld muss eine gültige Facebook-URL sein, navigieren Sie zu Ihrer Facebook-Seite und kopieren Sie die vollständige URL von dort",
  "instagram_url": "Dieses Feld muss eine gültige Instagram-URL sein, navigieren Sie zu Ihrer Instagram-Seite und kopieren Sie die vollständige URL von dort",
  "threads_url": "Dieses Feld muss eine gültige Threads-URL sein, navigieren Sie zu Ihrer Threads-Seite und kopieren Sie die vollständige URL von dort",
  "youtube_url": "Dieses Feld muss eine gültige YouTube-URL sein, navigieren Sie zu Ihrer YouTube-Seite und kopieren Sie die vollständige URL von dort",
  "twitch_url": "Dieses Feld muss eine gültige Twitch-URL sein, navigieren Sie zu Ihrer Twitch-Seite und kopieren Sie die vollständige URL von dort",
  "tiktok_url": "Dieses Feld muss eine gültige TikTok-URL sein, navigieren Sie zu Ihrer TikTok-Seite und kopieren Sie die vollständige URL von dort",
  "spotify_url": "Dieses Feld muss eine gültige Spotify-URL sein, navigieren Sie zu Ihrer Spotify-Seite und kopieren Sie die vollständige URL von dort",
  "radiocut_url": "Dieses Feld muss eine gültige RadioCut-URL sein, navigieren Sie zu Ihrer RadioCut-Seite und kopieren Sie die vollständige URL von dort",
  "google_play_url": "Dieses Feld muss eine gültige Google Play-URL sein, navigieren Sie zu Ihrer App-Seite auf Google Play und kopieren Sie die vollständige URL von dort",
  "app_store_url": "Dieses Feld muss eine gültige App Store-URL sein, navigieren Sie zu Ihrer App-Seite im App Store und kopieren Sie die vollständige URL von dort",
}

export default locale;