const locale: typeof import("./misc.en").default = {
  Relay: "Relay",
  Settings_updated: "Einstellungen aktualisiert",
  Master_relay: "Master-Relay",
  Enable_master_relay: "Master-Relay aktivieren",
  Master_Relay_URL: "URL des Master-Relays",
  Save: "Speichern",
  delete_station_not_owner_message_html: "Nur Kontoadministratoren können Stationen löschen.<br/><br/>Kontaktiere die Kontoadministratoren, wenn du diese Station löschen möchtest.",

  Cancel: "Abbrechen",
  OK: "OK",
  Delete: "Löschen",
  Create: "Erstellen",
  Copy: "Kopieren",
  Done: "Fertig",

  Copied_to_clipboard: "In die Zwischenablage kopiert",

  Id: "Id",
  Title: "Titel",
  Created: "Erstellt",
  Last_used: "Zuletzt verwendet",

  Transfer_station: "Station übertragen",
  Station_name: "Stationsname",
  station_transfer_title: "Übertrage Station @station auf ein anderes deiner Konten",
  station_transfer_message_html: "Um die Station @station auf ein anderes deiner Konten zu übertragen, gib den Stationsnamen ein: <b>@station</b> und wähle das Zielkonto aus.",
  station_transfer_not_owner_message_html: "Nur Kontoadministratoren können Stationen zwischen Konten übertragen. <br/><br/>Kontaktiere die Kontoadministratoren, um die Station auf ein anderes Konto zu übertragen.",
  station_transfer_no_targets_message: "Du musst Zugriff auf ein anderes Konto haben, um diese Station auf ein anderes Konto übertragen zu können.",
  Station_name_do_not_match: "Der Stationsname stimmt nicht überein",
  Target_account_is_required: "Ein Zielkonto ist erforderlich",
  Station_transferred: "Station übertragen",
  Select_a_target_account: "Wähle ein Zielkonto aus",

  Type_password_proceed: "Gib dein Passwort ein, um mit dieser Aktion fortzufahren.",

  Welcome: "Willkommen",

  account_welcome_title_html: "Hallo <b>@name</b>",
  account_welcome_message_1_html: "Willkommen bei <b>@brand</b>",
  account_welcome_message_2_html: "Ab jetzt bist du der Besitzer deines neuen Kontos",
  account_welcome_message_3_html: "Um jetzt zu senden, füge deiner Kontoliste deine erste Station hinzu",

  Create_my_first_station: "Meine erste Station erstellen",

  Your_email: "Deine E-Mail",
  Your_password: "Dein Passwort",

  "0_listeners": "0 Zuhörer",
  "1_listener": "1 Zuhörer",
  "n_listeners": "@n Zuhörer",

  Enable_master_relay_redirect_mode: "Umleitungsmodus im Master-Relay aktivieren",
  External_relay_error: "Fehler im Master-Relay",

  player: {
    Relay: "Relay",
    Live_Streaming: "Live-Streaming",
    Playlist: "Playlist",
  },

  This_action_is_permanent: "Diese Aktion ist dauerhaft.",
  
  api_keys: {
    API_Keys: "API-Schlüssel",
    API_key_deleted: "API-Schlüssel gelöscht",
    API_keys_page_message: "Erstelle API-Schlüssel, um programmatisch auf deine Konten zuzugreifen oder um Drittanbieter-Apps und -Diensten Zugriff zu gewähren.",
		Create_a_new_API_key: "Einen neuen API-Schlüssel erstellen",
    Remove_API_key: "API-Schlüssel entfernen",
    API_key_title: "Titel des API-Schlüssels",
    API_key_title_explain: "Der Titel wird von dir verwendet, um diesen API-Schlüssel zu identifizieren",							
    Copy_contents_message: "Kopiere den Inhalt des API-Schlüssels. Dieser Code wird nicht noch einmal angezeigt.",
    API_key_contents: "Inhalt des API-Schlüssels",
  }
}

export default locale;