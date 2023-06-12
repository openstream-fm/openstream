/// file: wip.ar.ts
const locale: typeof import("./wip.en").default = {
  "pages": {
    "account/members": {
      "head": {
        "title": "الأعضاء"
      },
      "title": "الأعضاء",

      "no_owner_message_p1": "هذا القسم متاح فقط لمسؤولي الحساب",
      "no_owner_message_p2": "تواصل مع مسؤولي الحساب إذا كنت بحاجة إلى دعوة أشخاص للمشاركة في هذا الحساب.",

      "Pending_invitations": "دعوات معلقة",
      "no_pending_invitations_message": "لا توجد دعوات معلقة",
      "invite_btn_text": "دعوة أشخاص",

      "validate": {
        "user_account_exists": "المستخدم بالبريد الإلكتروني @email ينتمي بالفعل إلى هذا الحساب",
      },

      "notifier": {
        "invitation_sent": "تم إرسال الدعوة",
      },

      "dialogs": {
        "invite": {
          "title": "ادعو الأشخاص للمشاركة في هذا الحساب بدور @role",
          "submit": "دعوة",
          "Email": "البريد الإلكتروني",
        }
      }
    },

    "email_invitation": {
      "head_page_title": {
        "not_found": "الدعوة غير موجودة",
        "expired": "انتهت صلاحية الدعوة",
        "accepted": "تم قبول الدعوة",
        "rejected": "تم رفض الدعوة",
        "ok": "دعوة معلقة",
      },

      "error_message": {
        "not_found": "الرابط الذي استخدمته للوصول إلى هذه الصفحة لم يعد موجودًا أو تم حذفه",
        "expired": "انتهت صلاحية الدعوة، اتصل بمسؤولي الحساب ليقوموا بإرسال دعوة جديدة لك",
        "accepted": "تم قبول الدعوة",
        "rejected": "تم رفض الدعوة، إذا كان هذا خطأ، اتصل بمسؤولي الحساب ليقوموا بإرسال دعوة جديدة لك",
      },

      "description": {
        "with_sender_name_html": "<b>@sender</b> يدعوك للانضمام إلى <b>@account</b> في Openstream.",
        "without_sender_name_html": "تمت دعوتك للانضمام إلى <b>@account</b> في Openstream",
      },

      "login_as_btn_html": "قم بتسجيل الدخول كـ <b>@email</b> لقبول الدعوة",

      "form": {
        "fields": {
          "first_name": "اسمك الأول",
          "last_name": "اسمك الأخير",
          "email": "بريدك الإلكتروني",
          "password": "كلمة المرور",
          "confirm_password": "تأكيد كلمة المرور",
        },
        "pre_message_html": "ل<b>قبول</b> الدعوة، أكمل النموذج.",
        "title": "تسجيل",
        "submit": "إرسال",
      },

      "notifier": {
        "accept_error": "حدث خطأ أثناء قبول الدعوة: @error"
      }
    },

    "me/invitations": {
      "head": {
        "title": "دعوات معلقة",
      },
      "title": "دعوات معلقة",

      "no_items_message": "ليس لديك دعوات معلقة",

      "notifier": {
        "accept_error": "حدث خطأ أثناء قبول الدعوة: @error",
        "accepted": "تم قبول الدعوة",
        "rejected": "تم رفض الدعوة",
      },

      "actions": {
        "reject": "رفض",
        "accept": "قبول",
      },

      "item_message_with_sender_html": "<b>@sender</b> يدعوك للانضمام إلى <b>@account</b>",
      "item_message_without_sender_html": "تمت دعوتك للانضمام إلى <b>@account</b>",

      "dialogs": {
        "reject": {
          "title": "رفض الدعوة",
          "message": "هل أنت متأكد من رغبتك في رفض الدعوة؟",
          "cancel": "إلغاء",
          "reject": "رفض الدعوة",
        }
      }
    }
  }
}

export default locale;