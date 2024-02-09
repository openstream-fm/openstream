import stats_map from "../share/stats-map/stats-map.en.js"
import validate from "../share/validate/validate.en.js";
import countries from "../share/countries/countries.en.js";
import langs from "../share/langs/langs.en.js";
import type_of_content from "../share/type-of-content/type-of-content.en.js";
import analytics from "../share/analytics/analytics.en.js";
import payments from "../share/payments/payments.en.js";
import station_profile from "../share/station-profile/station-profile.en.js";
import misc from "../misc/misc.en.js";
import language from "../share/language/language.en.js";

const locale = {

  "lang": "en",
  "region": null as string | null,

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

  "copy_to_clipboard": "Copy to clipboard",
  "show_password": "Show password",
  "hide_password": "Hide password",

  "prevent_unload_message":  "If you leave this page your changes will be lost. Do you want to leave anyway?",

  "drawer": {
    "account_selector": {
      "see_all_accounts": "See all accounts",
    },
    "dashboard": "Dashboard",
    "stations": "Stations",
    "members": "Members",
    "analytics": "Analytics",
  },

  "limits": {
    "of": "of",
    "stations": "Stations",
    "listeners": "Listeners",
    "transfer": "Transfer",
    "storage": "Storage",
  },

  "dialogs": {
    "delete": {
      "default_message": "This action is permanent.",
      "cancel": "Cancel",
      "delete": "Delete"
    }
  },

  "station_nav": {
    "dashboard": "Dashboard",
    "profile": "Profile",
    "playlist": "Playlist",
    "broadcast": "Broadcast",
    "settings": "Settings",
  },

  "plan_selector": {
    "price": {
      "per_month": "per month",
      "n_per_month": "@n / month",
    },

    "unlimited": "Unlimited",

    "trial": {
      "30_day": "30 day",
      "free_trial": "Free Trial",
      "tooltip": "You won't be charged until your trial ends, and you can cancel any time"
    },

    "features": {
      "station": "Station",
      "stations": "Stations",
      "listeners": "Listeners",
      "transfer": "Bandwidth",
      "storage": "Storage",
      "staff": "Staff users",
      "auto_dj": "Auto DJ",
      "stats": "Advanced Stats",
      "android_app": "Android App",
    },

    "tooltips": {
      "one_station": "You can only create one station with this plan",
      "n_stations": "Up to @n different stations",
      "listeners": "Up to @n concurrent listeners",
      "transfer": "@tb TB of monthly transfer will give you around @hours listening hours",
      "storage": "@gb GB of storage for music or old episodes",
      "staff": "Add all the staff users that you want",
      "auto_dj": "Broadcast from a playlist when you're not online",
      "stats": "Advanced live and historical stats, see who's listening your stations",
      "android_app": "An Android application branded to your stations and available worldwide through Google Play",

    }
  },

  "pages": {

    "error": {
      "retry": "Retry",
      "home": "Take me to home",
      "default_message": "An error ocurred",
      "offline": {
        "head": {
          "title": "Offline",
        },
        "title": "Seems that you are offline",
        "text": "You need internet access to use @app_name",
      }
    },

    "login": {
      "head": {
        "title": "Sign in",
      },
      "title": "Sign in",
      "fields": {
        "email": "Email",
        "password": "Password",
      },
      "links": {
        "forgot": "Forgot you password?",
        "new_user": "New user?",
        "sign_up": "Sign up",
      },
      "submit": "Sign in"
    },

    "recover": {
      "head": {
        "title": "Recover your account",
      },
      "title": "Recover",
      "comment": "We'll send you an e-mail for you to recover access",
      "sent_message_html": "We sent you an email to <b>@email</b> with further instructions",
      "links": {
        "login": "Back to login",
      },
      "submit": "Send",
    },

    "plans": {
      "head": {
        "title": "Plans and Pricing",
      },
      "title_1": "Going live in 3... 2... 1...",
      "title_2": "Start your radio station in less than 60 seconds.",
      "title_3": "You won't be billed until the end of your trial. And you can cancel anytime.",
      "plan_selector": {
        "select_btn_label": "Start Trial",
      }
    },

    "register": {
      "head": {
        "title": "Sign up",
      },
      "title": "Start your trial",
      "plan": {
        "selected_plan": "Selected plan",
        "n_per_month": "@n / month",
        "limits": {
          "station": "Station",
          "stations": "Stations",
          "listeners": "Listeners",
          "transfer": "Bandwidth",
          "storage": "Storage",
        },
        "links": {
          "plans": "Back to plans and pricing"
        }
      },
      "form": {
        "title": "Tell us about yourself",
        "account_name_comment": "If you are creating an account for an organization, you can fill this field with the organization's name",
        "fields": {
          "first_name": "Your first name",
          "last_name": "Your last name",
          "account_name": "A name for your account",
          "phone": "Your phone number",
          "email": "Your email",
          "password": "Your password",
          "confirm_password": "Confirm your password",
        },
        "next": "Next",
      },
      
      "pay": {
        "title": "Payment details",
        "message": "You wont be charged until your 30 day trial ends and you can cancel anytime."
      },

      "back": "Back to the previous step",

      "verification": {
        "title": "Enter the verification code",
        "message_html": "We sent you a verification code to <b>@email</b>",
        "submit": "Submit",
      },
      "links": {
        "login_comment": "Already have an account?",
        "login_link": "Sign in",
      }
    },

    "user_recovery": {
      "head_page_title": {
        "expired": "Link has expired",
        "used": "Link already used",
        "not_found": "Link not found",
        "ok": "Reset your password",
      },
      "fields": {
        "email": "Email",
        "password": "New password",
        "confirm_password": "Confirm password",
      },
      "error": {
        "used_message_html": "The link you used to access this page has already been used.<br /> Create a new link from the @user_recovery_page",
        "expired_message_html": "The link you used to access this page doesn't has expired.<br /> Create a new link from the @user_recovery_page",
        "not_found_message_html": "The link you used to access this page doesn't exist anymore.<br /> Create a new link from the @user_recovery_page",
        "user_recovery_page": "user recovery page",
      },
      "submit": "Send",
      "notifier": {
        "password_updated": "Password updated",
      }
    },

    "accounts": {
      "head": {
        "title": "Accounts",
      },
      "title": "Select an account",
      "create_new_account": "create a new account",
      "or": "or",
      "no_items_message_html": "You don't have a broadcaster account yet.<br/>To start broadcasting, sign up for a broadcaster account.",
      "no_items_create": "Create my broadcaster account",
    },

    "accounts/create_account": {
      "head": {
        "title": "Select a plan",
      },
      "title": "Select a plan for your new account",
      "select": "Select",
    },

    "accounts/create_account/plan": {
      "head": {
        "title": "Create a broadcaster account",
      },
      "title": "Create a broadcaster account",
      "plan": {
        "title": "Selected plan",
        "n_per_month": "@n / month",
        "station": "Station",
        "stations": "Stations",
        "listeners": "Listeners",
        "transfer": "Bandwidth",
        "storage": "Storage",
        "back": "Back to plans and pricing",
      },
      "form": {
        "title": "Tell us about the new account",
        "fields": {
          "account_name":"A name for your new account",
          "account_name_message": "If you are creating an account for an organization, you can fill this field with the organization's name"
        },
        "submit": "Create",
        "next": "Next",
        "pay": {
          "title": "Payment details",
        },
        "back": "Back to the previous step",
      }
    },

    "account/dashboard": {
      "edit": {
        "tooltip": "Edit",
        "dialog": {
          "field_label": "Account name",
          "title": "Edit your account name",
          "save": "Save",
        }
      },

      "stats_map": {
        "all_stations": "All stations",
      },

      "station_item": {
        "on_air": "ON AIR",
        "off_air": "OFF AIR",
        "playlist": "Playlist",
        "live": "Live",
      }
    },

    "account/analytics": {
      "head": {
        "title": "Analytics",
      },
      "title": "Analytics",
    },

    "stations": {
      "head": {
        "title": "Stations",
      },
      "title": "Select a station",
      "create_new_station": "create a new station",
      "or": "or",
      "no_items_message_html": "This account doesn't have stations yet.<br />To start broadcasting, create a new station.",
      "no_items_create": "Create a station",
    },

    "stations/create_station": {
      "head": {
        "title": "Create a station"
      },
      "title": "Create a station",
      "submit": "Create station",
      "notifier": {
        "station_created": "New station created",
      }
    },

    "station/dashboard": {
      "on_air": "ON AIR",
      "off_air": "OFF AIR",
      "playlist": "Playlist",
      "live": "Live",
      "preview": "Preview",
      "broadcast": "Broadcast",
      "aria_pause": "Pause",
      "aria_play": "Play",
    },

    "station/profile": {
      "head": {
        "title": "Station Profile",
      },
      "title": "Profile",
      "submit": "Save",
      "notifier": {
        "no_changes": "No changes to save",
        "station_updated": "Station updated",

      }
    },

    "station/playlist": {
      "head": {
        "title": "Playlist",
      },
      "title": "Playlist",
      "explain_html": "Create a playlist of music or old episodes to keep your station up 24x7.<br /> When connection is lost or you are not broadcasting, <b>Playlist</b> will automatically take over.",
      "upload": "Upload",
      "browse": "Browse",
      "upload_files": "Upload files",
      "tracks_title": "Tracks",
      "track": "track",
      "tracks": "tracks",
      "actions": {
        "restart_playlist": "Restart playlist",
        "shuffle_playlist": "Shuffle playlist",
        "unshuffle_playlist": "Unshuffle playlist",
        "drag_to_rearrange": "Drag to rearrange",
        "edit": "Edit",
        "delete": "Delete",
      },
      "columns": {
        "title": "Title",
        "artist": "Artist",
        "album": "Album",
        "duration": "Duration",
      },
      "selection": {
        "one_track_selected": "1 track selected",
        "n_tracks_selected": "@n tracks selected",
        "delete_selected": "Delete selected",
        "select_all": "Select all",
        "unselect_all": "Unselect all",
      },
      "uploading": {
        "success": "Uploaded successfully",
        "waiting": "Waiting",
        "in_progress": "In progress...",
        "retry": "Retry",
        "clear_done": "Clear done items",
      },
      "dialogs": {
        "delete_track": {
          "title": "Delete track @name"
        },
        "delete_tracks": {
          "title": "Delete @n tracks",
        },
        "edit_track": {
          "title": "Edit track @name",
          "fields": {
            "title": "Title",
            "artist": "Artist",
            "album": "Album",
          },
          "cancel": "Cancel",
          "save": "Save",
        },
        "shuffle_playlist": {
          "title": "Shuffle playlist",
          "message": "Are you sure you want to randomly shuffle the entire playlist?",
          "cancel": "Cancel",
          "submit": "Shuffle",
        },
        "unshuffle_playlist": {
          "title": "Unshuffle playlist",
          "message": "Are you sure you want to unshuffle the entire playlist?",
          "cancel": "Cancel",
          "submit": "Unshuffle",
        },
        "restart_playlist": {
          "title": "Restart playlist",
          "message": "Are you sure you want to restart the playlist?",
          "cancel": "Cancel",
          "submit": "Restart",
        }
      },
      "upload_prevent_unload_message": "Leaving this page will cancel pending uploads. Do you want to leave anyway?",
      "notifier": {
        "playlist_restarted": "Playlist restarted",
        "track_deleted": "Track deleted",
        "deleting_n_tracks": "Deleting @n tracks",
        "n_tracks_deleted": "@n tracks deleted",
        "playlist_unshuffled": "Playlist unshuffled",
        "playlist_shuffled": "Playlist shuffled",
      }
    },

    "station/broadcast": {
      "head": {
        "title": "Broadcast",
      },
      "title": "Broadcast",
      "icecast_settings": "Icecast Settings",
      "fields": {
        "address": "Address",
        "port": "Port",
        "mountpoint": "Mountpoint",
        "username": "Username",
        "password": "Password",
        "encoding": "Encoding",
      },
      "encoding_or": "or",
      "password_reset": "Reset",
      "links": {
        "title": "Stream URLs",
        "main": "MAIN",
      },
      "dialogs": {
        "reset_password": {
          "title": "Reset mount password",
          "message": "Are you sure you want to reset the mountpoint password?",
          "cancel": "Cancel",
          "submit": "Reset password",
        }
      },
      "notifier": {
        "copied_to_clipboard": "Copied to clipboard",
        "mount_password_reset": "Mountpoint password reset",
      }
    },

    "station/settings": {
      "head": {
        "title": "Settings",
      },
      "title": "Settings",
      "actions": {
        "title": "Actions",
        "delete_station": "Delete station",
      },
      "validate": {
        "station_name": "The station name doesn't match",
      },
      "notifier": {
        "station_deleted": "Station deleted",
      },
      "dialogs": {
        "delete_station": {
          "title": "Delete station @name",
          "message_html": "Deletion of a station is a permanent action, you won't be able to access the station's data again, so be sure of what you are doing.<br /><br />If you really want to delete the station @name type the name of the station in the following box: <b>@name</b><br />",
          "field_label": "Station name",
          "cancel": "Cancel",
          "submit": "Delete",
        }
      }
    },

    "me": {
      "title": "Profile",
      "fields": {
        "email": "Your email",
        "first_name": "Your first name",
        "last_name": "Your last name",
        "phone": "Your phone number",
        "current_password": "Current password",
        "new_password": "New password",
        "confirm_password": "Confirm password",
        "language": "Preferred language",
      },
      "submit": {
        "profile": "Save",
        "password": "Save",
      },
      "change_password": {
        "title": "Change your password",
      },
      "more": {
        "title": "More",
        "connected_devices": "Connected devices",
      },
      "notifier": {
        "no_changes": "No changes to save",
        "profile_updated": "Profile updated",
        "password_updated": "Password updated",
      }
    },

    "me/devices": {
      "head": {
        "title": "Devices",
      },
      "title": "Connected devices",
      "note": "The same device may appear more than once in this list. Devices will be disconnected after 7 days without usage.",
      "dialogs": {
        "disconnect": {
          "title": "Disconnect device",
          "message": "This action is permanent",
          "cancel": "Cancel",
          "submit": "Disconnect",
        },
      },

      "notifier": {
        "device_disconnected": "Device disconnected",
      },

      "device": {
        "browser": "Browser",
        "os": "System",
        "ip": "IP",
        "last_used": "Last used",
        "connected": "Connected",
        "unkown": "Unknown",
        "tooltips": {
          "disconnect": "Disconnect",
        }
      }
    },

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
        "member_access_revoked": "Member access revoked",
        "member_role_changed": "Member access role updated",
      },

      "actions": {
        "set_role_to": "Set role to @role",
        "revoke_access": "Revoke access",
        "delete": "Delete",
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

  "user_menu": {
    "profile": "Profile",
    "invitations": "Invitations",
    "accounts": "Accounts",
    "stations": "Stations",
    "sign_out": "Sign out",
  }
}

export default locale;