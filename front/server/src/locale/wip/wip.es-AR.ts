const locale: typeof import("./wip.en").default = {
  "pages": {
    "account/members": {
      "head": {
        "title": "Miembros"
      },
      "title": "Miembros",

      "no_owner_message_p1": "Esta sección esta disponible solo para los administradores de la cuenta",
      "no_owner_message_p2": "Contactate con los administradores de la cuenta si necesitás invitar gente a participar en esta cuenta.",

      "Pending_invitations": "Invitaciones pendientes",
      "no_pending_invitations_message": "No hay invitaciones pendientes",
      "invite_btn_text": "Invitar personas",

      "validate": {
        "user_account_exists": "El usuario con email @email ya forma parte de esta cuenta",
      },

      "notifier": {
        "invitation_sent": "Invitación enviada",
      },

      "dialogs": {
        "invite": {
          "title": "Invitá personas a participar de este cuenta con rol @role",
          "submit": "Invitar",
          "Email": "Email",
        }
      }
    },

    "email_invitation": {
      "head_page_title": {
        "not_found": "Invitación no encontrada",
        "expired": "La invitación ha expirado",
        "accepted": "La invitatión ya fue aceptada",
        "rejected": "La invitación ya fue rechazada",
        "ok": "Invitación pendiente",
      },

      "error_message": {
        "not_found": "El link que usaste para acceder a esta página ya no existe o fue eliminado",
        "expired": "La invitación ha expirado, contactate con los administradores de la cuenta para que te envíen una nueva invitación",
        "accepted": "La invitación ya fue aceptada",
        "rejected": "La invitación ya fue rechazada, si fue un error, contáctate con los administradores de la cuenta para que te envíen una nueva invitación",
      },

      "description": {
        "with_sender_name_html": "<b>@sender</b> te está invitando a que te unas a <b>@account</b> en Openstream.",
        "without_sender_name_html": "Fuiste invitado a unirte a <b>@account</b> en Openstream",
      },

      "login_as_btn_html": "Ingresá como <b>@email</b> para aceptar la invitación",

      "form": {
        "fields": {
          "first_name": "Tu nombre",
          "last_name": "Tu apellido",
          "email": "Tu email",
          "password": "Constraseña",
          "confirm_password": "Confirmar contraseña",
        },
        "pre_message_html": "Para <b>aceptar</b> la invitación, completa el formulario.",
        "title": "Registarme",
        "submit": "Enviar",
      },

      "notifier": {
        "accept_error": "Hubo un error aceptando la invitacion: @error"
      }
    },

    "me/invitations": {
      "head": {
        "title": "Invitaciones pendientes",
      },
      "title": "Invitaciones pendientes",

      "no_items_message": "No tenés invitaciones pendientes",

      "notifier": {
        "accept_error": "Hubo un error aceptando la invtación: @error",
        "accepted": "Invitación aceptada",
        "rejected": "Invitación rechazada",
      },

      "actions": {
        "reject": "Rechazar",
        "accept": "Aceptar",
      },

      "item_message_with_sender_html": "<b>@sender</b> te esta invitando a que te unas a <b>@account</b>",
      "item_message_without_sender_html": "Fuiste invitado a unirte a <b>@account</b>",

      "dialogs": {
        "reject": {
          "title": "Rechazar invitación",
          "message": "¿Estás seguro de que querés rechazar la invitación?",
          "cancel": "Cancelar",
          "reject": "Rechazar invitación",
        }
      }
    }
  }
}

export default locale;