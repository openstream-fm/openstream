/// file: studio.zh.ts
import stats_map from "../share/stats-map/stats-map.zh.js";
import validate from "../share/validate/validate.zh.js";
import countries from "../share/countries/countries.zh.js";
import langs from "../share/langs/langs.zh.js";
import type_of_content from "../share/type-of-content/type-of-content.zh.js";
import analytics from "../share/analytics/analytics.zh.js";
import payments from "../share/payments/payments.zh.js";
import station_profile from "../share/station-profile/station-profile.zh.js";
import misc from "../misc/misc.zh.js";
import language from "../share/language/language.zh.js";

const locale: import("./studio.locale.js").StudioLocale = {

  "lang": "zh",
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

  "copy_to_clipboard": "复制到剪贴板",
  "show_password": "显示密码",
  "hide_password": "隐藏密码",

  "prevent_unload_message":  "如果你离开此页面，你所做的更改将会丢失。你确定要离开此页面吗？",

  "drawer": {
    "account_selector": {
      "see_all_accounts": "查看所有账户",
    },
    "dashboard": "仪表板",
    "stations": "电台",
    "members": "成员",
    "analytics": "分析",
  },

  "limits": {
    "of": "的",
    "stations": "电台",
    "listeners": "听众",
    "transfer": "传输",
    "storage": "存储",
  },

  "dialogs": {
    "delete": {
      "default_message": "此操作是永久性的。",
      "cancel": "取消",
      "delete": "删除"
    }
  },

  "station_nav": {
    "dashboard": "仪表板",
    "profile": "个人资料",
    "playlist": "播放列表",
    "broadcast": "广播",
    "settings": "设置",
  },

  "plan_selector": {
    "price": {
      "per_month": "每月",
      "n_per_month": "@n / 月",
    },

    "unlimited": "无限",

    "trial": {
      "30_day": "30 天",
      "free_trial": "免费试用",
      "tooltip": "在你的试用期结束之前，我们不会向你收费，你可以随时取消"
    },

    "features": {
      "station": "电台",
      "stations": "电台",
      "listeners": "听众",
      "transfer": "传输",
      "storage": "存储",
      "staff": "用户",
      "auto_dj": "自动 DJ",
      "stats": "高级统计",
      "android_app": "安卓应用",
    },

    "tooltips": {
      "one_station": "在此计划中，你只能创建一个电台",
      "n_stations": "最多 @n 个不同的电台",
      "listeners": "最多 @n 个同时在线的听众",
      "transfer": "每月 @tb TB 的传输量，你可以播放大约 @hours 小时的音频",
      "storage": "@gb GB 的存储空间用于音乐或旧的剧集",
      "staff": "你可以为你的整个团队添加无限制的用户",
      "auto_dj": "当你离线或不在直播时，从播放列表中播放",
      "stats": "历史和实时统计，了解谁在听你的电台",
      "android_app": "带有你的品牌和电台的安卓应用，通过 Google Play 在全球范围内可用",
    }
  },

  "pages": {

    "error": {
      "retry": "重试",
      "home": "回到首页",
      "default_message": "发生错误",
      "offline": {
        "head": {
          "title": "无网络连接",
        },
        "title": "看起来你已经断开了网络连接",
        "text": "你需要网络连接才能使用 @app_name",
      }
    },

    "login": {
      "head": {
        "title": "登录",
      },
      "title": "登录",
      "fields": {
        "email": "邮箱",
        "password": "密码",
      },
      "links": {
        "forgot": "忘记密码？",
        "new_user": "新用户？",
        "sign_up": "注册",
      },
      "submit": "登录"
    },

    "recover": {
      "head": {
        "title": "恢复你的账户",
      },
      "title": "恢复",
      "comment": "我们将向你发送一封电子邮件，以便你恢复你的账户",
      "sent_message_html": "我们已经向 <b>@email</b> 发送了一封包含进一步指示的电子邮件",
      "links": {
        "login": "返回登录",
      },
      "submit": "发送",
    },

    "plans": {
      "head": {
        "title": "计划和价格",
      },
      "title_1": "现场直播在 3... 2... 1...",
      "title_2": "在不到60秒的时间内启动你的电台。",
      "title_3": "在你的试用期结束之前，我们不会向你收费。你可以随时取消。",
      "plan_selector": {
        "select_btn_label": "开始试用",
      }
    },

    "register": {
      "head": {
        "title": "注册",
      },
      "title": "开始你的免费试用",
      "plan": {
        "selected_plan": "已选计划",
        "n_per_month": "@n / 月",
        "limits": {
          "station": "电台",
          "stations": "电台",
          "listeners": "听众",
          "transfer": "传输",
          "storage": "存储",
        },
        "links": {
          "plans": "返回计划和价格"
        }
      },
      "form": {
        "title": "告诉我们一些关于你的信息",
        "account_name_comment": "如果你正在为一个组织创建一个账户，你可以用该组织的名称填写此字段",
        "fields": {
          "first_name": "你的名字",
          "last_name": "你的姓氏",
          "account_name": "你的账户名",
          "phone": "你的电话",
          "email": "你的邮箱",
          "password": "你的密码",
          "confirm_password": "确认你的密码",
        },
        "next": "下一步",
      },
      "pay": {
        "title": "支付详情",
        "message": "在你的30天免费试用期结束之前，我们不会向你收费，你可以随时取消。"
      },

      "back": "返回上一步",

      "verification": {
        "title": "输入验证码",
        "message_html": "我们已经向 <b>@email</b> 发送了一个验证码",
        "submit": "发送",
      },
      "links": {
        "login_comment": "已经有账户了？",
        "login_link": "登录",
      }
    },

    "user_recovery": {
      "head_page_title": {
        "expired": "链接已过期",
        "used": "链接已被使用",
        "not_found": "找不到链接",
        "ok": "重置你的密码",
      },
      "fields": {
        "email": "邮箱",
        "password": "新密码",
        "confirm_password": "确认密码",
      },
      "error": {
        "used_message_html": "你用来访问此页面的链接已经被使用过了。<br /> 从 @user_recovery_page 创建一个新的链接",
        "expired_message_html": "你用来访问此页面的链接已经过期。<br /> 从 @user_recovery_page 创建一个新的链接",
        "not_found_message_html": "你用来访问此页面的链接不存在。<br /> 从 @user_recovery_page 创建一个新的链接",
        "user_recovery_page": "恢复页面",
      },
      "submit": "发送",
      "notifier": {
        "password_updated": "密码已更新",
      }
    },

    "accounts": {
      "head": {
        "title": "账户",
      },
      "title": "选择一个账户",
      "create_new_account": "创建新账户",
      "or": "或",
      "no_items_message_html": "你还没有电台账户。<br/>要开始广播，创建你的电台账户。",
      "no_items_create": "创建我的电台账户",
    },

    "accounts/create_account": {
      "head": {
        "title": "选择一个计划",
      },
      "title": "为你的新账户选择一个计划",
      "select": "选择",
    },

    "accounts/create_account/plan": {
      "head": {
        "title": "创建发射帐户",
      },
      "title": "创建发射帐户",
      "plan": {
        "title": "已选套餐",
        "n_per_month": "每月 @n",
        "station": "电台",
        "stations": "电台",
        "listeners": "听众",
        "transfer": "传输",
        "storage": "存储",
        "back": "返回套餐和价格",
      },
      "form": {
        "title": "告诉我们关于新帐户的信息",
        "fields": {
          "account_name":"您的帐户名称",
          "account_name_message": "如果您为组织创建帐户，可以用组织名称填写此字段",
        },
        "submit": "创建",
        "next": "下一步",
        "pay": {
          "title": "付款详情",
        },
        "back": "返回上一步",
      }
    },

    "account/dashboard": {
      "edit": {
        "tooltip": "编辑",
        "dialog": {
          "field_label": "帐户名称",
          "title": "编辑您的帐户名称",
          "save": "保存",
        }
      },

      "stats_map": {
        "all_stations": "所有电台",
      },

      "station_item": {
        "on_air": "直播中",
        "off_air": "未直播",
        "playlist": "播放列表",
        "live": "现场直播",
      }
    },

    "account/analytics": {
      "head": {
        "title": "分析",
      },
      "title": "分析",
    },

    "stations": {
      "head": {
        "title": "电台",
      },
      "title": "选择一个电台",
      "create_new_station": "创建新电台",
      "or": "或者",
      "no_items_message_html": "此帐户尚无电台。<br />要开始广播，请创建新电台。",
      "no_items_create": "创建电台",
    },

    "stations/create_station": {
      "head": {
        "title": "创建电台"
      },
      "title": "创建电台",
      "submit": "创建电台",
      "notifier": {
        "station_created": "新电台已创建",
      }
    },

    "station/dashboard": {
      "on_air": "直播中",
      "off_air": "未直播",
      "playlist": "播放列表",
      "live": "现场直播",
      "preview": "预览",
      "broadcast": "广播",
      "aria_pause": "暂停",
      "aria_play": "播放",
    },

    "station/profile": {
      "head": {
        "title": "个人资料",
      },
      "title": "个人资料",
      "submit": "保存",
      "notifier": {
        "no_changes": "无需保存的更改",
        "station_updated": "电台已更新",

      }
    },

    "station/playlist": {
      "head": {
        "title": "播放列表",
      },
      "title": "播放列表",
      "explain_html": "创建音乐列表或旧剧集，让您的电台全天候保持活跃<br />当您没有连接或不在现场直播时，<b>播放列表</b>将自动接管。",
      "upload": "上传",
      "browse": "浏览",
      "upload_files": "上传文件",
      "tracks_title": "音轨",
      "track": "音轨",
      "tracks": "音轨",
      "actions": {
        "restart_playlist": "重新启动播放列表",
        "shuffle_playlist": "随机播放列表",
        "unshuffle_playlist": "取消随机播放列表",
        "drag_to_rearrange": "拖动以重新排序",
        "edit": "编辑",
        "delete": "删除",
      },
      "columns": {
        "title": "标题",
        "artist": "艺术家",
        "album": "专辑",
        "duration": "时长",
      },
      "selection": {
        "one_track_selected": "1 音轨已选",
        "n_tracks_selected": "@n 音轨已选",
        "delete_selected": "删除选定的",
        "select_all": "全选",
        "unselect_all": "取消全选",
      },
      "uploading": {
        "success": "上传成功",
        "waiting": "等待",
        "in_progress": "进行中...",
        "retry": "重试",
        "clear_done": "隐藏已完成项目",
      },
      "dialogs": {
        "delete_track": {
          "title": "删除音轨 @name"
        },
        "delete_tracks": {
          "title": "删除 @n 音轨",
        },
        "edit_track": {
          "title": "编辑音轨 @name",
          "fields": {
            "title": "标题",
            "artist": "艺术家",
            "album": "专辑",
          },
          "cancel": "取消",
          "save": "保存",
        },
        "shuffle_playlist": {
          "title": "随机播放列表",
          "message": "您确定要随机播放列表吗？",
          "cancel": "取消",
          "submit": "随机",
        },
        "unshuffle_playlist": {
          "title": "取消随机播放列表",
          "message": "您确定要取消随机播放列表吗？",
          "cancel": "取消",
          "submit": "取消随机",
        },
        "restart_playlist": {
          "title": "重新启动播放列表",
          "message": "您确定要重新启动播放列表吗？",
          "cancel": "取消",
          "submit": "重新启动",
        }
      },
      "upload_prevent_unload_message": "离开此页面将取消待上传的文件。您确定要离开吗？",
      "notifier": {
        "playlist_restarted": "播放列表已重新启动",
        "track_deleted": "音轨已删除",
        "deleting_n_tracks": "删除 @n 音轨",
        "n_tracks_deleted": "@n 音轨已删除",
        "playlist_unshuffled": "播放列表已取消随机",
        "playlist_shuffled": "播放列表已随机",
      }
    },

    "station/broadcast": {
      "head": {
        "title": "广播",
      },
      "title": "广播",
      "icecast_settings": "Icecast 设置",
      "fields": {
        "address": "地址",
        "port": "端口",
        "mountpoint": "挂载点",
        "username": "用户名",
        "password": "密码",
        "encoding": "格式",
      },
      "encoding_or": "或",
      "password_reset": "重置",
      "links": {
        "title": "广播 URLs",
        "main": "主要",
      },
      "notifier": {
        "copied_to_clipboard": "已复制到剪贴板",
        "mount_password_reset": "挂载点密码已重置",
      },
      "dialogs": {
        "reset_password": {
          "title": "重置挂载点密码",
          "message": "您确定要重置挂载点密码吗？",
          "cancel": "取消",
          "submit": "重置密码",
        }
      }
    },

    "station/settings": {
      "head": {
        "title": "设置",
      },
      "title": "设置",
      "actions": {
        "title": "操作",
        "delete_station": "删除电台",
      },
      "validate": {
        "station_name": "电台名称不匹配",
      },
      "notifier": {
        "station_deleted": "电台已删除",
      },
      "dialogs": {
        "delete_station": {
          "title": "删除电台 @name",
          "message_html": "删除电台是永久性操作，您将无法再访问电台信息，请确保您要继续操作。<br /><br />如果您确实要删除电台 @name，请在下面的字段中输入电台名称：<b>@name</b><br />",
          "field_label": "电台名称",
          "cancel": "取消",
          "submit": "删除",
        }
      }
    },

    "me": {
      "title": "个人资料",
      "fields": {
        "email": "您的电子邮件",
        "first_name": "您的名字",
        "last_name": "您的姓氏",
        "phone": "您的电话",
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
        "title": "更改您的密码",
      },
      "more": {
        "title": "更多",
        "connected_devices": "已连接设备",
      },
      "notifier": {
        "no_changes": "无需保存的更改",
        "profile_updated": "个人资料已更新",
        "password_updated": "密码已更新",
      }
    },

    "me/devices": {
      "head": {
        "title": "设备",
      },
      "title": "已连接设备",
      "note": "同一设备可能会在此列表中出现多次。设备在 7 天无活动后将被断开连接。",
      "dialogs": {
        "disconnect": {
          "title": "断开设备",
          "message": "此操作是永久性的。",
          "cancel": "取消",
          "submit": "断开",
        },
      },

      "notifier": {
        "device_disconnected": "设备已断开",
      },

      "device": {
        "browser": "浏览器",
        "os": "系统",
        "ip": "IP",
        "last_used": "上次使用",
        "connected": "已连接",
        "unkown": "未知",
        "tooltips": {
          "disconnect": "断开",
        }
      }
    },

    "account/members": {
      "head": {
        "title": "成员"
      },
      "title": "成员",

      "no_owner_message_p1": "此部分仅供帐户管理员使用。",
      "no_owner_message_p2": "如果您需要邀请其他人参加此帐户，请与帐户管理员联系。",

      "Pending_invitations": "待处理的邀请",
      "no_pending_invitations_message": "没有待处理的邀请",
      "invite_btn_text": "邀请人员",

      "validate": {
        "user_account_exists": "电子邮件为 @email 的用户已经是此帐户的成员",
      },

      "notifier": {
        "invitation_sent": "邀请已发送",
        "member_access_revoked": "成员访问权限已撤销",
        "member_role_changed": "成员访问角色已更新",
      },

      "actions": {
        "set_role_to": "将角色设置为 @role",
        "revoke_access": "撤销访问权限",
        "delete": "删除",
      },

      "dialogs": {
        "invite": {
          "title": "邀请人员以 @role 角色加入此帐户",
          "submit": "邀请",
          "Email": "电子邮件",
        }
      }
    },

    "email_invitation": {
      "head_page_title": {
        "not_found": "未找到邀请",
        "expired": "邀请已过期",
        "accepted": "邀请已被接受",
        "rejected": "邀请已被拒绝",
        "ok": "待处理的邀请",
      },

      "error_message": {
        "not_found": "您用于访问此页面的链接已不存在或已被删除",
        "expired": "邀请已过期，请联系帐户管理员以获取新的邀请",
        "accepted": "邀请已被接受",
        "rejected": "邀请已被拒绝，如果是错误，请联系帐户管理员以获取新的邀请",
      },

      "description": {
        "with_sender_name_html": "<b>@sender</b> 邀请您加入 Openstream 的 <b>@account</b>。",
        "without_sender_name_html": "您被邀请加入 Openstream 的 <b>@account</b>",
      },

      "login_as_btn_html": "以 <b>@email</b> 登录以接受邀请",

      "form": {
        "fields": {
          "first_name": "您的名字",
          "last_name": "您的姓氏",
          "email": "您的电子邮件",
          "password": "密码",
          "confirm_password": "确认密码",
        },
        "pre_message_html": "要<b>接受</b>邀请，请填写表格。",
        "title": "注册",
        "submit": "提交",
      },

      "notifier": {
        "accept_error": "接受邀请时出错：@error"
      }
    },

    "me/invitations": {
      "head": {
        "title": "待处理的邀请",
      },
      "title": "待处理的邀请",

      "no_items_message": "您没有待处理的邀请",

      "notifier": {
        "accept_error": "接受邀请时出错：@error",
        "accepted": "邀请已接受",
        "rejected": "邀请已拒绝",
      },

      "actions": {
        "reject": "拒绝",
        "accept": "接受",
      },

      "item_message_with_sender_html": "<b>@sender</b> 邀请您加入 <b>@account</b>",
      "item_message_without_sender_html": "您被邀请加入 <b>@account</b>",

      "dialogs": {
        "reject": {
          "title": "拒绝邀请",
          "message": "您确定要拒绝邀请吗？",
          "cancel": "取消",
          "reject": "拒绝邀请",
        }
      }
    }
  },

  "user_menu": {
    "profile": "个人资料",
    "invitations": "邀请",
    "accounts": "账户",
    "stations": "电台",
    "sign_out": "登出",
  }
}

export default locale;