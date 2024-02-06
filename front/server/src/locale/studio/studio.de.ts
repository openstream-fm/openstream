/// file: studio.de.ts
import stats_map from "../share/stats-map/stats-map.de";
import validate from "../share/validate/validate.de";
import countries from "../share/countries/countries.de";
import langs from "../share/langs/langs.de";
import type_of_content from "../share/type-of-content/type-of-content.de";
import analytics from "../share/analytics/analytics.de";
import payments from "../share/payments/payments.de";
import station_profile from "../share/station-profile/station-profile.de";
import misc from "../misc/misc.de";
import language from "../share/language/language.de";


const locale: import("./studio.locale").StudioLocale = {

  "lang": "de",
  "region": null,

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "brand_name": "Openstream",

  // @notranslate
  "app_name": "Openstream Studio",
  
  type_of_content,
  countries,
  langs,
  validate,
  stats_map,
  analytics,
  payments,
  misc,
  station_profile,
  language,

  "copy_to_clipboard": "In die Zwischenablage kopieren",
  "show_password": "Passwort anzeigen",
  "hide_password": "Passwort verstecken",

  "prevent_unload_message":  "Wenn Sie diese Seite verlassen, gehen die von Ihnen vorgenommenen Änderungen verloren. Möchten Sie die Seite trotzdem verlassen?",

  "drawer": {
    "account_selector": {
      "see_all_accounts": "Alle Konten anzeigen",
    },
    "dashboard": "Dashboard",
    "stations": "Stationen",
    "members": "Mitglieder",
    "analytics": "Analytik",
  },

  "limits": {
    "of": "von",
    "stations": "Stationen",
    "listeners": "Zuhörer",
    "transfer": "Übertragung",
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

  "plan_selector": {
    "price": {
      "per_month": "pro Monat",
      "n_per_month": "@n / Monat",
    },

    "unlimited": "Unbegrenzt",

    "trial": {
      "30_day": "30 Tage",
      "free_trial": "kostenlose Probe",
      "tooltip": "Es wird nichts berechnet, bis Ihr Test abgeschlossen ist, und Sie können jederzeit kündigen"
    },

    "features": {
      "station": "Station",
      "stations": "Stationen",
      "listeners": "Zuhörer",
      "transfer": "Übertragung",
      "storage": "Speicher",
      "staff": "Benutzer",
      "auto_dj": "Auto DJ",
      "stats": "Erweiterte Statistiken",
      "android_app": "Android-App",
    },

    "tooltips": {
      "one_station": "Sie können nur eine Station in diesem Plan erstellen",
      "n_stations": "Bis zu @n verschiedene Stationen",
      "listeners": "Bis zu @n gleichzeitige Zuhörer",
      "transfer": "Mit @tb TB monatlicher Übertragung können Sie etwa @hours Stunden Audio übertragen",
      "storage": "@gb GB Speicherplatz für Musik oder alte Episoden",
      "staff": "Sie können Benutzer für Ihr gesamtes Team ohne Limit hinzufügen",
      "auto_dj": "Senden Sie von einer Playlist, wenn Sie offline sind",
      "stats": "Historische und Live-Statistiken, wissen Sie, wer Ihre Stationen hört",
      "android_app": "Eine Android-App mit Ihrer Marke und Ihren Stationen, weltweit verfügbar über Google Play",
    }
  },

  "pages": {

    "error": {
      "retry": "Erneut versuchen",
      "home": "Zur Startseite gehen",
      "default_message": "Ein Fehler ist aufgetreten",
      "offline": {
        "head": {
          "title": "Offline",
        },
        "title": "Es sieht so aus, als wären Sie offline",
        "text": "Für die Nutzung von @app_name ist eine Internetverbindung erforderlich",
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
      "comment": "Wir senden Ihnen eine E-Mail, um Ihr Konto wiederherzustellen",
      "sent_message_html": "Wir haben eine E-Mail an <b>@email</b> mit Anweisungen zum Fortfahren gesendet",
      "links": {
        "login": "Zurück zur Anmeldung",
      },
      "submit": "Senden",
    },

    "plans": {
      "head": {
        "title": "Pläne und Preise",
      },
      "title_1": "Live in 3... 2... 1...",
      "title_2": "Starten Sie Ihren Sender in weniger als 60 Sekunden.",
      "title_3": "Es wird nichts berechnet, bis Ihr Test abgeschlossen ist. Und Sie können jederzeit kündigen.",
      "plan_selector": {
        "select_btn_label": "Test starten",
      }
    },

    "register": {
      "head": {
        "title": "Registrieren",
      },
      "title": "Starten Sie Ihre kostenlose Testversion",
      "plan": {
        "selected_plan": "Ausgewählter Plan",
        "n_per_month": "@n / Monat",
        "limits": {
          "station": "Station",
          "stations": "Stationen",
          "listeners": "Zuhörer",
          "transfer": "Übertragung",
          "storage": "Speicher",
        },
        "links": {
          "plans": "Zurück zu Pläne und Preise"
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
        "message": "Es wird nichts berechnet, bis Ihre kostenlose 30-Tage-Testversion abgeschlossen ist und Sie können jederzeit kündigen."
      },

      "back": "Zurück zum vorherigen Schritt",

      "verification": {
        "title": "Geben Sie den Bestätigungscode ein",
        "message_html": "Wir haben einen Bestätigungscode an <b>@email</b> gesendet",
        "submit": "Senden",
      },
      "links": {
        "login_comment": "Haben Sie bereits ein Konto?",
        "login_link": "Anmelden",
      }
    },

    "user_recovery": {
      "head_page_title": {
        "expired": "Der Link ist abgelaufen",
        "used": "Der Link wurde bereits verwendet",
        "not_found": "Link nicht gefunden",
        "ok": "Setzen Sie Ihr Passwort zurück",
      },
      "fields": {
        "email": "E-Mail",
        "password": "Neues Passwort",
        "confirm_password": "Passwort bestätigen",
      },
      "error": {
        "used_message_html": "Der Link, den Sie zum Zugriff auf diese Seite verwendet haben, wurde bereits verwendet.<br /> Erstellen Sie einen neuen Link von der @user_recovery_page",
        "expired_message_html": "Der Link, den Sie zum Zugriff auf diese Seite verwendet haben, ist abgelaufen.<br /> Erstellen Sie einen neuen Link von der @user_recovery_page",
        "not_found_message_html": "Der Link, den Sie zum Zugriff auf diese Seite verwendet haben, existiert nicht.<br /> Erstellen Sie einen neuen Link von der @user_recovery_page",
        "user_recovery_page": "Wiederherstellungsseite",
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
      "no_items_message_html": "Sie haben noch kein Senderkonto.<br/>Um mit der Übertragung zu beginnen, erstellen Sie Ihr Senderkonto.",
      "no_items_create": "Mein Senderkonto erstellen",
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
        "n_per_month": "@n / Monat",
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

    "account/analytics": {
      "head": {
        "title": "Analysen",
      },
      "title": "Analysen",
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
        "current_password": "Aktuelles Passwort",
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
    },

    "account/members": {
      "head": {
        "title": "Mitglieder"
      },
      "title": "Mitglieder",

      "no_owner_message_p1": "Dieser Bereich ist nur für Kontoadministratoren verfügbar.",
      "no_owner_message_p2": "Wenden Sie sich an die Kontoadministratoren, wenn Sie Personen einladen möchten, um an diesem Konto teilzunehmen.",

      "Pending_invitations": "Ausstehende Einladungen",
      "no_pending_invitations_message": "Keine ausstehenden Einladungen",
      "invite_btn_text": "Personen einladen",

      "validate": {
        "user_account_exists": "Der Benutzer mit der E-Mail-Adresse @email ist bereits Teil dieses Kontos",
      },

      "notifier": {
        "invitation_sent": "Einladung gesendet",
        "member_access_revoked": "Mitgliederzugriff widerrufen",
        "member_role_changed": "Mitgliederzugriffsrolle aktualisiert",
      },

      "actions": {
        "set_role_to": "Rolle auf @role setzen",
        "revoke_access": "Zugriff widerrufen",
        "delete": "Löschen",
      },

      "dialogs": {
        "invite": {
          "title": "Laden Sie Personen ein, an diesem Konto mit der Rolle @role teilzunehmen",
          "submit": "Einladen",
          "Email": "E-Mail",
        }
      }
    },

    "email_invitation": {
      "head_page_title": {
        "not_found": "Einladung nicht gefunden",
        "expired": "Einladung abgelaufen",
        "accepted": "Einladung bereits angenommen",
        "rejected": "Einladung bereits abgelehnt",
        "ok": "Ausstehende Einladung",
      },

      "error_message": {
        "not_found": "Der Link, den Sie zum Aufrufen dieser Seite verwendet haben, existiert nicht mehr oder wurde gelöscht",
        "expired": "Die Einladung ist abgelaufen, wenden Sie sich an die Kontoadministratoren, damit sie Ihnen eine neue Einladung senden",
        "accepted": "Die Einladung wurde bereits angenommen",
        "rejected": "Die Einladung wurde bereits abgelehnt, wenn dies ein Fehler war, wenden Sie sich an die Kontoadministratoren, damit sie Ihnen eine neue Einladung senden",
      },

      "description": {
        "with_sender_name_html": "<b>@sender</b> lädt Sie ein, sich <b>@account</b> bei Openstream anzuschließen.",
        "without_sender_name_html": "Sie wurden eingeladen, sich <b>@account</b> bei Openstream anzuschließen",
      },

      "login_as_btn_html": "Melden Sie sich als <b>@email</b> an, um die Einladung anzunehmen",

      "form": {
        "fields": {
          "first_name": "Dein Vorname",
          "last_name": "Dein Nachname",
          "email": "Deine E-Mail",
          "password": "Passwort",
          "confirm_password": "Passwort bestätigen",
        },
        "pre_message_html": "Um die Einladung <b>anzunehmen</b>, füllen Sie das Formular aus.",
        "title": "Registrieren",
        "submit": "Absenden",
      },

      "notifier": {
        "accept_error": "Es gab einen Fehler beim Annehmen der Einladung: @error"
      }
    },

    "me/invitations": {
      "head": {
        "title": "Ausstehende Einladungen",
      },
      "title": "Ausstehende Einladungen",

      "no_items_message": "Sie haben keine ausstehenden Einladungen",

      "notifier": {
        "accept_error": "Es gab einen Fehler beim Annehmen der Einladung: @error",
        "accepted": "Einladung angenommen",
        "rejected": "Einladung abgelehnt",
      },

      "actions": {
        "reject": "Ablehnen",
        "accept": "Annehmen",
      },

      "item_message_with_sender_html": "<b>@sender</b> lädt Sie ein, sich <b>@account</b> anzuschließen",
      "item_message_without_sender_html": "Sie wurden eingeladen, sich <b>@account</b> anzuschließen",

      "dialogs": {
        "reject": {
          "title": "Einladung ablehnen",
          "message": "Sind Sie sicher, dass Sie die Einladung ablehnen möchten?",
          "cancel": "Abbrechen",
          "reject": "Einladung ablehnen",
        }
      }
    }
  },

  "user_menu": {
    "profile": "Profil",
    "invitations": "Einladungen",
    "accounts": "Konten",
    "stations": "Stationen",
    "sign_out": "Abmelden",
  }
}

export default locale;