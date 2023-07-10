/// file: misc.fr.ts
/// file: wip.fr.ts
const locale: typeof import("./misc.en").default = {
  Relay: "Relais",
  Settings_updated: "Configuration mise à jour",
  Master_relay: "Relais maître",
  Enable_master_relay: "Activer le relais maître",
  Master_Relay_URL: "URL du relais maître",
  Save: "Enregistrer",
  delete_station_not_owner_message_html: "Seuls les administrateurs du compte peuvent supprimer des stations.<br/><br/>Contactez les administrateurs du compte si vous souhaitez supprimer cette station.",

  Cancel: "Annuler",
  OK: "Accepter",
  Transfer_station: "Transférer la station",
  Station_name: "Nom de la station",
  station_transfer_title: "Transférer la station @station à un autre de vos comptes",
  station_transfer_message_html: "Pour transférer la station @station à un autre de vos comptes, écrivez le nom de la station: <b>@station</b> et choisissez le compte de destination.",
  station_transfer_not_owner_message_html: "Seuls les administrateurs du compte peuvent transférer des stations entre comptes. <br/> <br/>Contactez les administrateurs du compte pour transférer la station à un autre compte.",
  station_transfer_no_targets_message: "Vous devez avoir accès à un autre compte pour pouvoir transférer cette station à un autre compte.",
  Station_name_do_not_match: "Le nom de la station ne correspond pas",
  Target_account_is_required: "Le compte cible est requis",
  Station_transferred: "Station transférée",
  Select_a_target_account: "Sélectionnez un compte cible",

  Welcome: "Bienvenue",

  account_welcome_title_html: "Bonjour <b>@name</b>",
  account_welcome_message_1_html: "Bienvenue à <b>@brand</b>",
  account_welcome_message_2_html: "Désormais, vous êtes le propriétaire de votre nouveau compte",
  account_welcome_message_3_html: "Pour commencer à diffuser maintenant, ajoutez votre première station à votre compte",

  Create_my_first_station: "Créer ma première station",

  "0_listeners": "0 auditeurs",
  "1_listener": "1 auditeur",
  "n_listeners": "@n auditeurs",
}

export default locale;