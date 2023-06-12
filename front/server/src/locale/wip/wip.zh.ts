/// file: wip.zh.ts
const locale: typeof import("./wip.en").default = {
  "pages": {
    "account/members": {
      "head": {
        "title": "成员"
      },
      "title": "成员",

      "no_owner_message_p1": "此部分仅供帐户管理员使用",
      "no_owner_message_p2": "如果您需要邀请其他人参加此帐户，请与帐户管理员联系。",

      "Pending_invitations": "待处理的邀请",
      "no_pending_invitations_message": "没有待处理的邀请",
      "invite_btn_text": "邀请人员",

      "validate": {
        "user_account_exists": "电子邮件为 @email 的用户已经是此帐户的成员",
      },

      "notifier": {
        "invitation_sent": "邀请已发送",
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
  }
}

export default locale;