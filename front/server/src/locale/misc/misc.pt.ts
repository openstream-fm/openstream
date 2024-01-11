const locale: typeof import("./misc.en").default = {
  Relay: "Relay",
  Settings_updated: "Configurações atualizadas",
  Master_relay: "Relay mestre",
  Enable_master_relay: "Habilitar relay mestre",
  Master_Relay_URL: "URL do relay mestre",
  Save: "Salvar",
  delete_station_not_owner_message_html: "Apenas os administradores da conta podem excluir estações.<br/><br/>Contate os administradores da conta se desejar excluir esta estação.",

  Cancel: "Cancelar",
  OK: "OK",
  Delete: "Excluir",
  Create: "Criar",
  Copy: "Copiar",
  Done: "Concluído",

  Copied_to_clipboard: "Copiado para a área de transferência",

  Id: "Id",
  Title: "Título",
  Created: "Criado",
  Last_used: "Último uso",

  Transfer_station: "Transferir estação",
  Station_name: "Nome da estação",
  station_transfer_title: "Transfira a estação @station para outra de suas contas",
  station_transfer_message_html: "Para transferir a estação @station para outra de suas contas, digite o nome da estação: <b>@station</b> e escolha a conta de destino.",
  station_transfer_not_owner_message_html: "Apenas os administradores da conta podem transferir estações entre contas. <br/> <br/>Contate os administradores da conta para transferir a estação para outra conta.",
  station_transfer_no_targets_message: "Você deve ter acesso a outra conta para poder transferir esta estação para outra conta.",
  Station_name_do_not_match: "O nome da estação não corresponde",
  Target_account_is_required: "A conta de destino é necessária",
  Station_transferred: "Estação transferida",
  Select_a_target_account: "Selecione uma conta de destino",

  Type_password_proceed: "Digite sua senha para prosseguir com esta ação.",

  Welcome: "Bem-vindo",

  account_welcome_title_html: "Olá <b>@name</b>",
  account_welcome_message_1_html: "Bem-vindo ao <b>@brand</b>",
  account_welcome_message_2_html: "A partir de agora você é o proprietário de sua nova conta",
  account_welcome_message_3_html: "Para começar a transmitir agora, adicione sua primeira estação à sua conta",

  Create_my_first_station: "Criar minha primeira estação",

  Your_email: "Seu e-mail",
  Your_password: "Sua senha",

  "0_listeners": "0 ouvintes",
  "1_listener": "1 ouvinte",
  "n_listeners": "@n ouvintes",

  Enable_master_relay_redirect_mode: "Habilitar modo de redirecionamento no relay mestre",
  External_relay_error: "Erro no relay externo",

  player: {
    Relay: "Relay",
    Live_Streaming: "Transmissão ao vivo",
    Playlist: "Playlist",
  },

  This_action_is_permanent: "Esta ação é permanente.",
  
  api_keys: {
    API_Keys: "Chaves de API",
    API_key_deleted: "Chave de API excluída",
    API_keys_page_message: "Crie chaves de API para acessar suas contas programaticamente ou para dar acesso a aplicativos e serviços de terceiros.",
    Create_a_new_API_key: "Criar uma nova chave de API",
    Remove_API_key: "Excluir chave de API",
    API_key_title: "Título da chave de API",
    API_key_title_explain: "O título será usado por você para identificar esta chave de API",
    Copy_contents_message: "Copie o conteúdo da chave de API. Este código não será mostrado novamente.",
    API_key_contents: "Conteúdo da chave de API",
  }
}

export default locale;