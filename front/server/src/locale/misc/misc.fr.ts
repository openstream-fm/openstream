const locale: typeof import("./misc.en").default = {
  Relay: "Relais",
  Settings_updated: "Paramètres mis à jour",
  Master_relay: "Relais maître",
  Enable_master_relay: "Activer le relais maître",
  Master_Relay_URL: "URL du relais maître",
  Save: "Enregistrer",
  delete_station_not_owner_message_html: "Seuls les administrateurs du compte peuvent supprimer des stations.<br/><br/>Contactez les administrateurs du compte si vous souhaitez supprimer cette station.",

  Cancel: "Annuler",
  OK: "OK",
  Delete: "Supprimer",
  Create: "Créer",
  Copy: "Copier",
  Done: "Terminé",

  Copied_to_clipboard: "Copié dans le presse-papiers",

  Id: "Id",
  Title: "Titre",
  Created: "Créé",
  Last_used: "Dernière utilisation",

  Transfer_station: "Transférer la station",
  Station_name: "Nom de la station",
  station_transfer_title: "Transférer la station @station à un autre de vos comptes",
  station_transfer_message_html: "Pour transférer la station @station à un autre de vos comptes, saisissez le nom de la station : <b>@station</b> et choisissez le compte de destination.",
  station_transfer_not_owner_message_html: "Seuls les administrateurs du compte peuvent transférer des stations entre comptes. <br/> <br/>Contactez les administrateurs du compte pour transférer la station à un autre compte.",
  station_transfer_no_targets_message: "Vous devez avoir accès à un autre compte pour pouvoir transférer cette station à un autre compte.",
  Station_name_do_not_match: "Le nom de la station ne correspond pas",
  Target_account_is_required: "Un compte de destination est requis",
  Station_transferred: "Station transférée",
  Select_a_target_account: "Sélectionnez un compte de destination",

  Type_password_proceed: "Entrez votre mot de passe pour procéder à cette action.",

  Welcome: "Bienvenue",

  account_welcome_title_html: "Bonjour <b>@name</b>",
  account_welcome_message_1_html: "Bienvenue chez <b>@brand</b>",
  account_welcome_message_2_html: "Vous êtes maintenant le propriétaire de votre nouveau compte",
  account_welcome_message_3_html: "Pour commencer à diffuser maintenant, ajoutez votre première station à votre compte",

  Create_my_first_station: "Créer ma première station",

  Your_email: "Votre email",
  Your_password: "Votre mot de passe",

  "0_listeners": "0 auditeurs",
  "1_listener": "1 auditeur",
  "n_listeners": "@n auditeurs",

  Enable_master_relay_redirect_mode: "Activer le mode de redirection du relais maître",
  External_relay_error: "Erreur du relais externe",

  player: {
    Relay: "Relais",
    Live_Streaming: "En direct",
    Playlist: "Playlist",
  },

  This_action_is_permanent: "Cette action est permanente.",
  
  api_keys: {
    API_Keys: "Clés API",
    API_key_deleted: "Clé API supprimée",
    API_keys_page_message: "Créez des clés API pour accéder à vos comptes de manière programmatique ou pour donner accès à des applications et services tiers.",
		Create_a_new_API_key: "Créer une nouvelle clé API",
    Remove_API_key: "Supprimer la clé API",
    API_key_title: "Titre de la clé API",
    API_key_title_explain: "Le titre sera utilisé par vous pour identifier cette clé API",							
    Copy_contents_message: "Copiez le contenu de la clé API. Ce code ne sera pas affiché à nouveau.",
    API_key_contents: "Contenu de la clé API",
  }
}

export default locale;