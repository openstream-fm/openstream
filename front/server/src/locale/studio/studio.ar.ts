/// file: studio.ar.ts
import stats_map from "../share/stats-map/stats-map.ar";
import validate from "../share/validate/validate.ar";
import countries from "../share/countries/countries.ar";
import type_of_content from "../share/type-of-content/type-of-content.ar";

const locale: import("./studio.locale").StudioLocale = {

  "lang": "ar",
  "region": null,

  // @notranslate
  "logo_text": "nuva",

  // @notranslate
  "app_name": "Nuva Studio",
  
  "station_type_of_content": type_of_content,
  "countries": countries,
  "validate": validate,
  "stats_map": stats_map,

  "language": {
    "auto": "الكشف التلقائي",
    "en": "English (الإنجليزية)",
    "es": "Español (الإسبانية)",
    "es-AR": "Español de Argentina (الإسبانية الأرجنتينية)",
    "pt": "Português (البرتغالية)",
    "de": "Deutsche (الألمانية)",
    "fr": "Française (الفرنسية)",
    "it": "Italiano (الإيطالية)",
    "zh": "简体中文 (الصينية المبسطة)",
    "ar": "عربي",
  },

  "copy_to_clipboard": "نسخ إلى الحافظة",
  "show_password": "إظهار كلمة المرور",
  "hide_password": "إخفاء كلمة المرور",


  "drawer": {
    "account_selector": {
      "see_all_accounts": "عرض جميع الحسابات",
    },
    "dashboard": "لوحة التحكم",
    "stations": "محطات",
    "members": "أعضاء",
    "analytics": "تحليلات",
  },

  "limits": {
    "of": "من",
    "stations": "محطات",
    "listeners": "المستمعين",
    "transfer": "نقل",
    "storage": "تخزين",
  },

  "dialogs": {
    "delete": {
      "default_message": "هذا الإجراء دائم.",
      "cancel": "إلغاء",
      "delete": "حذف"
    }
  },

  "station_nav": {
    "dashboard": "لوحة التحكم",
    "profile": "الملف الشخصي",
    "playlist": "قائمة التشغيل",
    "broadcast": "بث",
    "settings": "الإعدادات",
  },

  "station_profile": {
    "titles": {
      "logo": "الشعار",
      "profile_info": "الملف الشخصي",
      "contact_info": "معلومات الاتصال",
      "social": "وسائل التواصل الاجتماعي",
      "apps": "تطبيقات",
    },
    "validation": {
      "logo_required": "الشعار مطلوب",
    },
    "upload_image": "تحميل الصورة",
    "picture_requirement_labels": {
      "format": "تنسيقات الصور المقبولة:",
      "size": "الحجم الأدنى للصورة:",
      "file_size": "الحجم الأقصى للملف:",
      "square": "يجب أن تكون الصورة مربعة",
    },

    "labels": {
      "name": "الاسم",
      "slogan": "شعار",
      "description": "الوصف",
      "country": "البلد",
      "type_of_content": "نوع المحتوى",
      "email": "البريد الإلكتروني",
      "phone": "رقم الهاتف الكامل",
      "whatsapp": "رقم واتساب الكامل",
      "website": "عنوان الموقع الإلكتروني",
      "twitter": "عنوان تويتر",
      "facebook": "عنوان الفيسبوك",
      "instagram": "عنوان إنستغرام",
      "youtube": "عنوان يوتيوب",
      "twitch": "عنوان تويتش",
      "google_play": "عنوان متجر جوجل بلاي",
      "app_store": "عنوان متجر التطبيقات"
    }
  },

  "plan_selector": {
    "price": {
      "per_month": "في الشهر",
      "$_n_per_month": "$ @n / شهر",
    },

    "unlimited": "غير محدود",

    "trial": {
      "30_day": "30 يوم",
      "free_trial": "تجربة مجانية",
      "tooltip": "لن يتم محاسبتك حتى انتهاء فترة التجربة الخاصة بك ، ويمكنك إلغاء الاشتراك في أي وقت"
    },

    "features": {
      "station": "محطة",
      "stations": "محطات",
      "listeners": "المستمعين",
      "transfer": "نقل",
      "storage": "تخزين",
      "staff": "المستخدمين",
      "auto_dj": "دي جي تلقائي",
      "stats": "إحصائيات متقدمة",
      "android_app": "تطبيق أندرويد",
    },

    "tooltips": {
      "one_station": "يمكنك إنشاء محطة واحدة فقط في هذه الخطة",
      "n_stations": "حتى @n محطات مختلفة",
      "listeners": "حتى @n مستمع متزامن",
      "transfer": "مع @tb تيرابايت من نقل البيانات الشهري ، ستتمكن من بث حوالي @hours ساعة من الصوت",
      "storage": "@gb جيجابايت من التخزين للموسيقى أو الحلقات القديمة",
      "staff": "يمكنك إضافة المستخدمين لجميع أفراد فريقك بدون حدود",
      "auto_dj": "يبث من قائمة التشغيل عندما تكون غير متصل أو لا تبث مباشرة",
      "stats": "إحصائيات تاريخية ومباشرة ، اعرف من يستمع إلى محطاتك",
      "android_app": "تطبيق أندرويد بعلامتك التجارية ومحطاتك ، متاح للجميع عبر Google Play",
    }
  },

  "pages": {

    "error": {
      "retry": "إعادة المحاولة",
      "home": "الذهاب إلى البداية",
      "default_message": "حدث خطأ",
      "offline": {
        "head": {
          "title": "غير متصل",
        },
        "title": "يبدو أنك غير متصل",
        "text": "يتطلب الوصول إلى الإنترنت لاستخدام @app_name",
      }
    },

    "login": {
      "head": {
        "title": "تسجيل الدخول",
      },
      "title": "تسجيل الدخول",
      "fields": {
        "email": "البريد الإلكتروني",
        "password": "كلمة المرور",
      },
      "links": {
        "forgot": "نسيت كلمة المرور؟",
        "new_user": "مستخدم جديد؟",
        "sign_up": "سجل",
      },
      "submit": "تسجيل الدخول"
    },

    "recover": {
      "head": {
        "title": "استعادة حسابك",
      },
      "title": "استعادة",
      "comment": "سنرسل لك بريدًا إلكترونيًا لاستعادة حسابك",
      "sent_message_html": "أرسلنا بريدًا إلكترونيًا إلى <b>@email</b> بتعليمات للمتابعة",
      "links": {
        "login": "العودة إلى تسجيل الدخول",
      },
      "submit": "إرسال",
    },

    "plans": {
      "head": {
        "title": "الخطط والأسعار",
      },
      "title_1": "مباشرة في 3 ... 2 ... 1 ...",
      "title_2": "ابدأ محطتك في أقل من 60 ثانية.",
      "title_3": "لن يتم محاسبتك حتى انتهاء فترة التجربة الخاصة بك. ويمكنك إلغاء الاشتراك في أي وقت.",
      "plan_selector": {
        "select_btn_label": "بدء التجربة",
      }
    },

    "register": {
      "head": {
        "title": "سجل",
      },
      "title": "ابدأ تجربتك المجانية",
      "plan": {
        "selected_plan": "الخطة المحددة",
        "$_n_price_per_month": "$ @n / شهر",
        "limits": {
          "station": "محطة",
          "stations": "محطات",
          "listeners": "المستمعين",
          "transfer": "نقل",
          "storage": "تخزين",
        },
        "links": {
          "plans": "العودة إلى الخطط والأسعار"
        }
      },
      "form": {
        "title": "أخبرنا عن نفسك",
        "account_name_comment": "إذا كنت تقوم بإنشاء حساب لمنظمة ، يمكنك ملء هذا الحقل بالاسم الخاص بالمنظمة",
        "fields": {
          "first_name": "اسمك الأول",
          "last_name": "اسمك الأخير",
          "account_name": "اسم حسابك",
          "phone": "هاتفك",
          "email": "بريدك الإلكتروني",
          "password": "كلمة المرور الخاصة بك",
          "confirm_password": "تأكيد كلمة المرور",
        },
        "next": "التالى",
      },
      "verification": {
        "title": "أدخل رمز التحقق",
        "message_html": "لقد أرسلنا رمز التحقق إلى <b>@email</b>",
        "back": "العودة إلى النموذج",
        "submit": "إرسال",
      },
      "links": {
        "login_comment": "هل لديك حساب بالفعل؟",
        "login_link": "تسجيل الدخول",
      }
    },

    "user_recovery": {
      "head_page_title": {
        "expired": "انتهت صلاحية الرابط",
        "used": "تم استخدام الرابط",
        "not_found": "الرابط غير موجود",
        "ok": "أعد تعيين كلمة المرور الخاصة بك",
      },
      "fields": {
        "email": "البريد الإلكتروني",
        "password": "كلمة المرور الجديدة",
        "confirm_password": "تأكيد كلمة المرور",
      },
      "error": {
        "used_message_html": "تم استخدام الرابط الذي استخدمته للوصول إلى هذه الصفحة بالفعل. <br /> قم بإنشاء رابط جديد من @user_recovery_page",
        "expired_message_html": "انتهت صلاحية الرابط الذي استخدمته للوصول إلى هذه الصفحة. <br /> قم بإنشاء رابط جديد من @user_recovery_page",
        "not_found_message_html": "الرابط الذي استخدمته للوصول إلى هذه الصفحة غير موجود. <br /> قم بإنشاء رابط جديد من @user_recovery_page",
        "user_recovery_page": "صفحة الاسترداد",
      },
      "submit": "إرسال",
      "notifier": {
        "password_updated": "تم تحديث كلمة المرور",
      }
    },

    "accounts": {
      "head": {
        "title": "الحسابات",
      },
      "title": "اختر حسابًا",
      "create_new_account": "إنشاء حساب جديد",
      "or": "أو",
      "no_items_message_html": "ليس لديك حساب مستخدم حتى الآن. <br/> لبدء البث، قم بإنشاء حساب مستخدم.",
      "no_items_create": "إنشاء حساب المستخدم الخاص بي",
    },

    "accounts/create_account": {
      "head": {
        "title": "اختر خطة",
      },
      "title": "اختر خطة لحسابك الجديد",
      "select": "تحديد",
    },

    "accounts/create_account/plan": {
      "head": {
        "title": "إنشاء حساب مستخدم",
      },
      "title": "إنشاء حساب مستخدم",
      "plan": {
        "title": "الخطة المحددة",
        "$_n_per_month": "$ @n / شهر",
        "station": "محطة",
        "stations": "محطات",
        "listeners": "المستمعين",
        "transfer": "نقل",
        "storage": "تخزين",
        "back": "العودة إلى الخطط والأسعار",
      },
      "form": {
        "title": "أخبرنا عن الحساب الجديد",
        "fields": {
          "account_name":"اسم الحساب",
          "account_name_message": "إذا كنت تقوم بإنشاء حساب لمنظمة، يمكنك ملء هذا الحقل بالاسم الخاص بالمنظمة",
        },
        "submit": "إنشاء",
      }
    },

    "account/dashboard": {
      "edit": {
        "tooltip": "تعديل",
        "dialog": {
          "field_label": "اسم الحساب",
          "title": "تعديل اسم حسابك",
          "save": "حفظ",
        }
      },

      "stats_map": {
        "all_stations": "جميع المحطات",
      },

      "station_item": {
        "on_air": "تشغيل",
        "off_air": "إيقاف",
        "playlist": "قائمة التشغيل",
        "live": "مباشر",
      }
    },

    "stations": {
      "head": {
        "title": "المحطات",
      },
      "title": "اختر محطة",
      "create_new_station": "إنشاء محطة جديدة",
      "or": "أو",
      "no_items_message_html": "لا توجد محطات في هذا الحساب حتى الآن. <br /> لبدء البث، قم بإنشاء محطة جديدة.",
      "no_items_create": "إنشاء محطة",
    },

    "stations/create_station": {
      "head": {
        "title": "إنشاء محطة"
      },
      "title": "إنشاء محطة",
      "submit": "إنشاء محطة",
      "notifier": {
        "station_created": "تم إنشاء محطة جديدة",
      }
    },

    "station/dashboard": {
      "on_air": "تشغيل",
      "off_air": "إيقاف",
      "playlist": "قائمة التشغيل",
      "live": "مباشر",
      "preview": "معاينة",
      "broadcast": "بث",
      "aria_pause": "إيقاف مؤقت",
      "aria_play": "تشغيل",
    },

    "station/profile": {
      "head": {
        "title": "الملف الشخصي",
      },
      "title": "الملف الشخصي",
      "submit": "حفظ",
      "notifier": {
        "no_changes": "لا توجد تغييرات للحفظ",
        "station_updated": "تم تحديث المحطة",
      }
    },

    "station/playlist": {
      "head": {
        "title": "قائمة التشغيل",
      },
      "title": "قائمة التشغيل",
      "explain_html": "أنشئ قائمة تشغيل للموسيقى أو الحلقات القديمة للحفاظ على نشاط محطتك على مدار 24 ساعة في اليوم و7 أيام في الأسبوع <br /> عندما لا يكون لديك اتصال أو لا تبث مباشرة ، ستتحكم <b> قائمة التشغيل </b> تلقائيًا.",
      "upload": "تحميل",
      "browse": "استعراض",
      "upload_files": "تحميل الملفات",
      "tracks_title": "المسارات",
      "track": "مسار",
      "tracks": "المسارات",
      "actions": {
        "restart_playlist": "إعادة تشغيل قائمة التشغيل",
        "shuffle_playlist": "خلط قائمة التشغيل",
        "unshuffle_playlist": "إلغاء خلط قائمة التشغيل",
        "drag_to_rearrange": "اسحب لإعادة ترتيب",
        "edit": "تعديل",
        "delete": "حذف",
      },
      "columns": {
        "title": "العنوان",
        "artist": "الفنان",
        "album": "الألبوم",
        "duration": "المدة",
      },
      "selection": {
        "one_track_selected": "تم تحديد مسار واحد",
        "n_tracks_selected": "تم تحديد @n مسارات",
        "delete_selected": "حذف المحدد",
        "select_all": "تحديد الكل",
        "unselect_all": "إلغاء تحديد الكل",
      },
      "uploading": {
        "success": "تم التحميل بنجاح",
        "waiting": "في انتظار",
        "in_progress": "جاري...",
        "retry": "إعادة المحاولة",
        "clear_done": "إخفاء العناصر المنتهية",
      },
      "dialogs": {
        "delete_track": {
          "title": "حذف المسار @name"
        },
        "delete_tracks": {
          "title": "حذف @n مسارات",
        },
        "edit_track": {
          "title": "تعديل المسار @name",
          "fields": {
            "title": "العنوان",
            "artist": "الفنان",
            "album": "الألبوم",
          },
          "cancel": "إلغاء",
          "save": "حفظ",
        },
        "shuffle_playlist": {
          "title": "خلط قائمة التشغيل",
          "message": "هل أنت متأكد أنك تريد خلط قائمة التشغيل عشوائيًا؟",
          "cancel": "إلغاء",
          "submit": "خلط",
        },
        "unshuffle_playlist": {
          "title": "إلغاء خلط قائمة التشغيل",
          "message": "هل أنت متأكد أنك تريد إلغاء خلط قائمة التشغيل؟",
          "cancel": "إلغاء",
          "submit": "إلغاء الخلط",
        },
        "restart_playlist": {
          "title": "إعادة تشغيل قائمة التشغيل",
          "message": "هل أنت متأكد أنك تريد إعادة تشغيل قائمة التشغيل؟",
          "cancel": "إلغاء",
          "submit": "إعادة تشغيل",
        }
      },
      "upload_prevent_unload_message": "الخروج من هذه الصفحة سيؤدي إلى إلغاء التحميلات المعلقة. هل تريد الخروج على أي حال؟",
      "notifier": {
        "playlist_restarted": "تم إعادة تشغيل قائمة التشغيل",
        "track_deleted": "تم حذف المسار",
        "deleting_n_tracks": "حذف @n مسارات",
        "n_tracks_deleted": "تم حذف @n مسارات",
        "playlist_unshuffled": "تم إلغاء خلط قائمة التشغيل",
        "playlist_shuffled": "تم خلط قائمة التشغيل",
      }
    },

    "station/settings": {
      "head": {
        "title": "الإعدادات",
      },
      "title": "الإعدادات",
      "actions": {
        "title": "الإجراءات",
        "delete_station": "حذف المحطة",
      },
      "validate": {
        "station_name": "اسم المحطة غير متطابق",
      },
      "notifier": {
        "station_deleted": "تم حذف المحطة",
      },
      "dialogs": {
        "delete_station": {
          "title": "حذف المحطة @name",
          "message_html": "حذف المحطة هو إجراء دائم، لن تتمكن من الوصول مرة أخرى إلى معلومات المحطة، لذا تأكد من أنك متأكد من المضي قدما. <br /><br /> إذا كنت ترغب حقا في حذف المحطة @name أدخل اسم المحطة في الحقل التالي: <b>@name</b><br />",
          "field_label": "اسم المحطة",
          "cancel": "إلغاء",
          "submit": "حذف",
        }
      }
    },

    "station/broadcast": {
      "head": {
        "title": "بث",
      },
      "title": "بث",
      "icecast_settings": "إعدادات Icecast",
      "fields": {
        "address": "العنوان",
        "port": "المنفذ",
        "mountpoint": "نقطة التثبيت",
        "username": "اسم المستخدم",
        "password": "كلمة المرور",
        "encoding": "التنسيق",
      },
      "encoding_or": "أو",
      "password_reset": "إعادة تعيين",
      "links": {
        "title": "روابط البث",
        "main": "الرئيسية",
      },
      "notifier": {
        "copied_to_clipboard": "تم النسخ إلى الحافظة",
        "mount_password_reset": "تم إعادة تعيين كلمة المرور",
      },
      "dialogs": {
        "reset_password": {
          "title": "إعادةتعيين كلمة مرور نقطة التثبيت",
          "message": "هل أنت متأكد أنك تريد إعادة تعيين كلمة المرور لنقطة التثبيت؟",
          "cancel": "إلغاء",
          "submit": "إعادة تعيين كلمة المرور",
        }
      }
    },

    "me": {
      "title": "الملف الشخصي",
      "fields": {
        "email": "بريدك الإلكتروني",
        "first_name": "اسمك الأول",
        "last_name": "اسم العائلة",
        "phone": "هاتفك",
        "new_password": "كلمة مرور جديدة",
        "confirm_password": "تأكيد كلمة المرور",
        "language": "اللغة المفضلة",
      },
      "submit": {
        "profile": "حفظ",
        "password": "حفظ",
      },
      "change_password": {
        "title": "تغيير كلمة المرور",
      },
      "more": {
        "title": "المزيد",
        "connected_devices": "الأجهزة المتصلة",
      },
      "notifier": {
        "no_changes": "لا توجد تغييرات للحفظ",
        "profile_updated": "تم تحديث الملف الشخصي",
        "password_updated": "تم تحديث كلمة المرور",
      }
    },

    "me/devices": {
      "head": {
        "title": "الأجهزة",
      },
      "title": "الأجهزة المتصلة",
      "note": "قد يظهر نفس الجهاز أكثر من مرة في هذه القائمة. سيتم قطع اتصال الأجهزة بعد 7 أيام من عدم النشاط.",
      "dialogs": {
        "disconnect": {
          "title": "قطع اتصال الجهاز",
          "message": "هذا الإجراء دائم.",
          "cancel": "إلغاء",
          "submit": "قطع الاتصال",
        },
      },

      "notifier": {
        "device_disconnected": "تم قطع اتصال الجهاز",
      },

      "device": {
        "browser": "المتصفح",
        "os": "النظام",
        "ip": "IP",
        "last_used": "آخر استخدام",
        "connected": "متصل",
        "unkown": "غير معروف",
        "tooltips": {
          "disconnect": "قطع الاتصال",
        }
      }
    }
  },

  "user_menu": {
    "profile": "الملف الشخصي",
    "accounts": "الحسابات",
    "stations": "المحطات",
    "sign_out": "تسجيل الخروج",
  }
}

export default locale;