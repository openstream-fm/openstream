/// file: studio.pt.ts
import stats_map from "../share/stats-map/stats-map.pt.js";
import validate from "../share/validate/validate.pt.js";
import countries from "../share/countries/countries.pt.js";
import langs from "../share/langs/langs.pt.js";
import type_of_content from "../share/type-of-content/type-of-content.pt.js";
import analytics from "../share/analytics/analytics.pt.js";
import payments from "../share/payments/payments.pt.js";
import station_profile from "../share/station-profile/station-profile.pt.js";
import misc from "../misc/misc.pt.js";
import language from "../share/language/language.pt.js";

const locale: import("./studio.locale.js").StudioLocale = {

  "lang": "pt",
  "region": null,

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "brand_name": "Openstream",

  // @notranslate
  "app_name": "Openstream Studio",
  
  type_of_content,
  countries,
  langs,
  validate,
  stats_map,
  analytics,
  payments,
  misc,
  station_profile,
  language,

  "copy_to_clipboard": "Copiar para a área de transferência",
  "show_password": "Mostrar senha",
  "hide_password": "Ocultar senha",

  "prevent_unload_message":  "Se você sair desta página, as alterações feitas serão perdidas. Você quer sair da página mesmo assim?",

  "drawer": {
    "account_selector": {
      "see_all_accounts": "Ver todas as contas",
    },
    "dashboard": "Painel",
    "stations": "Estações",
    "members": "Membros",
    "analytics": "Analítica",
  },

  "limits": {
    "of": "de",
    "stations": "Estações",
    "listeners": "Ouvintes",
    "transfer": "Transferência",
    "storage": "Armazenamento",
  },

  "dialogs": {
    "delete": {
      "default_message": "Esta ação é permanente.",
      "cancel": "Cancelar",
      "delete": "Excluir"
    }
  },

  "station_nav": {
    "dashboard": "Painel",
    "profile": "Perfil",
    "playlist": "Playlist",
    "broadcast": "Transmitir",
    "settings": "Configurações",
  },

  "plan_selector": {
    "price": {
      "per_month": "por mês",
      "n_per_month": "@n / mês",
    },

    "unlimited": "Ilimitados",

    "trial": {
      "30_day": "30 dias",
      "free_trial": "de teste gratuito",
      "tooltip": "Você não será cobrado até que seu teste termine, e você pode cancelar a qualquer momento"
    },

    "features": {
      "station": "Estação",
      "stations": "Estações",
      "listeners": "Ouvintes",
      "transfer": "Transferência",
      "storage": "Armazenamento",
      "staff": "usuários",
      "auto_dj": "Auto DJ",
      "stats": "Estatísticas avançadas",
      "android_app": "Aplicativo para Android",
    },

    "tooltips": {
      "one_station": "Você só pode criar uma estação neste plano",
      "n_stations": "Até @n estações diferentes",
      "listeners": "Até @n ouvintes simultâneos",
      "transfer": "Com @tb TB de transferência mensal, você terá capacidade para transmitir cerca de @hours horas de áudio",
      "storage": "@gb GB de armazenamento para música ou episódios antigos",
      "staff": "Você pode adicionar usuários para toda a sua equipe sem limite",
      "auto_dj": "Transmita a partir de uma playlist quando estiver offline",
      "stats": "Estatísticas históricas e ao vivo, saiba quem está ouvindo suas estações",
      "android_app": "Um aplicativo para Android com sua marca e suas estações, disponível em todo o mundo através do Google Play",
    }
  },

  "pages": {

    "error": {
      "retry": "Tentar novamente",
      "home": "Ir para o início",
      "default_message": "Ocorreu um erro",
      "offline": {
        "head": {
          "title": "Sem conexão",
        },
        "title": "Parece que você está sem conexão",
        "text": "É necessário acesso à internet para usar o @app_name",
      }
    },

    "login": {
      "head": {
        "title": "Entrar",
      },
      "title": "Entrar",
      "fields": {
        "email": "Email",
        "password": "Senha",
      },
      "links": {
        "forgot": "Esqueceu sua senha?",
        "new_user": "Novo usuário?",
        "sign_up": "Registre-se",
      },
      "submit": "Entrar"
    },

    "recover": {
      "head": {
        "title": "Recuperar sua conta",
      },
      "title": "Recuperar",
      "comment": "Enviaremos um email para você recuperar sua conta",
      "sent_message_html": "Enviamos um email para <b>@email</b> com instruções para continuar",
      "links": {
        "login": "Voltar para Entrar",
      },
      "submit": "Enviar",
    },

    "plans": {
      "head": {
        "title": "Planos e Preços",
      },
      "title_1": "Ao vivo em 3... 2... 1...",
      "title_2": "Comece sua estação de rádio em menos de 60 segundos.",
      "title_3": "Você não será cobrado até que seu teste termine. E você pode cancelar a qualquer momento.",
      "plan_selector": {
        "select_btn_label": "Começar Teste",
      }
    },

    "register": {
      "head": {
        "title": "Registre-se",
      },
      "title": "Comece seu teste gratuito",
      "plan": {
        "selected_plan": "Plano selecionado",
        "n_per_month": "@n / mês",
        "limits": {
          "station": "Estação",
          "stations": "Estações",
          "listeners": "Ouvintes",
          "transfer": "Transferência",
          "storage": "Armazenamento",
        },
        "links": {
          "plans": "Voltar para planos e preços"
        }
      },
      "form": {
        "title": "Conte-nos sobre você",
        "account_name_comment": "Se você está criando uma conta para uma organização, você pode preencher este campo com o nome da organização",
        "fields": {
          "first_name": "Seu nome",
          "last_name": "Seu sobrenome",
          "account_name": "Um nome para sua conta",
          "phone": "Seu telefone",
          "email": "Seu email",
          "password": "Sua senha",
          "confirm_password": "Confirme sua senha",
        },
        "next": "Próximo",
      },
      "pay": {
        "title": "Detalhes do pagamento",
        "message": "Você não será cobrado até que seu teste gratuito de 30 dias termine e você pode cancelar a qualquer momento."
      },

      "back": "Voltar para o passo anterior",

      "verification": {
        "title": "Insira o código de verificação",
        "message_html": "Enviamos um código de verificação para <b>@email</b>",
        "submit": "Enviar",
      },
      "links": {
        "login_comment": "Já tem uma conta?",
        "login_link": "Entrar",
      }
    },

    "user_recovery": {
      "head_page_title": {
        "expired": "O link expirou",
        "used": "O link já foi usado",
        "not_found": "Link não encontrado",
        "ok": "Redefina sua senha",
      },
      "fields": {
        "email": "Email",
        "password": "Nova senha",
        "confirm_password": "Confirme a senha",
      },
      "error": {
        "used_message_html": "O link que você usou para acessar esta página já foi utilizado.<br /> Crie um novo link a partir da @user_recovery_page",
        "expired_message_html": "O link que você usou para acessar esta página expirou.<br /> Crie um novo link a partir da @user_recovery_page",
        "not_found_message_html": "O link que você usou para acessar esta página não existe.<br /> Crie um novo link a partir da @user_recovery_page",
        "user_recovery_page": "página de recuperação",
      },
      "submit": "Enviar",
      "notifier": {
        "password_updated": "Senha atualizada",
      }
    },

    "accounts": {
      "head": {
        "title": "Contas",
      },
      "title": "Escolha uma conta",
      "create_new_account": "crie uma nova conta",
      "or": "ou",
      "no_items_message_html": "Você ainda não tem uma conta de transmissão.<br/>Para começar a transmitir, crie sua conta de transmissão.",
      "no_items_create": "Criar minha conta de transmissão",
    },

    "accounts/create_account": {
      "head": {
        "title": "Escolha um plano",
      },
      "title": "Escolha um plano para sua nova conta",
      "select": "Selecionar",
    },

    "accounts/create_account/plan": {
      "head": {
        "title": "Criar uma conta de transmissor",
      },
      "title": "Criar uma conta de transmissor",
      "plan": {
        "title": "Plano selecionado",
        "n_per_month": "@n / mês",
        "station": "Estação",
        "stations": "Estações",
        "listeners": "Ouvintes",
        "transfer": "Transferência",
        "storage": "Armazenamento",
        "back": "Voltar para planos e preços",
      },
      "form": {
        "title": "Informe-nos sobre a nova conta",
        "fields": {
          "account_name":"Um nome para sua nova conta",
          "account_name_message": "Se você estiver criando uma conta para uma organização, pode preencher este campo com o nome da organização"
        },
        "submit": "Criar",
        "next": "Próximo",
        "pay": {
          "title": "Detalhes do pagamento",
        },
        "back": "Voltar ao passo anterior",
      }
    },
    
    "account/dashboard": {
      "edit": {
        "tooltip": "Editar",
        "dialog": {
          "field_label": "Nome da conta",
          "title": "Edite o nome da sua conta",
          "save": "Salvar",
        }
      },

      "stats_map": {
        "all_stations": "Todas as estações",
      },

      "station_item": {
        "on_air": "ON AIR",
        "off_air": "OFF AIR",
        "playlist": "Playlist",
        "live": "Ao vivo",
      }
    },

    "account/analytics": {
      "head": {
        "title": "Análise",
      },
      "title": "Análise",
    },

    "stations": {
      "head": {
        "title": "Estações",
      },
      "title": "Escolha uma estação",
      "create_new_station": "criar uma nova estação",
      "or": "ou",
      "no_items_message_html": "Esta conta ainda não possui estações.<br />Para começar a transmitir, crie uma nova estação.",
      "no_items_create": "Criar uma estação",
    },

    "stations/create_station": {
      "head": {
        "title": "Criar uma estação"
      },
      "title": "Criar uma estação",
      "submit": "Criar estação",
      "notifier": {
        "station_created": "Nova estação criada",
      }
    },

    "station/dashboard": {
      "on_air": "ON AIR",
      "off_air": "OFF AIR",
      "playlist": "Playlist",
      "live": "Ao vivo",
      "preview": "Visualizar",
      "broadcast": "Transmitir",
      "aria_pause": "Pausar",
      "aria_play": "Reproduzir",
    },

    "station/profile": {
      "head": {
        "title": "Perfil",
      },
      "title": "Perfil",
      "submit": "Salvar",
      "notifier": {
        "no_changes": "Sem alterações para salvar",
        "station_updated": "Estação atualizada",

      }
    },

    "station/playlist": {
      "head": {
        "title": "Playlist",
      },
      "title": "Playlist",
      "explain_html": "Crie uma lista de músicas ou episódios antigos para manter sua estação ativa 24x7<br /> Quando você estiver offline ou não estiver transmitindo ao vivo, o <b>Playlist</b> assumirá o controle automaticamente.",
      "upload": "Carregar",
      "browse": "Navegar",
      "upload_files": "Carregar arquivos",
      "tracks_title": "Faixas",
      "track": "faixa",
      "tracks": "faixas",
      "actions": {
        "restart_playlist": "Reiniciar playlist",
        "shuffle_playlist": "Embaralhar playlist",
        "unshuffle_playlist": "Desembaralhar playlist",
        "drag_to_rearrange": "Arraste para reorganizar",
        "edit": "Editar",
        "delete": "Excluir",
      },
      "columns": {
        "title": "Título",
        "artist": "Artista",
        "album": "Álbum",
        "duration": "Duração",
      },
      "selection": {
        "one_track_selected": "1 faixa selecionada",
        "n_tracks_selected": "@n faixas selecionadas",
        "delete_selected": "Excluir selecionados",
        "select_all": "Selecionar todos",
        "unselect_all": "Desselecionar todos",
      },
      "uploading": {
        "success": "Carregado com sucesso",
        "waiting": "Aguardando",
        "in_progress": "Em andamento...",
        "retry": "Tentar novamente",
        "clear_done": "Ocultar itens concluídos",
      },
      "dialogs": {
        "delete_track": {
          "title": "Excluir faixa @name"
        },
        "delete_tracks": {
          "title": "Excluir @n faixas",
        },
        "edit_track": {
          "title": "Editar faixa @name",
          "fields": {
            "title": "Título",
            "artist": "Artista",
            "album": "Álbum",
          },
          "cancel": "Cancelar",
          "save": "Salvar",
        },
        "shuffle_playlist": {
          "title": "Embaralhar playlist",
          "message": "Tem certeza de que deseja embaralhar aleatoriamente a lista de reprodução?",
          "cancel": "Cancelar",
          "submit": "Embaralhar",
        },
        "unshuffle_playlist": {
          "title": "Desembaralhar playlist",
          "message": "Tem certeza de que deseja desembaralhar a lista de reprodução?",
          "cancel": "Cancelar",
          "submit": "Desembaralhar",
        },
        "restart_playlist": {
          "title": "Reiniciar playlist",
          "message": "Tem certeza deque deseja reiniciar a lista de reprodução?",
          "cancel": "Cancelar",
          "submit": "Reiniciar",
        }
      },
      "upload_prevent_unload_message": "Sair desta página cancelará os uploads pendentes. Você deseja sair mesmo assim?",
      "notifier": {
        "playlist_restarted": "Playlist reiniciada",
        "track_deleted": "Faixa excluída",
        "deleting_n_tracks": "Excluindo @n faixas",
        "n_tracks_deleted": "@n faixas excluídas",
        "playlist_unshuffled": "Playlist desembaralhada",
        "playlist_shuffled": "Playlist embaralhada",
      }
    },

    "station/broadcast": {
      "head": {
        "title": "Transmitir",
      },
      "title": "Transmitir",
      "icecast_settings": "Configurações do Icecast",
      "fields": {
        "address": "Endereço",
        "port": "Porta",
        "mountpoint": "Ponto de montagem",
        "username": "Usuário",
        "password": "Senha",
        "encoding": "Formato",
      },
      "encoding_or": "ou",
      "password_reset": "Redefinir",
      "links": {
        "title": "URLs de transmissão",
        "main": "PRINCIPAL",
      },
      "notifier": {
        "copied_to_clipboard": "Copiado para a área de transferência",
        "mount_password_reset": "Senha redefinida",
      },
      "dialogs": {
        "reset_password": {
          "title": "Redefinir a senha do ponto de montagem",
          "message": "Tem certeza de que deseja redefinir a senha do ponto de montagem?",
          "cancel": "Cancelar",
          "submit": "Redefinir senha",
        }
      }
    },

    "station/settings": {
      "head": {
        "title": "Configurações",
      },
      "title": "Configurações",
      "actions": {
        "title": "Ações",
        "delete_station": "Excluir estação",
      },
      "validate": {
        "station_name": "O nome da estação não coincide",
      },
      "notifier": {
        "station_deleted": "Estação excluída",
      },
      "dialogs": {
        "delete_station": {
          "title": "Excluir estação @name",
          "message_html": "A exclusão de uma estação é uma ação permanente, você não poderá acessar novamente as informações da estação, então certifique-se de que deseja prosseguir.<br /><br />Se você realmente deseja excluir a estação @name, insira o nome da estação no campo a seguir: <b>@name</b><br />",
          "field_label": "Nome da estação",
          "cancel": "Cancelar",
          "submit": "Excluir",
        }
      }
    },

    "me": {
      "title": "Perfil",
      "fields": {
        "email": "Seu email",
        "first_name": "Seu nome",
        "last_name": "Seu sobrenome",
        "phone": "Seu telefone",
        "current_password": "Senha atual",
        "new_password": "Nova senha",
        "confirm_password": "Confirmar senha",
        "language": "Idioma preferido",
      },
      "submit": {
        "profile": "Salvar",
        "password": "Salvar",
      },
      "change_password": {
        "title": "Alterar sua senha",
      },
      "more": {
        "title": "Mais",
        "connected_devices": "Dispositivos conectados",
      },
      "notifier": {
        "no_changes": "Sem alterações para salvar",
        "profile_updated": "Perfil atualizado",
        "password_updated": "Senha atualizada",
      }
    },

    "me/devices": {
      "head": {
        "title": "Dispositivos",
      },
      "title": "Dispositivos conectados",
      "note": "O mesmo dispositivo pode aparecer mais de uma vez nesta lista. Os dispositivos serão desconectados após 7 dias sem atividade.",
      "dialogs": {
        "disconnect": {
          "title": "Desconectar dispositivo",
          "message": "Esta ação é permanente.",
          "cancel": "Cancelar",
          "submit": "Desconectar",
        },
      },

      "notifier": {
        "device_disconnected": "Dispositivo desconectado",
      },

      "device": {
        "browser": "Navegador",
        "os": "Sistema",
        "ip": "IP",
        "last_used": "Usado pela última vez",
        "connected": "Conectado",
        "unkown": "Desconhecido",
        "tooltips": {
          "disconnect": "Desconectar",
        }
      }
    },

    "account/members": {
      "head": {
        "title": "Membros"
      },
      "title": "Membros",

      "no_owner_message_p1": "Esta seção está disponível apenas para os administradores da conta.",
      "no_owner_message_p2": "Entre em contato com os administradores da conta se precisar convidar pessoas para participar desta conta.",

      "Pending_invitations": "Convites pendentes",
      "no_pending_invitations_message": "Não há convites pendentes",
      "invite_btn_text": "Convidar pessoas",

      "validate": {
        "user_account_exists": "O usuário com email @email já faz parte desta conta",
      },

      "notifier": {
        "invitation_sent": "Convite enviado",
        "member_access_revoked": "Acesso do membro revogado",
        "member_role_changed": "Função de acesso do membro atualizada",    
      },

      "actions": {
        "set_role_to": "Definir função para @role",
        "revoke_access": "Revogar acesso",
        "delete": "Excluir",
      },

      "dialogs": {
        "invite": {
          "title": "Convide pessoas para participar desta conta com a função @role",
          "submit": "Convidar",
          "Email": "Email",
        }
      }
    },

    "email_invitation": {
      "head_page_title": {
        "not_found": "Convite não encontrado",
        "expired": "O convite expirou",
        "accepted": "O convite já foi aceito",
        "rejected": "O convite já foi rejeitado",
        "ok": "Convite pendente",
      },

      "error_message": {
        "not_found": "O link que você usou para acessar esta página não existe ou foi removido",
        "expired": "O convite expirou, entre em contato com os administradores da conta para que eles enviem um novo convite",
        "accepted": "O convite já foi aceito",
        "rejected": "O convite já foi rejeitado, se foi um erro, entre em contato com os administradores da conta para que eles enviem um novo convite",
      },

      "description": {
        "with_sender_name_html": "<b>@sender</b> está te convidando para se juntar a <b>@account</b> no Openstream.",
        "without_sender_name_html": "Você foi convidado para se juntar a <b>@account</b> no Openstream",
      },

      "login_as_btn_html": "Entre como <b>@email</b> para aceitar o convite",

      "form": {
        "fields": {
          "first_name": "Seu nome",
          "last_name": "Seu sobrenome",
          "email": "Seu email",
          "password": "Senha",
          "confirm_password": "Confirmar senha",
        },
        "pre_message_html": "Para <b>aceitar</b> o convite, preencha o formulário.",
        "title": "Registrar-me",
        "submit": "Enviar",
      },

      "notifier": {
        "accept_error": "Houve um erro ao aceitar o convite: @error"
      }
    },

    "me/invitations": {
      "head": {
        "title": "Convites pendentes",
      },
      "title": "Convites pendentes",

      "no_items_message": "Você não tem convites pendentes",

      "notifier": {
        "accept_error": "Houve um erro ao aceitar o convite: @error",
        "accepted": "Convite aceito",
        "rejected": "Convite rejeitado",
      },

      "actions": {
        "reject": "Rejeitar",
        "accept": "Aceitar",
      },

      "item_message_with_sender_html": "<b>@sender</b> está te convidando para se juntar a <b>@account</b>",
      "item_message_without_sender_html": "Você foi convidado para se juntar a <b>@account</b>",

      "dialogs": {
        "reject": {
          "title": "Rejeitar convite",
          "message": "Tem certeza de que deseja rejeitar o convite?",
          "cancel": "Cancelar",
          "reject": "Rejeitar convite",
        }
      }
    }
  },

  "user_menu": {
    "profile": "Perfil",
    "invitations": "Convites",
    "accounts": "Contas",
    "stations": "Estações",
    "sign_out": "Sair",
  }
}

export default locale;