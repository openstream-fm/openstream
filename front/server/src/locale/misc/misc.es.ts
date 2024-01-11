/// file: wip.es.ts
const locale: typeof import("./misc.en").default = {
  Relay: "Relay",
  Settings_updated: "Configuración actualizada",
  Master_relay: "Relay maestro",
  Enable_master_relay: "Habilitar relay maestro",
  Master_Relay_URL: "URL del relay maestro",
  Save: "Guardar",
  delete_station_not_owner_message_html: "Solo los administradores de la cuenta pueden eliminar estaciones.<br/><br/>Contacta a los administradores de la cuenta si deseas eliminar esta estación.",

  Cancel: "Cancelar",
  OK: "OK",
  Delete: "Eliminar",
  Create: "Crear",
  Copy: "Copiar",
  Done: "Listo",

  Copied_to_clipboard: "Copiado al portapapeles",

  Id: "Id",
  Title: "Título",
  Created: "Creado",
  Last_used: "Usado por última vez",

  Transfer_station: "Transferir estación",
  Station_name: "Nombre de la estación",
  station_transfer_title: "Transfiere estación @station a otra de tus cuentas",
  station_transfer_message_html: "Para transferir la estación @station a otra de tus cuentas, escribe el nombre de la estación: <b>@station</b> y elige la cuenta de destino.",
  station_transfer_not_owner_message_html: "Solo los administradores de la cuenta pueden transferir estaciones entre cuentas. <br/> <br/>Contacta a los administradores de la cuenta para transferir la estación a otra cuenta.",
  station_transfer_no_targets_message: "Debes tener acceso a otra cuenta para poder transferir esta emisora a otra cuenta.",
  Station_name_do_not_match: "El nombre de la estación no concuerda",
  Target_account_is_required: "La cuenta de destino es requerida",
  Station_transferred: "Estación tranferida",
  Select_a_target_account: "Selecciona una cuenta de destino",

  Type_password_proceed: "Ingresa tu contraseña para proceder con esta acción.",

  Welcome: "Bienvenido",

  account_welcome_title_html: "Hola <b>@name</b>",
  account_welcome_message_1_html: "Bienvenido a <b>@brand</b>",
  account_welcome_message_2_html: "Desde ahora eres el dueño de tu nueva cuenta",
  account_welcome_message_3_html: "Para empezar a transmitir ahora, agregá tu primera estación a tu cuenta",

  Create_my_first_station: "Crear mi primera estación",

  Your_email: "Tu email",
  Your_password: "Tu contraseña",

  "0_listeners": "0 oyentes",
  "1_listener": "1 oyente",
  "n_listeners": "@n oyentes",

  Enable_master_relay_redirect_mode: "Habilitar modo de redirección en el relay maestro",
  External_relay_error: "Error en el relay maestro",

  player: {
    Relay: "Relay",
    Live_Streaming: "En vivo",
    Playlist: "Playlist",
  },

  This_action_is_permanent: "Esta acción es permanente.",
  
  api_keys: {
    API_Keys: "Claves de acceso",
    API_key_deleted: "Clave de acceso eliminada",
    API_keys_page_message: "Crea claves de acceso para acceder a tus cuentas en openstream programaticamente o para dar acceso a aplicaciones y servicios de terceros.",
		Create_a_new_API_key: "Crear nueva clave de acceso",
    Remove_API_key: "Eliminar clave de acceso",
    API_key_title: "Título de la clave de acceso",
    API_key_title_explain: "El título será usado por ti para identificar esta clave de acceso",							
    Copy_contents_message: "Copia el contenido de la clave de acceso. Este códifo no será mostrado nuevamente.",
    API_key_contents: "Contenido de la clave de acceso",
  }
}

export default locale;