/// file: wip.pt.ts
const locale: typeof import("./wip.en").default = {
  "pages": {
    "account/members": {
      "head": {
        "title": "Membros"
      },
      "title": "Membros",

      "no_owner_message_p1": "Esta seção está disponível apenas para os administradores da conta",
      "no_owner_message_p2": "Entre em contato com os administradores da conta se precisar convidar pessoas para participar desta conta.",

      "Pending_invitations": "Convites pendentes",
      "no_pending_invitations_message": "Não há convites pendentes",
      "invite_btn_text": "Convidar pessoas",

      "validate": {
        "user_account_exists": "O usuário com email @email já faz parte desta conta",
      },

      "notifier": {
        "invitation_sent": "Convite enviado",
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
  }
}

export default locale;