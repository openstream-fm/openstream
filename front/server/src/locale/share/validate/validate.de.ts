const locale: import("./validate.locale").ValidateLocale = {

  "required": "Dieses Feld ist erforderlich",

  "number": "Dieses Feld muss eine gültige Zahl sein",
  "email": "Dieses Feld muss eine gültige E-Mail-Adresse sein",

  "min": "Dieses Feld muss @min oder größer sein",
  "max": "Dieses Feld muss @max oder kleiner sein",

  "minlen": "Dieses Feld muss mindestens @minlen Zeichen haben",
  "maxlen": "Dieses Feld darf höchstens @maxlen Zeichen haben",

  "new_password": {
    "minlen": "Das neue Passwort muss mindestens @minlen Zeichen haben",
    "maxlen": "Das neue Passwort darf höchstens @maxlen Zeichen haben"
  },

  "confirmation_password": "Bestätigungspasswort stimmt nicht überein",

  "phone": {
    "tel": "Dieses Feld muss eine gültige internationale Telefonnummer sein",
    "whatsapp": "Dieses Feld muss eine gültige internationale WhatsApp-Nummer sein",
  },

  "url": {
    "valid": "Dieses Feld muss eine gültige URL sein",
    "protocol": "Dieses Feld muss mit http:// oder https:// beginnen",
  },
  
  "email_registered": "Diese E-Mail ist bereits registriert",

  "twitter_url": "Dieses Feld muss eine gültige Twitter-URL sein, navigieren Sie zu Ihrer Twitter-Seite und kopieren Sie die gesamte URL von dort",
  "facebook_url": "Dieses Feld muss eine gültige Facebook-URL sein, navigieren Sie zu Ihrer Facebook-Seite und kopieren Sie die gesamte URL von dort",
  "instagram_url": "Dieses Feld muss eine gültige Instagram-URL sein, navigieren Sie zu Ihrer Instagram-Seite und kopieren Sie die gesamte URL von dort",
  "youtube_url": "Dieses Feld muss eine gültige YouTube-URL sein, navigieren Sie zu Ihrer YouTube-Seite und kopieren Sie die gesamte URL von dort",
  "twitch_url": "Dieses Feld muss eine gültige Twitch-URL sein, navigieren Sie zu Ihrer Twitch-Seite und kopieren Sie die gesamte URL von dort",
  "google_play_url": "Dieses Feld muss eine gültige Google Play-URL sein, navigieren Sie zur Seite Ihrer App bei Google Play und kopieren Sie die gesamte URL von dort",
  "app_store_url": "Dieses Feld muss eine gültige App Store-URL sein, navigieren Sie zur Seite Ihrer App im App Store und kopieren Sie die gesamte URL von dort",
}

export default locale;