/// file: wip.it.ts
const locale: typeof import("./wip.en").default = {
  "pages": {
    "account/members": {
      "head": {
        "title": "Membri"
      },
      "title": "Membri",

      "no_owner_message_p1": "Questa sezione è disponibile solo per gli amministratori dell'account",
      "no_owner_message_p2": "Contatta gli amministratori dell'account se hai bisogno di invitare persone a partecipare a questo account.",

      "Pending_invitations": "Inviti in sospeso",
      "no_pending_invitations_message": "Non ci sono inviti in sospeso",
      "invite_btn_text": "Invita persone",

      "validate": {
        "user_account_exists": "L'utente con email @email fa già parte di questo account",
      },

      "notifier": {
        "invitation_sent": "Invito inviato",
      },

      "dialogs": {
        "invite": {
          "title": "Invita persone a partecipare a questo account con ruolo @role",
          "submit": "Invita",
          "Email": "Email",
        }
      }
    },

    "email_invitation": {
      "head_page_title": {
        "not_found": "Invito non trovato",
        "expired": "L'invito è scaduto",
        "accepted": "L'invito è stato accettato",
        "rejected": "L'invito è stato rifiutato",
        "ok": "Invito in sospeso",
      },

      "error_message": {
        "not_found": "Il link che hai usato per accedere a questa pagina non esiste più o è stato eliminato",
        "expired": "L'invito è scaduto, contatta gli amministratori dell'account per ricevere un nuovo invito",
        "accepted": "L'invito è stato accettato",
        "rejected": "L'invito è stato rifiutato, se è stato un errore, contatta gli amministratori dell'account per ricevere un nuovo invito",
      },

      "description": {
        "with_sender_name_html": "<b>@sender</b> ti sta invitando a unirti a <b>@account</b> su Openstream.",
        "without_sender_name_html": "Sei stato invitato a unirti a <b>@account</b> su Openstream",
      },

      "login_as_btn_html": "Accedi come <b>@email</b> per accettare l'invito",

      "form": {
        "fields": {
          "first_name": "Il tuo nome",
          "last_name": "Il tuo cognome",
          "email": "La tua email",
          "password": "Password",
          "confirm_password": "Conferma password",
        },
        "pre_message_html": "Per <b>accettare</b> l'invito, completa il modulo.",
        "title": "Registrati",
        "submit": "Invia",
      },

      "notifier": {
        "accept_error": "Si è verificato un errore nell'accettare l'invito: @error"
      }
    },

    "me/invitations": {
      "head": {
        "title": "Inviti in sospeso",
      },
      "title": "Inviti in sospeso",

      "no_items_message": "Non hai inviti in sospeso",

      "notifier": {
        "accept_error": "Si è verificato un errore nell'accettare l'invito: @error",
        "accepted": "Invito accettato",
        "rejected": "Invito rifiutato",
      },

      "actions": {
        "reject": "Rifiuta",
        "accept": "Accetta",
      },

      "item_message_with_sender_html": "<b>@sender</b> ti sta invitando a unirti a <b>@account</b>",
      "item_message_without_sender_html": "Sei stato invitato a unirti a <b>@account</b>",

      "dialogs": {
        "reject": {
          "title": "Rifiuta invito",
          "message": "Sei sicuro di voler rifiutare l'invito?",
          "cancel": "Annulla",
          "reject": "Rifiuta invito",
        }
      }
    }
  }
}

export default locale;