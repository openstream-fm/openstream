/// file: wip.de.ts
const locale: typeof import("./wip.en").default = {
  "pages": {
    "account/members": {
      "head": {
        "title": "Mitglieder"
      },
      "title": "Mitglieder",

      "no_owner_message_p1": "Dieser Bereich ist nur für Kontoadministratoren verfügbar",
      "no_owner_message_p2": "Wenden Sie sich an die Kontoadministratoren, wenn Sie Personen einladen möchten, um an diesem Konto teilzunehmen.",

      "Pending_invitations": "Ausstehende Einladungen",
      "no_pending_invitations_message": "Keine ausstehenden Einladungen",
      "invite_btn_text": "Personen einladen",

      "validate": {
        "user_account_exists": "Der Benutzer mit der E-Mail-Adresse @email ist bereits Teil dieses Kontos",
      },

      "notifier": {
        "invitation_sent": "Einladung gesendet",
      },

      "dialogs": {
        "invite": {
          "title": "Laden Sie Personen ein, an diesem Konto mit der Rolle @role teilzunehmen",
          "submit": "Einladen",
          "Email": "E-Mail",
        }
      }
    },

    "email_invitation": {
      "head_page_title": {
        "not_found": "Einladung nicht gefunden",
        "expired": "Einladung abgelaufen",
        "accepted": "Einladung bereits angenommen",
        "rejected": "Einladung bereits abgelehnt",
        "ok": "Ausstehende Einladung",
      },

      "error_message": {
        "not_found": "Der Link, den Sie zum Aufrufen dieser Seite verwendet haben, existiert nicht mehr oder wurde gelöscht",
        "expired": "Die Einladung ist abgelaufen, wenden Sie sich an die Kontoadministratoren, damit sie Ihnen eine neue Einladung senden",
        "accepted": "Die Einladung wurde bereits angenommen",
        "rejected": "Die Einladung wurde bereits abgelehnt, wenn dies ein Fehler war, wenden Sie sich an die Kontoadministratoren, damit sie Ihnen eine neue Einladung senden",
      },

      "description": {
        "with_sender_name_html": "<b>@sender</b> lädt Sie ein, sich <b>@account</b> bei Openstream anzuschließen.",
        "without_sender_name_html": "Sie wurden eingeladen, sich <b>@account</b> bei Openstream anzuschließen",
      },

      "login_as_btn_html": "Melden Sie sich als <b>@email</b> an, um die Einladung anzunehmen",

      "form": {
        "fields": {
          "first_name": "Dein Vorname",
          "last_name": "Dein Nachname",
          "email": "Deine E-Mail",
          "password": "Passwort",
          "confirm_password": "Passwort bestätigen",
        },
        "pre_message_html": "Um die Einladung <b>anzunehmen</b>, füllen Sie das Formular aus.",
        "title": "Registrieren",
        "submit": "Absenden",
      },

      "notifier": {
        "accept_error": "Es gab einen Fehler beim Annehmen der Einladung: @error"
      }
    },

    "me/invitations": {
      "head": {
        "title": "Ausstehende Einladungen",
      },
      "title": "Ausstehende Einladungen",

      "no_items_message": "Sie haben keine ausstehenden Einladungen",

      "notifier": {
        "accept_error": "Es gab einen Fehler beim Annehmen der Einladung: @error",
        "accepted": "Einladung angenommen",
        "rejected": "Einladung abgelehnt",
      },

      "actions": {
        "reject": "Ablehnen",
        "accept": "Annehmen",
      },

      "item_message_with_sender_html": "<b>@sender</b> lädt Sie ein, sich <b>@account</b> anzuschließen",
      "item_message_without_sender_html": "Sie wurden eingeladen, sich <b>@account</b> anzuschließen",

      "dialogs": {
        "reject": {
          "title": "Einladung ablehnen",
          "message": "Sind Sie sicher, dass Sie die Einladung ablehnen möchten?",
          "cancel": "Abbrechen",
          "reject": "Einladung ablehnen",
        }
      }
    }
  }
}

export default locale;