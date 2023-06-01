// de.ts
import stats_map from "../share/stats-map/stats-map.de"
import validate from "../share/validate/validate.de";
import countries from "../share/countries/countries.de";
import type_of_content from "../share/type-of-content/type-of-content.de";
import analytics from "../share/analytics/analytics.de";

const locale = {

  "lang": "de",
  "region": null as string | null,

  // @notranslate
  "logo_text": "nuva",

  // @notranslate
  "app_name": "Nuva Studio",

  "station_type_of_content": type_of_content,
  "countries": countries,
  "validate": validate,
  "stats_map": stats_map,
  "analytics": analytics,

  "copy_to_clipboard": "In die Zwischenablage kopieren",
  "show_password": "Passwort anzeigen",
  "hide_password": "Passwort ausblenden",

  "language": {
    "auto": "Automatisch erkennen",
    "en": "English (Englisch)",
    "es": "Español (Spanisch)",
    "es-AR": "Español de Argentina (Argentinisches Spanisch)",
    "pt": "Potuguês (Portugiesisch)",
    "de": "Deutsch",
    "fr": "Française (Französisch)",
    "it": "Italiano (Italienisch)",
    "zh": "简体中文 (Vereinfachtes Chinesisch)",
    "ar": "عربي (Araber)",
  },

  "drawer": {
    "account_selector": {
      "see_all_accounts": "Alle Konten anzeigen",
    },
    "dashboard": "Dashboard",
    "stations": "Stationen",
    "members": "Mitglieder",
    "analytics": "Analysen",
  },

  "limits": {
    "of": "von",
    "stations": "Stationen",
    "listeners": "Zuhörer",
    "transfer": "Transfer",
    "storage": "Speicher",
  },

  "dialogs": {
    "delete": {
      "default_message": "Diese Aktion ist dauerhaft.",
      "cancel": "Abbrechen",
      "delete": "Löschen"
    }
  },

  "station_nav": {
    "dashboard": "Dashboard",
    "profile": "Profil",
    "playlist": "Playlist",
    "broadcast": "Übertragung",
    "settings": "Einstellungen",
  },

  "station_profile": {
    "titles": {
      "logo": "Logo",
      "profile_info": "Profilinformationen",
      "contact_info": "Kontaktinformationen",
      "social": "Soziale Links",
      "apps": "App-Links",
    },
    "validation": {
      "logo_required": "Das Logo ist erforderlich",
    },
    "upload_image": "Bild hochladen",
    "picture_requirement_labels": {
      "format": "Akzeptierte Bildformate:",
      "size": "Mindestbildgröße:",
      "file_size": "Maximale Dateigröße:",
      "square": "Bild muss quadratisch sein",
    },

    "labels": {
      "name": "Name",
      "slogan": "Slogan",
      "description": "Beschreibung",
      "country": "Land",
      "type_of_content": "Art des Inhalts",
      "email": "E-Mail",
      "phone": "Vollständige Telefonnummer",
      "whatsapp": "Vollständige WhatsApp-Nummer",
      "website": "Website-URL",
      "twitter": "Twitter-URL",
      "facebook": "Facebook-URL",
      "instagram": "Instagram-URL",
      "youtube": "Youtube-URL",
      "twitch": "Twitch-URL",
      "google_play": "Google Play-URL",
      "app_store": "App Store-URL"
    }
  },

  "plan_selector": {
    "price": {
      "per_month": "pro Monat",
      "$_n_per_month": "$ @n / Monat",
    },

    "unlimited": "Unbegrenzt",

    "trial": {
      "30_day": "30 Tage",
      "free_trial": "Kostenlose Testversion",
      "tooltip": "Sie werden erst nach Ende Ihrer Testversion in Rechnung gestellt, und Sie können jederzeit kündigen"
    },

    "features": {
      "station": "Station",
      "stations": "Stationen",
      "listeners": "Zuhörer",
      "transfer": "Bandbreite",
      "storage": "Speicher",
      "staff": "Mitarbeiterbenutzer",
      "auto_dj": "Auto DJ",
      "stats": "Erweiterte Statistiken",
      "android_app": "Android-App",
    },

    "tooltips": {
      "one_station": "Mit diesem Plan können Sie nur eine Station erstellen",
      "n_stations": "Bis zu @n verschiedene Stationen",
      "listeners": "Bis zu @n gleichzeitige Zuhörer",
      "transfer": "@tb TB monatlicher Transfer ermöglichen etwa @hours Hörstunden",
      "storage": "@gb GB Speicherplatz für Musik oder alte Episoden",
      "staff": "Fügen Sie alle gewünschten Mitarbeiterbenutzer hinzu",
      "auto_dj": "Senden Sie aus einer Playlist, wenn Sie nicht online sind",
      "stats": "Erweiterte Live- und historische Statistiken, sehen Sie, wer Ihre Stationen hört",
      "android_app": "Eine Android-Anwendung, die auf Ihre Stationen zugeschnitten ist und weltweit über Google Play verfügbar ist",

    }
  },

  "pages": {

    "error": {
      "retry": "Wiederholen",
      "home": "Bring mich nach Hause",
      "default_message": "Ein Fehler ist aufgetreten",
      "offline": {
        "head": {
          "title": "Offline",
        },
        "title": "Es scheint, dass Sie offline sind",
        "text": "Sie benötigen Internetzugang, um @app_name zu verwenden",
      }
    },

    "login": {
      "head": {
        "title": "Anmelden",
      },
      "title": "Anmelden",
      "fields": {
        "email": "E-Mail",
        "password": "Passwort",
      },
      "links": {
        "forgot": "Passwort vergessen?",
        "new_user": "Neuer Benutzer?",
        "sign_up": "Registrieren",
      },
      "submit": "Anmelden"
    },

    "recover": {
      "head": {
        "title": "Konto wiederherstellen",
      },
      "title": "Wiederherstellen",
      "comment": "Wir senden Ihnen eine E-Mail, damit Sie den Zugriff wiederherstellen können",
      "sent_message_html": "Wir haben Ihnen eine E-Mail an <b>@email</b> mit weiteren Anweisungen gesendet",
      "links": {
        "login": "Zurück zur Anmeldung",
      },
      "submit": "Senden",
    },

    "plans": {
      "head": {
        "title": "Pläne und Preise",
      },
      "title_1": "Live gehen in 3... 2... 1...",
      "title_2": "Starten Sie Ihre Radiostation in weniger als 60 Sekunden.",
      "title_3": "Sie werden erst nach Ende Ihrer Testversion in Rechnung gestellt. UndSie können jederzeit kündigen.",
      "plan_selector": {
        "select_btn_label": "Testversion starten",
      }
    },

    "register": {
      "head": {
        "title": "Registrieren",
      },
      "title": "Testversion starten",
      "plan": {
        "selected_plan": "Ausgewählter Plan",
        "$_n_price_per_month": "$ @n / Monat",
        "limits": {
          "station": "Station",
          "stations": "Stationen",
          "listeners": "Zuhörer",
          "transfer": "Übertragung",
          "storage": "Speicher",
        },
        "links": {
          "plans": "Zurück zu Plänen und Preisen"
        }
      },
      "form": {
        "title": "Erzählen Sie uns von sich",
        "account_name_comment": "Wenn Sie ein Konto für eine Organisation erstellen, können Sie dieses Feld mit dem Namen der Organisation ausfüllen",
        "fields": {
          "first_name": "Ihr Vorname",
          "last_name": "Ihr Nachname",
          "account_name": "Ein Name für Ihr Konto",
          "phone": "Ihre Telefonnummer",
          "email": "Ihre E-Mail",
          "password": "Ihr Passwort",
          "confirm_password": "Bestätigen Sie Ihr Passwort",
        },
        "next": "Weiter",
      },
      "pay": {
        "title": "Zahlungsdetails",
        "message": "Sie werden erst nach Ablauf Ihrer 30-tägigen Testversion in Rechnung gestellt, und Sie können jederzeit kündigen."
      },

      "back": "Gehen Sie zurück zum vorherigen Schritt",

      "verification": {
        "title": "Geben Sie den Bestätigungscode ein",
        "message_html": "Wir haben Ihnen einen Bestätigungscode an <b>@email</b> gesendet",
        "submit": "Absenden",
      },

      "links": {
        "login_comment": "Haben Sie bereits ein Konto?",
        "login_link": "Anmelden",
      }
    },

    "user_recovery": {
      "head_page_title": {
        "expired": "Link ist abgelaufen",
        "used": "Link bereits verwendet",
        "not_found": "Link nicht gefunden",
        "ok": "Setzen Sie Ihr Passwort zurück",
      },
      "fields": {
        "email": "E-Mail",
        "password": "Neues Passwort",
        "confirm_password": "Passwort bestätigen",
      },
      "error": {
        "used_message_html": "Der Link, den Sie zum Aufrufen dieser Seite verwendet haben, ist abgelaufen.<br /> Erstellen Sie einen neuen Link von der @user_recovery_page",
        "expired_message_html": "Der Link, den Sie zum Aufrufen dieser Seite verwendet haben, ist abgelaufen.<br /> Erstellen Sie einen neuen Link von der @user_recovery_page",
        "not_found_message_html": "Der Link, den Sie zum Aufrufen dieser Seite verwendet haben, existiert nicht mehr.<br /> Erstellen Sie einen neuen Link von der @user_recovery_page",
        "user_recovery_page": "Benutzer-Wiederherstellungsseite",
      },
      "submit": "Senden",
      "notifier": {
        "password_updated": "Passwort aktualisiert",
      }
    },

    "accounts": {
      "head": {
        "title": "Konten",
      },
      "title": "Wählen Sie ein Konto",
      "create_new_account": "ein neues Konto erstellen",
      "or": "oder",
      "no_items_message_html": "Sie haben noch kein Broadcaster-Konto.<br/>Um mit der Übertragung zu beginnen, melden Sie sich für ein Broadcaster-Konto an.",
      "no_items_create": "Mein Broadcaster-Konto erstellen",
    },

    "accounts/create_account": {
      "head": {
        "title": "Wählen Sie einen Plan",
      },
      "title": "Wählen Sie einen Plan für Ihr neues Konto",
      "select": "Auswählen",
    },

    "accounts/create_account/plan": {
      "head": {
        "title": "Erstellen Sie ein Broadcaster-Konto",
      },
      "title": "Erstellen Sie ein Broadcaster-Konto",
      "plan": {
        "title": "Ausgewählter Plan",
        "$_n_per_month": "$ @n / Monat",
        "station": "Station",
        "stations": "Stationen",
        "listeners": "Zuhörer",
        "transfer": "Bandbreite",
        "storage": "Speicher",
        "back": "Zurück zu Plänen und Preisen",
      },
      "form": {
        "title": "Erzählen Sie uns von dem neuen Konto",
        "fields": {
          "account_name":"Ein Name für Ihr neues Konto",
          "account_name_message": "Wenn Sie ein Konto für eine Organisation erstellen, können Sie dieses Feld mit dem Namen der Organisation ausfüllen"
        },
        "submit": "Erstellen",
        "next": "Weiter",
        "pay": {
          "title": "Zahlungsdetails",
        },
        "back": "Zurück zum vorherigen Schritt",
      }
    },

    "account/dashboard": {
      "edit": {
        "tooltip": "Bearbeiten",
        "dialog": {
          "field_label": "Kontoname",
          "title": "Bearbeiten Sie Ihren Kontonamen",
          "save": "Speichern",
        }
      },

      "stats_map": {
        "all_stations": "Alle Stationen",
      },

      "station_item": {
        "on_air": "ON AIR",
        "off_air": "OFF AIR",
        "playlist": "Playlist",
        "live": "Live",
      }
    },

    "stations": {
      "head": {
        "title": "Stationen",
      },
      "title": "Wählen Sie eine Station",
      "create_new_station": "eine neue Station erstellen",
      "or": "oder",
      "no_items_message_html": "Dieses Konto hat noch keine Stationen.<br />Um mit der Übertragung zu beginnen, erstellen Sie eine neue Station.",
      "no_items_create": "Station erstellen",
    },

    "stations/create_station": {
      "head": {
        "title": "Station erstellen"
      },
      "title": "Station erstellen",
      "submit": "Station erstellen",
      "notifier": {
        "station_created": "Neue Station erstellt",
      }
    },

    "station/dashboard": {
      "on_air": "ON AIR",
      "off_air": "OFF AIR",
      "playlist": "Playlist",
      "live": "Live",
      "preview": "Vorschau",
      "broadcast": "Übertragung",
      "aria_pause": "Pause",
      "aria_play": "Abspielen",
    },

    "station/profile": {
      "head": {
        "title": "Stationsprofil",
      },
      "title": "Profil",
      "submit": "Speichern",
      "notifier": {
        "no_changes": "Keine Änderungen zum Speichern",
        "station_updated": "Station aktualisiert",

      }
    },

    "station/playlist": {
      "head": {
        "title": "Playlist",
      },
      "title": "Playlist",
      "explain_html": "Erstellen Sie eine Playlist mit Musik oder alten Episoden, um Ihre Station 24x7 am Laufen zu halten.<br /> Wenn die Verbindung verloren geht oder Sie nicht senden, übernimmt <b>Playlist</b> automatisch.",
      "upload": "Hochladen",
      "browse": "Durchsuchen",
      "upload_files": "Dateien hochladen",
      "tracks_title": "Titel",
      "track": "Titel",
      "tracks": "Titel",
      "actions": {
        "restart_playlist": "Playlist neu starten",
        "shuffle_playlist": "Playlist mischen",
        "unshuffle_playlist": "Playlist entmischen",
        "drag_to_rearrange": "Zum Anordnen ziehen",
        "edit": "Bearbeiten",
        "delete": "Löschen",
      },
      "columns": {
        "title": "Titel",
        "artist": "Künstler",
        "album": "Album",
        "duration": "Dauer",
      },
      "selection": {
        "one_track_selected": "1 Titel ausgewählt",
        "n_tracks_selected": "@n Titel ausgewählt",
        "delete_selected": "Ausgewählte löschen",
        "select_all": "Alle auswählen",
        "unselect_all": "Alle abwählen",
      },
      "uploading": {
        "success": "Erfolgreich hochgeladen",
        "waiting": "Warten",
        "in_progress": "In Bearbeitung...",
        "retry": "Wiederholen",
        "clear_done": "Erledigte Elemente löschen",
      },
      "dialogs": {
        "delete_track": {
          "title": "Titel @name löschen"
        },
        "delete_tracks": {
          "title": "@n Titel löschen",
        },
        "edit_track": {
          "title": "Titel @name bearbeiten",
          "fields": {
            "title": "Titel",
            "artist": "Künstler",
            "album": "Album",
          },
          "cancel": "Abbrechen",
          "save": "Speichern",
        },
        "shuffle_playlist": {
          "title": "Playlist mischen",
          "message": "Möchten Sie die gesamte Playlist wirklich zufällig mischen?",
          "cancel": "Abbrechen",
          "submit": "Mischen",
        },
        "unshuffle_playlist": {
          "title": "Playlist entmischen",
          "message": "Möchten Sie die gesamte Playlist wirklich entmischen?",
          "cancel": "Abbrechen",
          "submit": "Entmischen",
        },
        "restart_playlist": {
          "title": "Playlist neu starten",
          "message": "Möchten Sie die Playlist wirklich neu starten?",
          "cancel": "Abbrechen",
          "submit": "Neu starten",
        }
      },
      "upload_prevent_unload_message": "Das Verlassen dieser Seite wird ausstehende Uploads abbrechen. Möchten Sie trotzdem gehen?",
      "notifier": {
        "playlist_restarted": "Playlist neu gestartet",
        "track_deleted": "Titel gelöscht",
        "deleting_n_tracks": "@n Titel löschen",
        "n_tracks_deleted": "@n Titel gelöscht",
        "playlist_unshuffled": "Playlist entmischt",
        "playlist_shuffled": "Playlist gemischt",
      }
    },

    "station/broadcast": {
      "head": {
        "title": "Übertragung",
      },
      "title": "Übertragung",
      "icecast_settings": "Icecast-Einstellungen",
      "fields": {
        "address": "Adresse",
        "port": "Port",
        "mountpoint": "Mountpoint",
        "username": "Benutzername",
        "password": "Passwort",
        "encoding": "Kodierung",
      },
      "encoding_or": "oder",
      "password_reset": "Zurücksetzen",
      "links": {
        "title": "Stream-URLs",
        "main": "HAUPT",
      },
      "dialogs": {
        "reset_password": {
          "title": "Mount-Passwort zurücksetzen",
          "message": "Möchten Sie das Mountpoint-Passwort wirklich zurücksetzen?",
          "cancel": "Abbrechen",
          "submit": "Passwort zurücksetzen",
        }
      },
      "notifier": {
        "copied_to_clipboard": "In die Zwischenablage kopiert",
        "mount_password_reset": "Mountpoint-Passwort zurückgesetzt",
      }
    },

    "station/settings": {
      "head": {
        "title": "Einstellungen",
      },
      "title": "Einstellungen",
      "actions": {
        "title": "Aktionen",
        "delete_station": "Station löschen",
      },
      "validate": {
        "station_name": "Der Stationsname stimmt nicht überein",
      },
      "notifier": {
        "station_deleted": "Station gelöscht",
      },
      "dialogs": {
        "delete_station": {
          "title": "Station @name löschen",
          "message_html": "Das Löschen einer Station ist eine dauerhafte Aktion, auf die Daten der Station können Sie nicht mehr zugreifen, seien Sie also sicher, was Sie tun.<br /><br />Wenn Sie die Station @name wirklich löschen möchten, geben Sie den Namen der Station in das folgende Feld ein: <b>@name</b><br />",
          "field_label": "Stationsname",
          "cancel": "Abbrechen",
          "submit": "Löschen",
        }
      }
    },

    "me": {
      "title": "Profil",
      "fields": {
        "email": "Ihre E-Mail",
        "first_name": "Ihr Vorname",
        "last_name": "Ihr Nachname",
        "phone": "Ihre Telefonnummer",
        "new_password": "Neues Passwort",
        "confirm_password": "Passwort bestätigen",
        "language": "Bevorzugte Sprache",
      },
      "submit": {
        "profile": "Speichern",
        "password": "Speichern",
      },
      "change_password": {
        "title": "Passwort ändern",
      },
      "more": {
        "title": "Mehr",
        "connected_devices": "Verbundene Geräte",
      },
      "notifier": {
        "no_changes": "Keine Änderungen zum Speichern",
        "profile_updated": "Profil aktualisiert",
        "password_updated": "Passwort aktualisiert",
      }
    },

    "me/devices": {
      "head": {
        "title": "Geräte",
      },
      "title": "Verbundene Geräte",
      "note": "Das gleiche Gerät kann in dieser Liste mehrmals erscheinen. Geräte werden nach 7 Tagen ohne Nutzung getrennt.",
      "dialogs": {
        "disconnect": {
          "title": "Gerät trennen",
          "message": "Diese Aktion ist dauerhaft",
          "cancel": "Abbrechen",
          "submit": "Trennen",
        },
      },

      "notifier": {
        "device_disconnected": "Gerät getrennt",
      },

      "device": {
        "browser": "Browser",
        "os": "System",
        "ip": "IP",
        "last_used": "Zuletzt verwendet",
        "connected": "Verbunden",
        "unkown": "Unbekannt",
        "tooltips": {
          "disconnect": "Trennen",
        }
      }
    }
  },

  "user_menu": {
    "profile": "Profil",
    "accounts": "Konten",
    "stations": "Stationen",
    "sign_out": "Abmelden",
  }
}

export default locale;
         