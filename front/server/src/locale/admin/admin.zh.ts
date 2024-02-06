import stats_map from "../share/stats-map/stats-map.zh";
import validate from "../share/validate/validate.zh";
import analytics from "../share/analytics/analytics.zh";
import countries from "../share/countries/countries.zh";
import langs from "../share/langs/langs.zh";
import misc from "../misc/misc.zh";
import language from "../share/language/language.zh";

const locale: import("./admin.locale").AdminLocale = {

  "lang": "zh",
  "region": null,

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "app_name": "Openstream Admin",

  "show_password": "显示密码",
  "hide_password": "隐藏密码",

  "validate": validate,
  "stats_map": stats_map,
  "analytics": analytics,
  "countries": countries,
  "langs": langs,
  "misc": misc,
  "language": language,

  "pages": {
    "me": {
      "title": "个人资料",
      "fields": {
        "email": "你的邮箱",
        "first_name": "你的名字",
        "last_name": "你的姓氏",
        "phone": "你的电话",
        "current_password": "当前密码",
        "new_password": "新密码",
        "confirm_password": "确认密码",
        "language": "首选语言",
      },
      "submit": {
        "profile": "保存",
        "password": "保存",
      },
      "change_password": {
        "title": "更改你的密码",
      },
      "more": {
        "title": "更多",
        "connected_devices": "已连接的设备",
      },
      "notifier": {
        "no_changes": "没有要保存的更改",
        "profile_updated": "个人资料已更新",
        "password_updated": "密码已更新",
      }
    },

    "me/devices": {
      "head": {
        "title": "设备",
      },
      "title": "已连接的设备",
      "note": "同一设备可能会在此列表中多次出现。设备在7天无活动后将被断开连接。",
      "dialogs": {
        "disconnect": {
          "title": "断开设备连接",
          "message": "此操作是永久性的。",
          "cancel": "取消",
          "submit": "断开连接",
        },
      },

      "notifier": {
        "device_disconnected": "设备已断开连接",
      },

      "device": {
        "browser": "浏览器",
        "os": "系统",
        "ip": "IP",
        "last_used": "最后使用时间",
        "connected": "已连接",
        "unkown": "未知",
        "tooltips": {
          "disconnect": "断开连接",
        }
      }
    }
  }
}

export default locale;