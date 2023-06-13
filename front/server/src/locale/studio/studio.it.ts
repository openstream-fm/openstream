/// file: countries.it.ts
import stats_map from "../share/stats-map/stats-map.it";
import validate from "../share/validate/validate.it";
import countries from "../share/countries/countries.it";
import type_of_content from "../share/type-of-content/type-of-content.it";
import analytics from "../share/analytics/analytics.it";
import payments from "../share/payments/payments.it";

const locale: import("./studio.locale").StudioLocale = {

  "lang": "it",
  "region": null,

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "app_name": "Openstream Studio",
  
  "station_type_of_content": type_of_content,
  "countries": countries,
  "validate": validate,
  "stats_map": stats_map,
  "analytics": analytics,
  "payments": payments,

  "language": {
    "auto": "Rilevamento automatico",
    "en": "English (Inglese)",
    "es": "Español (Spagnolo)",
    "es-AR": "Español de Argentina (Spagnolo Argentino)",
    "pt": "Português (Portoghese)",
    "de": "Deutsche (Tedesco)",
    "fr": "Française (Francese)",
    "it": "Italiano",
    "zh": "简体中文 (Cinese Semplificato)",
    "ar": "عربي (Arabo)",
  },

  "copy_to_clipboard": "Copia negli appunti",
  "show_password": "Mostra password",
  "hide_password": "Nascondi password",


  "drawer": {
    "account_selector": {
      "see_all_accounts": "Visualizza tutti gli account",
    },
    "dashboard": "Pannello di controllo",
    "stations": "Stazioni",
    "members": "Membri",
    "analytics": "Analisi",
  },

  "limits": {
    "of": "di",
    "stations": "Stazioni",
    "listeners": "Ascoltatori",
    "transfer": "Trasferimento",
    "storage": "Archiviazione",
  },

  "dialogs": {
    "delete": {
      "default_message": "Questa azione è permanente.",
      "cancel": "Annulla",
      "delete": "Elimina"
    }
  },

  "station_nav": {
    "dashboard": "Pannello di controllo",
    "profile": "Profilo",
    "playlist": "Playlist",
    "broadcast": "Trasmetti",
    "settings": "Impostazioni",
  },

  "station_profile": {
    "titles": {
      "logo": "Logo",
      "profile_info": "Profilo",
      "contact_info": "Informazioni di contatto",
      "social": "Social network",
      "apps": "Applicazioni",
    },
    "validation": {
      "logo_required": "Il logo è obbligatorio",
    },
    "upload_image": "Carica immagine",
    "picture_requirement_labels": {
      "format": "Formati di immagine accettati:",
      "size": "Dimensione minima dell'immagine:",
      "file_size": "Dimensione massima del file:",
      "square": "L'immagine deve essere quadrata",
    },

    "labels": {
      "name": "Nome",
      "slogan": "Slogan",
      "description": "Descrizione",
      "country": "Paese",
      "type_of_content": "Tipo di contenuto",
      "email": "Email",
      "phone": "Numero di telefono completo",
      "whatsapp": "Numero di WhatsApp completo",
      "website": "URL del sito web",
      "twitter": "URL di Twitter",
      "facebook": "URL di Facebook",
      "instagram": "URL di Instagram",
      "youtube": "URL di Youtube",
      "twitch": "URL di Twitch",
      "google_play": "URL di Google Play",
      "app_store": "URL di App Store"
    }
  },

  "plan_selector": {
    "price": {
      "per_month": "al mese",
      "n_per_month": "@n / mese",
    },

    "unlimited": "Illimitati",

    "trial": {
      "30_day": "30 giorni",
      "free_trial": "di prova gratuita",
      "tooltip": "Non ti verrà addebitato nulla fino alla fine del periodo di prova e puoi cancellare in qualsiasi momento"
    },

    "features": {
      "station": "Stazione",
      "stations": "Stazioni",
      "listeners": "Ascoltatori",
      "transfer": "Trasferimento",
      "storage": "Archiviazione",
      "staff": "utenti",
      "auto_dj": "Auto DJ",
      "stats": "Statistiche avanzate",
      "android_app": "Applicazione per Android",
    },

    "tooltips": {
      "one_station": "Puoi creare solo una stazione con questo piano",
      "n_stations": "Fino a @n stazioni diverse",
      "listeners": "Fino a @n ascoltatori simultanei",
      "transfer": "Con @tb TB di trasferimento mensile avrai la capacità di trasmettere circa @hours ore di audio",
      "storage": "@gb GB di archiviazione per musica o episodi precedenti",
      "staff": "Puoi aggiungere utenti per tutto il tuo team senza limiti",
      "auto_dj": "Trasmetti da una playlist quando sei offline",
      "stats": "Statistiche storiche e in tempo reale, scopri chi sta ascoltando le tue stazioni",
      "android_app": "Un'applicazione per Android con il tuo marchio e le tue stazioni, disponibile in tutto il mondo tramite Google Play",
    }
  },

  "pages": {

    "error": {
      "retry": "Riprova",
      "home": "Vai alla home",
      "default_message": "Si è verificato un errore",
      "offline": {
        "head": {
          "title": "Offline",
        },
        "title": "Sembra che tu sia offline",
        "text": "È necessario l'accesso a Internet per utilizzare @app_name",
      }
    },

    "login": {
      "head": {
        "title": "Accedi",
      },
      "title": "Accedi",
      "fields": {
        "email": "Email",
        "password": "Password",
      },
      "links": {
        "forgot": "Hai dimenticato la password?",
        "new_user": "Nuovo utente?",
        "sign_up": "Iscriviti",
      },
      "submit": "Accedi"
    },

    "recover": {
      "head": {
        "title": "Recupera il tuo account",
      },
      "title": "Recupera",
      "comment": "Ti invieremo un'email per recuperare il tuo account",
      "sent_message_html": "Abbiamo inviato un'email a <b>@email</b> con le istruzioni per continuare",
      "links": {
        "login": "Torna all'accesso",
      },
      "submit": "Invia",
    },

    "plans": {
      "head": {
        "title": "Piani e prezzi",
      },
      "title_1": "In diretta in 3... 2... 1...",
      "title_2": "Avvia la tua stazione radio in meno di 60 secondi.",
      "title_3": "Non ti verrà addebitato nulla fino alla fine del periodo di prova. E puoi cancellare in qualsiasi momento.",
      "plan_selector": {
        "select_btn_label": "Inizia la prova",
      }
    },

    "register": {
      "head": {
        "title": "Registrati",
      },
      "title": "Inizia la tua prova gratuita",
      "plan": {
        "selected_plan": "Piano selezionato",
        "n_per_month": "@n / mese",
        "limits": {
          "station": "Stazione",
          "stations": "Stazioni",
          "listeners": "Ascoltatori",
          "transfer": "Trasferimento",
          "storage": "Archiviazione",
        },
        "links": {
          "plans": "Torna a piani e prezzi"
        }
      },
      "form": {
        "title": "Raccontaci di te",
        "account_name_comment": "Se stai creando un account per un'organizzazione, puoi compilare questo campo con il nome dell'organizzazione",
        "fields": {
          "first_name": "Il tuo nome",
          "last_name": "Il tuo cognome",
          "account_name": "Un nome per il tuo account",
          "phone": "Il tuo telefono",
          "email": "La tua email",
          "password": "La tua password",
          "confirm_password": "Conferma la tua password",
        },
        "next": "Avanti",
      },
      "pay": {
        "title": "Dettagli del pagamento",
        "message": "Non verrai addebitato fino alla fine del tuo periodo di prova di 30 giorni e puoi cancellare in qualsiasi momento."
      },

      "back": "Torna al passo precedente",

      "verification": {
        "title": "Inserisci il codice di verifica",
        "message_html": "Ti abbiamo inviato un codice di verifica a <b>@email</b>",
        "submit": "Invia",
      },
      "links": {
        "login_comment": "Hai già un account?",
        "login_link": "Accedi",
      }
    },

    "user_recovery": {
      "head_page_title": {
        "expired": "Il link è scaduto",
        "used": "Il link è già stato utilizzato",
        "not_found": "Link non trovato",
        "ok": "Reimposta la tua password",
      },
      "fields": {
        "email": "Email",
        "password": "Nuova password",
        "confirm_password": "Conferma password",
      },
      "error": {
        "used_message_html": "Il link che hai utilizzato per accedere a questa pagina è già stato utilizzato. <br /> Crea un nuovo link dalla @user_recovery_page",
        "expired_message_html": "Il link che hai utilizzato per accedere a questa pagina è scaduto. <br /> Crea un nuovo link dalla @user_recovery_page",
        "not_found_message_html": "Il link che hai utilizzato per accedere a questa pagina non esiste. <br /> Crea un nuovo link dalla @user_recovery_page",
        "user_recovery_page": "pagina di recupero",
      },
      "submit": "Invia",
      "notifier": {
        "password_updated": "Password aggiornata",
      }
    },

    "accounts": {
      "head": {
        "title": "Account",
      },
      "title": "Scegli un account",
      "create_new_account": "crea un nuovo account",
      "or": "o",
      "no_items_message_html": "Non hai ancora un account emittente. <br/> Per iniziare a trasmettere, crea il tuo account emittente.",
      "no_items_create": "Crea il mio account emittente",
    },

    "accounts/create_account": {
      "head": {
        "title": "Scegli un piano",
      },
      "title": "Scegli un piano per il tuo nuovo account",
      "select": "Seleziona",
    },

    "accounts/create_account/plan": {
      "head": {
        "title": "Crea un account emittente",
      },
      "title": "Crea un account emittente",
      "plan": {
        "title": "Piano selezionato",
        "n_per_month": "@n / mese",
        "station": "Stazione",
        "stations": "Stazioni",
        "listeners": "Ascoltatori",
        "transfer": "Trasferimento",
        "storage": "Archiviazione",
        "back": "Torna a piani e prezzi",
      },
      "form": {
        "title": "Raccontaci del nuovo account",
        "fields": {
          "account_name":"Un nome per il tuo account",
          "account_name_message": "Se stai creando un account per un'organizzazione, puoi compilare questo campo con il nome dell'organizzazione",
        },
        "submit": "Crea",
        "next": "Avanti",
        "pay": {
          "title": "Dettagli del pagamento",
        },
        "back": "Torna al passo precedente",
      }
    },

    "account/dashboard": {
      "edit": {
        "tooltip": "Modifica",
        "dialog": {
          "field_label": "Nome dell'account",
          "title": "Modifica il nome del tuo account",
          "save": "Salva",
        }
      },

      "stats_map": {
        "all_stations": "Tutte le stazioni",
      },

      "station_item": {
        "on_air": "ON AIR",
        "off_air": "OFF AIR",
        "playlist": "Playlist",
        "live": "In diretta",
      }
    },

    "account/analytics": {
      "head": {
        "title": "Analisi",
      },
      "title": "Analisi",
    },

    "stations": {
      "head": {
        "title": "Stazioni",
      },
      "title": "Scegli una stazione",
      "create_new_station": "crea una nuova stazione",
      "or": "o",
      "no_items_message_html": "Questo account non ha ancora stazioni. <br /> Per iniziare a trasmettere, crea una nuova stazione.",
      "no_items_create": "Crea una stazione",
    },

    "stations/create_station": {
      "head": {
        "title": "Crea una stazione"
      },
      "title": "Crea una stazione",
      "submit": "Crea stazione",
      "notifier": {
        "station_created": "Nuova stazione creata",
      }
    },

    "station/dashboard": {
      "on_air": "ON AIR",
      "off_air": "OFF AIR",
      "playlist": "Playlist",
      "live": "In diretta",
      "preview": "Anteprima",
      "broadcast": "Trasmetti",
      "aria_pause": "Pausa",
      "aria_play": "Riproduci",
    },

    "station/profile": {
      "head": {
        "title": "Profilo",
      },
      "title": "Profilo",
      "submit": "Salva",
      "notifier": {
        "no_changes": "Nessuna modifica da salvare",
        "station_updated": "Stazione aggiornata",

      }
    },

    "station/playlist": {
      "head": {
        "title": "Playlist",
      },
      "title": "Playlist",
      "explain_html": "Crea una lista di musica o vecchi episodi per mantenere la tua stazione attiva 24x7 <br /> Quando non sei connesso o non stai trasmettendo in diretta, <b>Playlist</b> prenderà il controllo automaticamente.",
      "upload": "Carica",
      "browse": "Sfoglia",
      "upload_files": "Carica file",
      "tracks_title": "Brani",
      "track": "brano",
      "tracks": "brani",
      "actions": {
        "restart_playlist": "Riavvia playlist",
        "shuffle_playlist": "Mescola playlist",
        "unshuffle_playlist": "Smescola playlist",
        "drag_to_rearrange": "Trascina per riordinare",
        "edit": "Modifica",
        "delete": "Elimina",
      },
      "columns": {
        "title": "Titolo",
        "artist": "Artista",
        "album": "Album",
        "duration": "Durata",
      },
      "selection": {
        "one_track_selected": "1 brano selezionato",
        "n_tracks_selected": "@n brani selezionati",
        "delete_selected": "Elimina selezionati",
        "select_all": "Seleziona tutto",
        "unselect_all": "Deseleziona tutto",
      },
      "uploading": {
        "success": "Caricamento riuscito",
        "waiting": "In attesa",
        "in_progress": "In corso...",
        "retry": "Riprova",
        "clear_done": "Nascondi elementi completati",
      },
      "dialogs": {
        "delete_track": {
          "title": "Elimina brano @name"
        },
        "delete_tracks": {
          "title": "Elimina @n brani",
        },
        "edit_track": {
          "title": "Modifica brano @name",
          "fields": {
            "title": "Titolo",
            "artist": "Artista",
            "album": "Album",
          },
          "cancel": "Annulla",
          "save": "Salva",
        },
        "shuffle_playlist": {
          "title": "Mescola playlist",
          "message": "Sei sicuro di voler mescolare casualmente la playlist?",
          "cancel": "Annulla",
          "submit": "Mescola",
        },
        "unshuffle_playlist": {
          "title": "Smescola playlist",
          "message": "Sei sicuro di voler smescolare la playlist?",
          "cancel": "Annulla",
          "submit": "Smescola",
        },
        "restart_playlist": {
          "title": "Riavvia playlist",
          "message": "Sei sicuro di voler riavviare la playlist?",
          "cancel": "Annulla",
          "submit": "Riavvia",
        }
      },
      "upload_prevent_unload_message": "Uscire da questa pagina annullerà i caricamenti in sospeso. Vuoi uscire comunque?",
      "notifier": {
        "playlist_restarted": "Playlist riavviata",
        "track_deleted": "Brano eliminato",
        "deleting_n_tracks": "Eliminazione di @n brani",
        "n_tracks_deleted": "@n brani eliminati",
        "playlist_unshuffled": "Playlist smescolata",
        "playlist_shuffled": "Playlist mescolata",
      }
    },

    "station/broadcast": {
      "head": {
        "title": "Trasmetti",
      },
      "title": "Trasmetti",
      "icecast_settings": "Configurazione di Icecast",
      "fields": {
        "address": "Indirizzo",
        "port": "Porta",
        "mountpoint": "Punto di montaggio",
        "username": "Nome utente",
        "password": "Password",
        "encoding": "Formato",
      },
      "encoding_or": "o",
      "password_reset": "Reimposta",
      "links": {
        "title": "URL di trasmissione",
        "main": "PRINCIPALE",
      },
      "notifier": {
        "copied_to_clipboard": "Copiato negli appunti",
        "mount_password_reset": "Password reimpostata",
      },
      "dialogs": {
        "reset_password": {
          "title": "Reimposta la password del punto di montaggio",
          "message": "Sei sicuro di voler reimpostare la password del punto di montaggio?",
          "cancel": "Annulla",
          "submit": "Reimposta password",
        }
      }
    },

    "station/settings": {
      "head": {
        "title": "Configurazione",
      },
      "title": "Configurazione",
      "actions": {
        "title": "Azioni",
        "delete_station": "Elimina stazione",
      },
      "validate": {
        "station_name": "Il nome della stazione non corrisponde",
      },
      "notifier": {
        "station_deleted": "Stazione eliminata",
      },
      "dialogs": {
        "delete_station": {
          "title": "Elimina stazione @name",
          "message_html": "L'eliminazione di una stazione è un'azione permanente, non potrai più accedere alle informazioni della stazione, quindi assicurati di voler procedere.<br /><br />Se vuoi davvero eliminare la stazione @name inserisci il nome della stazione nel campo seguente: <b>@name</b><br />",
          "field_label": "Nome della stazione",
          "cancel": "Annulla",
          "submit": "Elimina",
        }
      }
    },

    "me": {
      "title": "Profilo",
      "fields": {
        "email": "La tua email",
        "first_name": "Il tuo nome",
        "last_name": "Il tuo cognome",
        "phone": "Il tuo telefono",
        "new_password": "Nuova password",
        "confirm_password": "Conferma password",
        "language": "Lingua preferita",
      },
      "submit": {
        "profile": "Salva",
        "password": "Salva",
      },
      "change_password": {
        "title": "Cambia la tua password",
      },
      "more": {
        "title": "Altro",
        "connected_devices": "Dispositivi connessi",
      },
      "notifier": {
        "no_changes": "Nessuna modifica da salvare",
        "profile_updated": "Profilo aggiornato",
        "password_updated": "Password aggiornata",
      }
    },

    "me/devices": {
      "head": {
        "title": "Dispositivi",
      },
      "title": "Dispositivi connessi",
      "note": "Lo stesso dispositivo può apparire più di una volta in questa lista. I dispositivi verranno disconnessi dopo 7 giorni di inattività.",
      "dialogs": {
        "disconnect": {
          "title": "Disconnetti dispositivo",
          "message": "Questa azione è permanente.",
          "cancel": "Annulla",
          "submit": "Disconnetti",
        },
      },

      "notifier": {
        "device_disconnected": "Dispositivo disconnesso",
      },

      "device": {
        "browser": "Browser",
        "os": "Sistema",
        "ip": "IP",
        "last_used": "Ultimo utilizzo",
        "connected": "Connesso",
        "unkown": "Sconosciuto",
        "tooltips": {
          "disconnect": "Disconnetti",
        }
      }
    },

    "account/members": {
      "head": {
        "title": "Membri"
      },
      "title": "Membri",

      "no_owner_message_p1": "Questa sezione è disponibile solo per gli amministratori dell'account",
      "no_owner_message_p2": "Contatta gli amministratori dell'account se hai bisogno di invitare persone a partecipare a questo account.",

      "Pending_invitations": "Inviti in sospeso",
      "no_pending_invitations_message": "Non ci sono inviti in sospeso",
      "invite_btn_text": "Invita persone",

      "validate": {
        "user_account_exists": "L'utente con email @email fa già parte di questo account",
      },

      "notifier": {
        "invitation_sent": "Invito inviato",
        "member_access_revoked": "Accesso membro revocato",
        "member_role_changed": "Ruolo di accesso membro aggiornato",
      },

      "actions": {
        "set_role_to": "Imposta ruolo a @role",
        "revoke_access": "Revoca accesso",
        "delete": "Elimina",
      },

      "dialogs": {
        "invite": {
          "title": "Invita persone a partecipare a questo account con ruolo @role",
          "submit": "Invita",
          "Email": "Email",
        }
      }
    },

    "email_invitation": {
      "head_page_title": {
        "not_found": "Invito non trovato",
        "expired": "L'invito è scaduto",
        "accepted": "L'invito è stato accettato",
        "rejected": "L'invito è stato rifiutato",
        "ok": "Invito in sospeso",
      },

      "error_message": {
        "not_found": "Il link che hai usato per accedere a questa pagina non esiste più o è stato eliminato",
        "expired": "L'invito è scaduto, contatta gli amministratori dell'account per ricevere un nuovo invito",
        "accepted": "L'invito è stato accettato",
        "rejected": "L'invito è stato rifiutato, se è stato un errore, contatta gli amministratori dell'account per ricevere un nuovo invito",
      },

      "description": {
        "with_sender_name_html": "<b>@sender</b> ti sta invitando a unirti a <b>@account</b> su Openstream.",
        "without_sender_name_html": "Sei stato invitato a unirti a <b>@account</b> su Openstream",
      },

      "login_as_btn_html": "Accedi come <b>@email</b> per accettare l'invito",

      "form": {
        "fields": {
          "first_name": "Il tuo nome",
          "last_name": "Il tuo cognome",
          "email": "La tua email",
          "password": "Password",
          "confirm_password": "Conferma password",
        },
        "pre_message_html": "Per <b>accettare</b> l'invito, completa il modulo.",
        "title": "Registrati",
        "submit": "Invia",
      },

      "notifier": {
        "accept_error": "Si è verificato un errore nell'accettare l'invito: @error"
      }
    },

    "me/invitations": {
      "head": {
        "title": "Inviti in sospeso",
      },
      "title": "Inviti in sospeso",

      "no_items_message": "Non hai inviti in sospeso",

      "notifier": {
        "accept_error": "Si è verificato un errore nell'accettare l'invito: @error",
        "accepted": "Invito accettato",
        "rejected": "Invito rifiutato",
      },

      "actions": {
        "reject": "Rifiuta",
        "accept": "Accetta",
      },

      "item_message_with_sender_html": "<b>@sender</b> ti sta invitando a unirti a <b>@account</b>",
      "item_message_without_sender_html": "Sei stato invitato a unirti a <b>@account</b>",

      "dialogs": {
        "reject": {
          "title": "Rifiuta invito",
          "message": "Sei sicuro di voler rifiutare l'invito?",
          "cancel": "Annulla",
          "reject": "Rifiuta invito",
        }
      }
    }
  },

  "user_menu": {
    "profile": "Profilo",
    "invitations": "Inviti",
    "accounts": "Account",
    "stations": "Stazioni",
    "sign_out": "Esci",
  }
}

export default locale;