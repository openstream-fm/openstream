/// file: studio.fr.ts
import stats_map from "../share/stats-map/stats-map.fr.js";
import validate from "../share/validate/validate.fr.js";
import countries from "../share/countries/countries.fr.js";
import langs from "../share/langs/langs.fr.js";
import type_of_content from "../share/type-of-content/type-of-content.fr.js";
import analytics from "../share/analytics/analytics.fr.js";
import payments from "../share/payments/payments.fr.js";
import station_profile from "../share/station-profile/station-profile.fr.js";
import misc from "../misc/misc.fr.js";
import language from "../share/language/language.fr.js";

const locale: import("./studio.locale.js").StudioLocale = {

  "lang": "fr",
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

  "copy_to_clipboard": "Copier dans le presse-papiers",
  "show_password": "Afficher le mot de passe",
  "hide_password": "Masquer le mot de passe",

  "prevent_unload_message":  "Si vous quittez cette page, les modifications que vous avez apportées seront perdues. Voulez-vous quitter la page quand même?",

  "drawer": {
    "account_selector": {
      "see_all_accounts": "Voir tous les comptes",
    },
    "dashboard": "Tableau de bord",
    "stations": "Stations",
    "members": "Membres",
    "analytics": "Analytique",
  },

  "limits": {
    "of": "de",
    "stations": "Stations",
    "listeners": "Auditeurs",
    "transfer": "Transfert",
    "storage": "Stockage",
  },

  "dialogs": {
    "delete": {
      "default_message": "Cette action est permanente.",
      "cancel": "Annuler",
      "delete": "Supprimer"
    }
  },

  "station_nav": {
    "dashboard": "Tableau de bord",
    "profile": "Profil",
    "playlist": "Playlist",
    "broadcast": "Diffuser",
    "settings": "Paramètres",
  },

  "plan_selector": {
    "price": {
      "per_month": "par mois",
      "n_per_month": "@n / mois",
    },

    "unlimited": "Illimité",

    "trial": {
      "30_day": "30 jours",
      "free_trial": "d'essai gratuit",
      "tooltip": "Vous ne serez pas facturé jusqu'à la fin de votre essai, et vous pouvez annuler à tout moment"
    },

    "features": {
      "station": "Station",
      "stations": "Stations",
      "listeners": "Auditeurs",
      "transfer": "Transfert",
      "storage": "Stockage",
      "staff": "utilisateurs",
      "auto_dj": "Auto DJ",
      "stats": "Statistiques avancées",
      "android_app": "Application pour Android",
    },

    "tooltips": {
      "one_station": "Vous ne pouvez créer qu'une seule station avec ce plan",
      "n_stations": "Jusqu'à @n stations différentes",
      "listeners": "Jusqu'à @n auditeurs simultanés",
      "transfer": "Avec @tb TB de transfert mensuel, vous aurez la capacité de diffuser environ @hours heures d'audio",
      "storage": "@gb GB de stockage pour la musique ou les anciens épisodes",
      "staff": "Vous pouvez ajouter des utilisateurs pour toute votre équipe sans limite",
      "auto_dj": "Diffusez à partir d'une playlist lorsque vous êtes hors ligne",
      "stats": "Statistiques historiques et en direct, sachez qui écoute vos stations",
      "android_app": "Une application pour Android avec votre marque et vos stations, disponible dans le monde entier via Google Play",
    }
  },

  "pages": {

    "error": {
      "retry": "Réessayer",
      "home": "Aller à l'accueil",
      "default_message": "Une erreur s'est produite",
      "offline": {
        "head": {
          "title": "Hors ligne",
        },
        "title": "Il semble que vous soyez hors ligne",
        "text": "Un accès à Internet est nécessaire pour utiliser @app_name",
      }
    },

    "login": {
      "head": {
        "title": "Se connecter",
      },
      "title": "Se connecter",
      "fields": {
        "email": "Email",
        "password": "Mot de passe",
      },
      "links": {
        "forgot": "Mot de passe oublié?",
        "new_user": "Nouvel utilisateur?",
        "sign_up": "S'inscrire",
      },
      "submit": "Se connecter"
    },

    "recover": {
      "head": {
        "title": "Récupérer votre compte",
      },
      "title": "Récupérer",
      "comment": "Nous vous enverrons un email pour que vous puissiez récupérer votre compte",
      "sent_message_html": "Nous avons envoyé un email à <b>@email</b> avec des instructions pour continuer",
      "links": {
        "login": "Retour à la connexion",
      },
      "submit": "Envoyer",
    },

    "plans": {
      "head": {
        "title": "Plans et tarifs",
      },
      "title_1": "En direct dans 3... 2... 1...",
      "title_2": "Commencez votre station en moins de 60 secondes.",
      "title_3": "Vous ne serez pas facturé jusqu'à la fin de votre essai. Et vous pouvez annuler à tout moment.",
      "plan_selector": {
        "select_btn_label": "Commencer l'essai",
      }
    },

    "register": {
      "head": {
        "title": "S'inscrire",
      },
      "title": "Commencez votre essai gratuit",
      "plan": {
        "selected_plan": "Plan sélectionné",
        "n_per_month": "@n / mois",
        "limits": {
          "station": "Station",
          "stations": "Stations",
          "listeners": "Auditeurs",
          "transfer": "Transfert",
          "storage": "Stockage",
        },
        "links": {
          "plans": "Retour aux plans et tarifs"
        }
      },
      "form": {
        "title": "Parlez-nous de vous",
        "account_name_comment": "Si vous créez un compte pour une organisation, vous pouvez remplir ce champ avec le nom de l'organisation",
        "fields": {
          "first_name": "Votre prénom",
          "last_name": "Votre nom",
          "account_name": "Un nom pour votre compte",
          "phone": "Votre téléphone",
          "email": "Votre email",
          "password": "Votre mot de passe",
          "confirm_password": "Confirmez votre mot de passe",
        },
        "next": "Suivant",
      },
      "pay": {
        "title": "Détails de paiement",
        "message": "Vous ne serez pas facturé jusqu'à la fin de votre essai gratuit de 30 jours et vous pouvez annuler à tout moment."
      },

      "back": "Retour à l'étape précédente",

      "verification": {
        "title": "Entrez le code de vérification",
        "message_html": "Nous avons envoyé un code de vérification à <b>@email</b>",
        "submit": "Envoyer",
      },
      "links": {
        "login_comment": "Vous avez déjà un compte?",
        "login_link": "Se connecter",
      }
    },

    "user_recovery": {
      "head_page_title": {
        "expired": "Le lien a expiré",
        "used": "Le lien a déjà été utilisé",
        "not_found": "Lien non trouvé",
        "ok": "Réinitialisez votre mot de passe",
      },
      "fields": {
        "email": "Email",
        "password": "Nouveau mot de passe",
        "confirm_password": "Confirmez le mot de passe",
      },
      "error": {
        "used_message_html": "Le lien que vous avez utilisé pour accéder à cette page a déjà été utilisé.<br /> Créez un nouveau lien depuis la @user_recovery_page",
        "expired_message_html": "Le lien que vous avez utilisé pour accéder à cette page a expiré.<br /> Créez un nouveau lien depuis la @user_recovery_page",
        "not_found_message_html": "Le lien que vous avez utilisé pour accéder à cette page n'existe pas.<br /> Créez un nouveau lien depuis la @user_recovery_page",
        "user_recovery_page": "page de récupération",
      },
      "submit": "Envoyer",
      "notifier": {
        "password_updated": "Mot de passe mis à jour",
      }
    },

    "accounts": {
      "head": {
        "title": "Comptes",
      },
      "title": "Choisissez un compte",
      "create_new_account": "créer un nouveau compte",
      "or": "ou",
      "no_items_message_html": "Vous n'avez pas encore de compte de diffusion.<br/>Pour commencer à diffuser, créez votre compte de diffusion.",
      "no_items_create": "Créer mon compte de diffusion",
    },

    "accounts/create_account": {
      "head": {
        "title": "Choisissez un plan",
      },
      "title": "Choisissez un plan pour votre nouveau compte",
      "select": "Sélectionner",
    },

    "accounts/create_account/plan": {
      "head": {
        "title": "Créer un compte émetteur",
      },
      "title": "Créer un compte émetteur",
      "plan": {
        "title": "Plan sélectionné",
        "n_per_month": "@n / mois",
        "station": "Station",
        "stations": "Stations",
        "listeners": "Auditeurs",
        "transfer": "Transfert",
        "storage": "Stockage",
        "back": "Retour aux plans et tarifs",
      },
      "form": {
        "title": "Parlez-nous du nouveau compte",
        "fields": {
          "account_name":"Un nom pour votre nouveau compte",
          "account_name_message": "Si vous créez un compte pour une organisation, vous pouvez remplir ce champ avec le nom de l'organisation",
        },
        "submit": "Créer",
        "next": "Suivant",
        "pay": {
          "title": "Détails du paiement",
        },
        "back": "Retour à l'étape précédente",
      }
    },

    "account/dashboard": {
      "edit": {
        "tooltip": "Modifier",
        "dialog": {
          "field_label": "Nom du compte",
          "title": "Modifiez le nom de votre compte",
          "save": "Enregistrer",
        }
      },

      "stats_map": {
        "all_stations": "Toutes les stations",
      },

      "station_item": {
        "on_air": "ON AIR",
        "off_air": "OFF AIR",
        "playlist": "Playlist",
        "live": "En direct",
      }
    },

    "account/analytics": {
      "head": {
        "title": "Analytique",
      },
      "title": "Analytique",
    },

    "stations": {
      "head": {
        "title": "Stations",
      },
      "title": "Choisissez une station",
      "create_new_station": "créez une nouvelle station",
      "or": "ou",
      "no_items_message_html": "Ce compte n'a pas encore de stations.<br />Pour commencer à diffuser, créez une nouvelle station.",
      "no_items_create": "Créer une station",
    },

    "stations/create_station": {
      "head": {
        "title": "Créer une station"
      },
      "title": "Créer une station",
      "submit": "Créer la station",
      "notifier": {
        "station_created": "Nouvelle station créée",
      }
    },

    "station/dashboard": {
      "on_air": "ON AIR",
      "off_air": "OFF AIR",
      "playlist": "Playlist",
      "live": "En direct",
      "preview": "Aperçu",
      "broadcast": "Diffuser",
      "aria_pause": "Pause",
      "aria_play": "Lecture",
    },

    "station/profile": {
      "head": {
        "title": "Profil",
      },
      "title": "Profil",
      "submit": "Enregistrer",
      "notifier": {
        "no_changes": "Aucun changement à enregistrer",
        "station_updated": "Station mise à jour",

      }
    },

    "station/playlist": {
      "head": {
        "title": "Playlist",
      },
      "title": "Playlist",
      "explain_html": "Créez une liste de musique ou d'anciens épisodes pour garder votre station active 24x7<br /> Lorsque vous n'êtes pas connecté ou que vous ne diffusez pas en direct, <b>Playlist</b> prendra automatiquement le contrôle.",
      "upload": "Télécharger",
      "browse": "Parcourir",
      "upload_files": "Télécharger des fichiers",
      "tracks_title": "Pistes",
      "track": "piste",
      "tracks": "pistes",
      "actions": {
        "restart_playlist": "Redémarrer la playlist",
        "shuffle_playlist": "Mélanger la playlist",
        "unshuffle_playlist": "Démélanger la playlist",
        "drag_to_rearrange": "Faites glisser pour réorganiser",
        "edit": "Modifier",
        "delete": "Supprimer",
      },
      "columns": {
        "title": "Titre",
        "artist": "Artiste",
        "album": "Album",
        "duration": "Durée",
      },
      "selection": {
        "one_track_selected": "1 piste sélectionnée",
        "n_tracks_selected": "@n pistes sélectionnées",
        "delete_selected": "Supprimer les éléments sélectionnés",
        "select_all": "Tout sélectionner",
        "unselect_all": "Tout déselectionner",
      },
      "uploading": {
        "success": "Téléchargement réussi",
        "waiting": "En attente",
        "in_progress": "En cours...",
        "retry": "Réessayer",
        "clear_done": "Masquer les éléments terminés",
      },
      "dialogs": {
        "delete_track": {
          "title": "Supprimer la piste @name"
        },
        "delete_tracks": {
          "title": "Supprimer @n pistes",
        },
        "edit_track": {
          "title": "Modifier la piste @name",
          "fields": {
            "title": "Titre",
            "artist": "Artiste",
            "album": "Album",
          },
          "cancel": "Annuler",
          "save": "Enregistrer",
        },
        "shuffle_playlist": {
          "title": "Mélanger la playlist",
          "message": "Êtes-vous sûr de vouloir mélanger aléatoirement la liste de lecture?",
          "cancel": "Annuler",
          "submit": "Mélanger",
        },
        "unshuffle_playlist": {
          "title": "Démélanger la playlist",
          "message": "Êtes-vous sûr de vouloir démélanger la liste de lecture?",
          "cancel": "Annuler",
          "submit": "Démélanger",
        },
        "restart_playlist": {
          "title": "Redémarrer la playlist",
          "message": "Êtes-vous sûr de vouloir redémarrer la liste de lecture?",
          "cancel": "Annuler",
          "submit": "Redémarrer",
        }
      },
      "upload_prevent_unload_message": "Quitter cette page annulera les téléchargements en attente. Voulez-vous quitter quand même?",
      "notifier": {
        "playlist_restarted": "Playlist redémarrée",
        "track_deleted": "Piste supprimée",
        "deleting_n_tracks": "Suppression de @n pistes",
        "n_tracks_deleted": "@n pistes supprimées",
        "playlist_unshuffled": "Playlist démélangée",
        "playlist_shuffled": "Playlist mélangée",
      }
    },

    "station/broadcast": {
      "head": {
        "title": "Diffuser",
      },
      "title": "Diffuser",
      "icecast_settings": "Configuration d'Icecast",
      "fields": {
        "address": "Adresse",
        "port": "Port",
        "mountpoint": "Point de montage",
        "username": "Nom d'utilisateur",
        "password": "Mot de passe",
        "encoding": "Format",
      },
      "encoding_or": "ou",
      "password_reset": "Réinitialiser",
      "links": {
        "title": "URLs de diffusion",
        "main": "PRINCIPAL",
      },
      "notifier": {
        "copied_to_clipboard": "Copié dans le presse-papiers",
        "mount_password_reset": "Mot de passe réinitialisé",
      },
      "dialogs": {
        "reset_password": {
          "title": "Réinitialiser le mot de passe du point de montage",
          "message": "Êtes-vous sûr de vouloir réinitialiser le mot de passe du point de montage?",
          "cancel": "Annuler",
          "submit": "Réinitialiser le mot de passe",
        }
      }
    },

    "station/settings": {
      "head": {
        "title": "Paramètres",
      },
      "title": "Paramètres",
      "actions": {
        "title": "Actions",
        "delete_station": "Supprimer la station",
      },
      "validate": {
        "station_name": "Le nom de la station ne correspond pas",
      },
      "notifier": {
        "station_deleted": "Station supprimée",
      },
      "dialogs": {
        "delete_station": {
          "title": "Supprimer la station @name",
          "message_html": "La suppression d'une station est une action permanente, vous ne pourrez plus accéder aux informations de la station, assurez-vous donc d'être sûr de procéder.<br /><br />Si vous voulez vraiment supprimer la station @name, saisissez le nom de la station dans le champ suivant: <b>@name</b><br />",
          "field_label": "Nom de la station",
          "cancel": "Annuler",
          "submit": "Supprimer",
        }
      }
    },

    "me": {
      "title": "Profil",
      "fields": {
        "email": "Votre email",
        "first_name": "Votre prénom",
        "last_name": "Votre nom",
        "phone": "Votre téléphone",
        "current_password": "Mot de passe actuel",
        "new_password": "Nouveau mot de passe",
        "confirm_password": "Confirmer le mot de passe",
        "language": "Langue préférée",
      },
      "submit": {
        "profile": "Enregistrer",
        "password": "Enregistrer",
      },
      "change_password": {
        "title": "Changez votre mot de passe",
      },
      "more": {
        "title": "Plus",
        "connected_devices": "Appareils connectés",
      },
      "notifier": {
        "no_changes": "Aucun changement à enregistrer",
        "profile_updated": "Profil mis à jour",
        "password_updated": "Mot de passe mis à jour",
      }
    },

    "me/devices": {
      "head": {
        "title": "Appareils",
      },
      "title": "Appareils connectés",
      "note": "Le même appareil peut apparaître plusieurs fois dans cette liste. Les appareils seront déconnectés après 7 jours d'inactivité.",
      "dialogs": {
        "disconnect": {
          "title": "Déconnecter l'appareil",
          "message": "Cette action est permanente.",
          "cancel": "Annuler",
          "submit": "Déconnecter",
        },
      },

      "notifier": {
        "device_disconnected": "Appareil déconnecté",
      },

      "device": {
        "browser": "Navigateur",
        "os": "Système",
        "ip": "IP",
        "last_used": "Dernière utilisation",
        "connected": "Connecté",
        "unkown": "Inconnu",
        "tooltips": {
          "disconnect": "Déconnecter",
        }
      }
    },

    "account/members": {
      "head": {
        "title": "Membres"
      },
      "title": "Membres",

      "no_owner_message_p1": "Cette section est disponible uniquement pour les administrateurs du compte.",
      "no_owner_message_p2": "Contactez les administrateurs du compte si vous avez besoin d'inviter des personnes à participer à ce compte.",

      "Pending_invitations": "Invitations en attente",
      "no_pending_invitations_message": "Aucune invitation en attente",
      "invite_btn_text": "Inviter des personnes",

      "validate": {
        "user_account_exists": "L'utilisateur avec l'email @email fait déjà partie de ce compte",
      },

      "notifier": {
        "invitation_sent": "Invitation envoyée",
        "member_access_revoked": "Accès membre révoqué",
        "member_role_changed": "Rôle d'accès membre mis à jour",
      },

      "actions": {
        "set_role_to": "Définir le rôle à @role",
        "revoke_access": "Révoquer l'accès",
        "delete": "Supprimer",
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
  },

  "user_menu": {
    "profile": "Profil",
    "invitations": "Invitations",
    "accounts": "Comptes",
    "stations": "Stations",
    "sign_out": "Se déconnecter",
  }
}

export default locale;