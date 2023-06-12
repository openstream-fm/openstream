const locale = {
  "pages": {
    "account/members": {
      "head": {
        "title": "Members"
      },
      "title": "Members",

      "no_owner_message_p1": "This section is only available for account administrators.",
      "no_owner_message_p2": "Contact the account administrators if you need to invite people to this account.",

      "Pending_invitations": "Pending invitations",
      "no_pending_invitations_message": "There are no pending invitations",
      "invite_btn_text": "Invite people",

      "validate": {
        "user_account_exists": "The user with email @email is already a member of the account",
      },

      "notifier": {
        "invitation_sent": "Invitation sent",
      },

      "dialogs": {
        "invite": {
          "title": "Invite people to manage this account with @role role",
          "submit": "Invite",
          "Email": "Email",
        }
      }
    },

    "email_invitation": {
      "head_page_title": {
        "not_found": "Invitation not found",
        "expired": "Invitation has expired",
        "accepted": "Invitation already accepted",
        "rejected": "Invitation already rejected",
        "ok": "Pending invitation",
      },

      "error_message": {
        "not_found": "The link used to access this page has doesn't exist or has been deleted",
        "expired": "The invitation has expired, ask the administrators of the account to send a new invitation",
        "accepted": "This invitation has already been accepted",
        "rejected": "This invitation has already been rejected, if that was an error, ask the administrators of the account to send a new invitation",
      },

      "description": {
        "with_sender_name_html": "<b>@sender</b> has invited you to join <b>@account</b> at Openstream.",
        "without_sender_name_html": "You have been invited yo join <b>@account</b> at Openstream",
      },

      "login_as_btn_html": "Login as <b>@email</b> to accept the invitation",

      "form": {
        "fields": {
          "first_name": "Yout first name",
          "last_name": "Yout last name",
          "email": "Your email",
          "password": "Password",
          "confirm_password": "Confirm password",
        },
        "pre_message_html": "To <b>accept</b> the invitation, fill the form.",
        "title": "Sign up",
        "submit": "Submit",
      },

      "notifier": {
        "accept_error": "There was an error accepting the invitaiton: @error"
      }
    },

    "me/invitations": {
      "head": {
        "title": "Pending invitations",     
      },
      "title": "Pending invitations",
      
      "no_items_message": "You don't have any pending invitations",

      "notifier": {
        "accept_error": "There was an error accepting the invitation: @error",
        "accepted": "Invitation accepted",
        "rejected": "Invitation rejected",
      },

      "actions": {
        "reject": "Reject",
        "accept": "Accept",
      },

      "item_message_with_sender_html": "<b>@sender</b> has invited you to join <b>@account</b>",
      "item_message_without_sender_html":  "You have been invited to join <b>@account</b>",
      
      "dialogs": {
        "reject": {
          "title": "Reject invitation",
          "message": "Are you sure you want to reject the invitation?",
          "cancel": "Cancel",
          "reject": "Reject invitation",
        }
      }
    }
  },
}

export default locale;