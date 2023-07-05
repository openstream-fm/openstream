/// file: misc.it.ts
/// file: wip.it.ts
const locale: typeof import("./misc.en").default = {
  Relay: "Relay",
  Settings_updated: "Impostazioni aggiornate",
  Master_relay: "Relay principale",
  Enable_master_relay: "Abilita relay principale",
  Master_Relay_URL: "URL del relay principale",
  Save: "Salva",
  delete_station_not_owner_message_html: "Solo gli amministratori dell'account possono eliminare le stazioni.<br/><br/>Contatta gli amministratori dell'account se desideri eliminare questa stazione.",

  Cancel: "Annulla",
  OK: "OK",
  Transfer_station: "Trasferisci stazione",
  Station_name: "Nome della stazione",
  station_transfer_title: "Trasferisci la stazione @station ad un altro tuo account",
  station_transfer_message_html: "Per trasferire la stazione @station ad un altro tuo account, scrivi il nome della stazione: <b>@station</b> e scegli l'account di destinazione.",
  station_transfer_not_owner_message_html: "Solo gli amministratori dell'account possono trasferire stazioni tra account. <br/> <br/>Contatta gli amministratori dell'account per trasferire la stazione ad un altro account.",
  station_transfer_no_targets_message: "Devi avere accesso ad un altro account per poter trasferire questa stazione ad un altro account.",
  Station_name_do_not_match: "Il nome della stazione non corrisponde",
  Target_account_is_required: "L'account di destinazione è richiesto",
  Station_transferred: "Stazione trasferita",
  Select_a_target_account: "Seleziona un account di destinazione",

  Welcome: "Benvenuto",

  account_welcome_title_html: "Ciao <b>@name</b>",
  account_welcome_message_1_html: "Benvenuto su <b>@brand</b>",
  account_welcome_message_2_html: "Da ora in poi sei il proprietario del tuo nuovo account",
  account_welcome_message_3_html: "Per iniziare a trasmettere ora, aggiungi la tua prima stazione al tuo account",

  Create_my_first_station: "Crea la mia prima stazione",
}

export default locale;