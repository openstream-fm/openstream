/// file: misc.de.ts
/// file: wip.de.ts
const locale: typeof import("./misc.en").default = {
  Relay: "Relais",
  Settings_updated: "Einstellungen aktualisiert",
  Master_relay: "Master-Relais",
  Enable_master_relay: "Master-Relais aktivieren",
  Master_Relay_URL: "URL des Master-Relais",
  Save: "Speichern",
  delete_station_not_owner_message_html: "Nur Kontoadministratoren können Stationen löschen.<br/><br/>Kontaktieren Sie die Kontoadministratoren, wenn Sie diese Station löschen möchten.",

  Cancel: "Abbrechen",
  OK: "Akzeptieren",
  Transfer_station: "Station übertragen",
  Station_name: "Name der Station",
  station_transfer_title: "Übertrage Station @station auf ein anderes deiner Konten",
  station_transfer_message_html: "Um die Station @station auf ein anderes deiner Konten zu übertragen, gib den Namen der Station: <b>@station</b> ein und wähle das Zielkonto aus.",
  station_transfer_not_owner_message_html: "Nur Kontoadministratoren können Stationen zwischen Konten übertragen. <br/> <br/>Kontaktieren Sie die Kontoadministratoren, um die Station auf ein anderes Konto zu übertragen.",
  station_transfer_no_targets_message: "Du musst Zugriff auf ein anderes Konto haben, um diese Station auf ein anderes Konto zu übertragen.",
  Station_name_do_not_match: "Der Stationsname stimmt nicht überein",
  Target_account_is_required: "Ein Zielkonto ist erforderlich",
  Station_transferred: "Station übertragen",
  Select_a_target_account: "Wähle ein Zielkonto aus",

  Welcome: "Willkommen",

  account_welcome_title_html: "Hallo <b>@name</b>",
  account_welcome_message_1_html: "Willkommen bei <b>@brand</b>",
  account_welcome_message_2_html: "Ab jetzt bist du der Besitzer deines neuen Kontos",
  account_welcome_message_3_html: "Um jetzt mit der Übertragung zu beginnen, füge deine erste Station zu deinem Konto hinzu",

  Create_my_first_station: "Meine erste Station erstellen",

  "0_listeners": "0 Zuhörer",
  "1_listener": "1 Zuhörer",
  "n_listeners": "@n Zuhörer",
}

export default locale;