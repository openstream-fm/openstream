const locale: typeof import("./misc.en.js").default = {
  Relay: "Relay",
  Settings_updated: "Impostazioni aggiornate",
  Master_relay: "Relay principale",
  Enable_master_relay: "Abilita relay principale",
  Master_Relay_URL: "URL del relay principale",
  Save: "Salva",
  delete_station_not_owner_message_html: "Solo gli amministratori dell'account possono eliminare le stazioni.<br/><br/>Contatta gli amministratori dell'account se desideri eliminare questa stazione.",

  Cancel: "Annulla",
  OK: "OK",
  Delete: "Elimina",
  Create: "Crea",
  Copy: "Copia",
  Done: "Fatto",

  Copied_to_clipboard: "Copiato negli appunti",

  Id: "Id",
  Title: "Titolo",
  Created: "Creato",
  Last_used: "Ultimo utilizzo",

  Transfer_station: "Trasferisci stazione",
  Station_name: "Nome della stazione",
  station_transfer_title: "Trasferisci la stazione @station ad un altro tuo account",
  station_transfer_message_html: "Per trasferire la stazione @station ad un altro tuo account, scrivi il nome della stazione: <b>@station</b> e scegli l'account di destinazione.",
  station_transfer_not_owner_message_html: "Solo gli amministratori dell'account possono trasferire stazioni tra account. <br/> <br/>Contatta gli amministratori dell'account per trasferire la stazione ad un altro account.",
  station_transfer_no_targets_message: "Devi avere accesso ad un altro account per poter trasferire questa stazione ad un altro account.",
  Station_name_do_not_match: "Il nome della stazione non corrisponde",
  Target_account_is_required: "È richiesto un account di destinazione",
  Station_transferred: "Stazione trasferita",
  Select_a_target_account: "Seleziona un account di destinazione",

  Type_password_proceed: "Inserisci la tua password per procedere con questa azione.",

  Welcome: "Benvenuto",

  account_welcome_title_html: "Ciao <b>@name</b>",
  account_welcome_message_1_html: "Benvenuto in <b>@brand</b>",
  account_welcome_message_2_html: "Da ora sei il proprietario del tuo nuovo account",
  account_welcome_message_3_html: "Per iniziare a trasmettere ora, aggiungi la tua prima stazione al tuo account",

  Create_my_first_station: "Crea la mia prima stazione",

  Your_email: "La tua email",
  Your_password: "La tua password",

  "0_listeners": "0 ascoltatori",
  "1_listener": "1 ascoltatore",
  "n_listeners": "@n ascoltatori",

  Enable_master_relay_redirect_mode: "Abilita la modalità di reindirizzamento nel relay principale",
  External_relay_error: "Errore nel relay esterno",

  player: {
    Relay: "Relay",
    Live_Streaming: "In diretta",
    Playlist: "Playlist",
  },

  This_action_is_permanent: "Questa azione è permanente.",
  
  api_keys: {
    API_Keys: "Chiavi API",
    API_key_deleted: "Chiave API eliminata",
    API_keys_page_message: "Crea chiavi API per accedere programmaticamente ai tuoi account su openstream o per dare accesso ad applicazioni e servizi di terze parti.",
		Create_a_new_API_key: "Crea una nuova chiave API",
    Remove_API_key: "Rimuovi chiave API",
    API_key_title: "Titolo della chiave API",
    API_key_title_explain: "Il titolo sarà usato da te per identificare questa chiave API",							
    Copy_contents_message: "Copia il contenuto della chiave API. Questo codice non sarà mostrato nuovamente.",
    API_key_contents: "Contenuto della chiave API",
  }
}

export default locale;