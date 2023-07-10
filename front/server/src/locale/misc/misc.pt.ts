/// file: misc.pt.ts
/// file: wip.pt.ts
const locale: typeof import("./misc.en").default = {
  Relay: "Relé",
  Settings_updated: "Configurações atualizadas",
  Master_relay: "Relé mestre",
  Enable_master_relay: "Habilitar relé mestre",
  Master_Relay_URL: "URL do relé mestre",
  Save: "Salvar",
  delete_station_not_owner_message_html: "Apenas os administradores da conta podem excluir estações.<br/><br/>Entre em contato com os administradores da conta se desejar excluir esta estação.",

  Cancel: "Cancelar",
  OK: "Aceitar",
  Transfer_station: "Transferir estação",
  Station_name: "Nome da estação",
  station_transfer_title: "Transferir estação @station para outra de suas contas",
  station_transfer_message_html: "Para transferir a estação @station para outra de suas contas, escreva o nome da estação: <b>@station</b> e escolha a conta de destino.",
  station_transfer_not_owner_message_html: "Apenas os administradores da conta podem transferir estações entre contas. <br/> <br/>Entre em contato com os administradores da conta para transferir a estação para outra conta.",
  station_transfer_no_targets_message: "Você deve ter acesso a outra conta para poder transferir esta estação para outra conta.",
  Station_name_do_not_match: "O nome da estação não coincide",
  Target_account_is_required: "A conta de destino é necessária",
  Station_transferred: "Estação transferida",
  Select_a_target_account: "Selecione uma conta de destino",

  Welcome: "Bem-vindo",

  account_welcome_title_html: "Olá <b>@name</b>",
  account_welcome_message_1_html: "Bem-vindo ao <b>@brand</b>",
  account_welcome_message_2_html: "A partir de agora, você é o proprietário de sua nova conta",
  account_welcome_message_3_html: "Para começar a transmitir agora, adicione sua primeira estação à sua conta",

  Create_my_first_station: "Criar minha primeira estação",
  
  "0_listeners": "0 ouvintes",
  "1_listener": "1 ouvinte",
  "n_listeners": "@n ouvintes",
}

export default locale;