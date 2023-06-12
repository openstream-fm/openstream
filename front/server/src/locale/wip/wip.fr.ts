/// file: wip.fr.ts
const locale: typeof import("./wip.en").default = {
  "pages": {
    "account/members": {
      "head": {
        "title": "Membres"
      },
      "title": "Membres",

      "no_owner_message_p1": "Cette section est disponible uniquement pour les administrateurs du compte",
      "no_owner_message_p2": "Contactez les administrateurs du compte si vous avez besoin d'inviter des personnes à participer à ce compte.",

      "Pending_invitations": "Invitations en attente",
      "no_pending_invitations_message": "Aucune invitation en attente",
      "invite_btn_text": "Inviter des personnes",

      "validate": {
        "user_account_exists": "L'utilisateur avec l'email @email fait déjà partie de ce compte",
      },

      "notifier": {
        "invitation_sent": "Invitation envoyée",
      },

      "dialogs": {
        "invite": {
          "title": "Invitez des personnes à participer à ce compte avec le rôle @role",
          "submit": "Inviter",
          "Email": "Email",
        }
      }
    },

    "email_invitation": {
      "head_page_title": {
        "not_found": "Invitation non trouvée",
        "expired": "L'invitation a expiré",
        "accepted": "L'invitation a déjà été acceptée",
        "rejected": "L'invitation a déjà été refusée",
        "ok": "Invitation en attente",
      },

      "error_message": {
        "not_found": "Le lien que vous avez utilisé pour accéder à cette page n'existe plus ou a été supprimé",
        "expired": "L'invitation a expiré, contactez les administrateurs du compte pour qu'ils vous envoient une nouvelle invitation",
        "accepted": "L'invitation a déjà été acceptée",
        "rejected": "L'invitation a déjà été refusée, si c'était une erreur, contactez les administrateurs du compte pour qu'ils vous envoient une nouvelle invitation",
      },

      "description": {
        "with_sender_name_html": "<b>@sender</b> vous invite à rejoindre <b>@account</b> sur Openstream.",
        "without_sender_name_html": "Vous avez été invité à rejoindre <b>@account</b> sur Openstream",
      },

      "login_as_btn_html": "Connectez-vous en tant que <b>@email</b> pour accepter l'invitation",

      "form": {
        "fields": {
          "first_name": "Votre prénom",
          "last_name": "Votre nom",
          "email": "Votre email",
          "password": "Mot de passe",
          "confirm_password": "Confirmer le mot de passe",
        },
        "pre_message_html": "Pour <b>accepter</b> l'invitation, remplissez le formulaire.",
        "title": "M'inscrire",
        "submit": "Envoyer",
      },

      "notifier": {
        "accept_error": "Une erreur s'est produite lors de l'acceptation de l'invitation : @error"
      }
    },

    "me/invitations": {
      "head": {
        "title": "Invitations en attente",
      },
      "title": "Invitations en attente",

      "no_items_message": "Vous n'avez pas d'invitations en attente",

      "notifier": {
        "accept_error": "Une erreur s'est produite lors de l'acceptation de l'invitation : @error",
        "accepted": "Invitation acceptée",
        "rejected": "Invitation refusée",
      },

      "actions": {
        "reject": "Refuser",
        "accept": "Accepter",
      },

      "item_message_with_sender_html": "<b>@sender</b> vous invite à rejoindre <b>@account</b>",
      "item_message_without_sender_html": "Vous avez été invité à rejoindre <b>@account</b>",

      "dialogs": {
        "reject": {
          "title": "Refuser l'invitation",
          "message": "Êtes-vous sûr de vouloir refuser l'invitation ?",
          "cancel": "Annuler",
          "reject": "Refuser l'invitation",
        }
      }
    }
  }
}

export default locale;