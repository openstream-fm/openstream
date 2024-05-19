import stats_map from "../share/stats-map/stats-map.ar.js";
import validate from "../share/validate/validate.ar.js";
import analytics from "../share/analytics/analytics.ar.js";
import countries from "../share/countries/countries.ar.js";
import langs from "../share/langs/langs.ar.js";
import misc from "../misc/misc.ar.js";
import language from "../share/language/language.ar.js";

const locale: import("./admin.locale.js").AdminLocale = {

  "lang": "ar",
  "region": null,

  // @notranslate
  "logo_text": "openstream", 

  // @notranslate
  "app_name": "Openstream Admin",

  "show_password": "إظهار كلمة المرور",
  "hide_password": "إخفاء كلمة المرور",

  "validate": validate,
  "stats_map": stats_map,
  "analytics": analytics,
  "countries": countries,
  "langs": langs,
  "misc": misc,
  "language": language,

  "pages": {
    "me": {
      "title": "الملف الشخصي",
      "fields": {
        "email": "بريدك الإلكتروني",
        "first_name": "اسمك الأول",
        "last_name": "اسم العائلة",
        "phone": "هاتفك",
        "current_password": "كلمة المرور الحالية",
        "new_password": "كلمة المرور الجديدة",
        "confirm_password": "تأكيد كلمة المرور",
        "language": "اللغة المفضلة",
      },
      "submit": {
        "profile": "حفظ",
        "password": "حفظ",
      },
      "change_password": {
        "title": "غير كلمة المرور",
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
      "note": "قد يظهر نفس الجهاز أكثر من مرة في هذه القائمة. سيتم فصل الأجهزة بعد 7 أيام من عدم النشاط.",
      "dialogs": {
        "disconnect": {
          "title": "فصل الجهاز",
          "message": "هذا الإجراء دائم.",
          "cancel": "إلغاء",
          "submit": "فصل",
        },
      },

      "notifier": {
        "device_disconnected": "تم فصل الجهاز",
      },

      "device": {
        "browser": "المتصفح",
        "os": "النظام",
        "ip": "الآي بي",
        "last_used": "آخر استخدام",
        "connected": "متصل",
        "unkown": "مجهول",
        "tooltips": {
          "disconnect": "فصل",
        }
      }
    }
  }
}

export default locale;