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
  OK: "Aceptar",
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

  Welcome: "Bienvenido",

  account_welcome_title_html: "Hola <b>@name</b>",
  account_welcome_message_1_html: "Bienvenido a <b>@brand</b>",
  account_welcome_message_2_html: "Desde ahora eres el dueño de tu nueva cuenta",
  account_welcome_message_3_html: "Para empezar a transmitir ahora, agregá tu primera estación a tu cuenta",

  Create_my_first_station: "Crear mi primera estación",

  "0_listeners": "0 oyentes",
  "1_listener": "1 oyente",
  "n_listeners": "@n oyentes",
}

export default locale;