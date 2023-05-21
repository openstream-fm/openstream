import stats_map from "../share/stats-map/stats-map.en"
import validate from "../share/validate/validate.en";


const locale = {

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "app_name": "Openstream Studio",

  "validate": validate,
  "stats_map": stats_map,

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

  "station_profile": {
    "titles": {
      "logo": "Logo",
      "profile_info": "Profile information",
      "contact_info": "Contact information",
      "social": "Social links",
      "apps": "App links",
    },
    "validation": {
      "logo_required": "The logo is required",
    },
    "upload_image": "Upload Image",
    "picture_requirement_labels": {
      "format": "Image formats accepted:",
      "size": "Minimum image size:",
      "file_size": "Maximum file size:",
      "square": "Image must be square",
    },

    "labels": {
      "name": "Name",
      "slogan": "Slogan",
      "description": "Description",
      "country": "Country",
      "type_of_content": "Type of content",
      "email": "Email",
      "phone": "Full phone number",
      "whatsapp": "Full WhatsApp number",
      "website": "Website URL",
      "twitter": "Twitter URL",
      "facebook": "Facebook URL",
      "instagram": "Instagram URL",
      "youtube": "Youtube URL",
      "twitch": "Twitch URL",
      "google_play": "Google Play URL",
      "app_store": "App Store URL"
    }
  },

  "plan_selector": {
    "price": {
      "per_month": "per month",
      "$_n_per_month": "$ @n / month",
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
        "$_n_price_per_month": "$ @n / month",
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
      "verification": {
        "title": "Enter the verification code",
        "message_html": "We sent you a verification code to <b>@email</b>",
        "back": "Back to form",
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
        "used_message_html": "The link you used to access this page is expired.<br /> Create a new link from the @user_recovery_page",
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

    "accounts.create_account": {
      "head": {
        "title": "Select a plan",
      },
      "title": "Select a plan for your nwe account",
      "select": "Select",
    },

    "account.dashboard": {
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

    "stations.create_station": {
      "create_a_stations": "Create a station",
      "notifier": {
        "station_created": "New station created",
      }
    },

    "station.dashboard": {
      "on_air": "ON AIR",
      "off_air": "OFF AIR",
      "playlist": "Playlist",
      "live": "Live",
      "preview": "Preview",
      "broadcast": "Broadcast",
      "aria_pause": "Pause",
      "aria_play": "Play",
    },

    "station.profile": {
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

    "station.playlist": {
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

    "station.broadcast": {
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
        "main": "Main",
      },
      "notifier": {
        "copied_to_clipboard": "Copied to clipboard",
        "mount_password_reset": "Mountpoint password reset",
      }
    },

    "station.settings": {
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
        "new_password": "New password",
        "confirm_password": "Confirm password",
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

    "me.devices": {
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
    }
  },

  "user_menu": {
    "profile": "Profile",
    "accounts": "Accounts",
    "stations": "Stations",
    "sign_out": "Sign out",
  }
}

export default locale;