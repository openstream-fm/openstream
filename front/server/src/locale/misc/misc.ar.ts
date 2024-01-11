const locale: typeof import("./misc.en").default = {
  Relay: "الريلاي",
  Settings_updated: "تم تحديث الإعدادات",
  Master_relay: "ريلاي رئيسي",
  Enable_master_relay: "تفعيل الريلاي الرئيسي",
  Master_Relay_URL: "رابط الريلاي الرئيسي",
  Save: "حفظ",
  delete_station_not_owner_message_html: "فقط مدراء الحساب يمكنهم حذف المحطات.<br/><br/>اتصل بمدراء الحساب إذا أردت حذف هذه المحطة.",

  Cancel: "إلغاء",
  OK: "موافق",
  Delete: "حذف",
  Create: "إنشاء",
  Copy: "نسخ",
  Done: "تم",

  Copied_to_clipboard: "تم النسخ إلى الحافظة",

  Id: "الهوية",
  Title: "العنوان",
  Created: "تم الإنشاء",
  Last_used: "آخر استخدام",

  Transfer_station: "نقل المحطة",
  Station_name: "اسم المحطة",
  station_transfer_title: "نقل المحطة @station إلى حساب آخر خاص بك",
  station_transfer_message_html: "لنقل المحطة @station إلى حساب آخر خاص بك، اكتب اسم المحطة: <b>@station</b> واختر الحساب الوجهة.",
  station_transfer_not_owner_message_html: "فقط مدراء الحساب يمكنهم نقل المحطات بين الحسابات. <br/><br/>اتصل بمدراء الحساب لنقل المحطة إلى حساب آخر.",
  station_transfer_no_targets_message: "يجب أن يكون لديك حساب آخر لتتمكن من نقل هذه المحطة إلى حساب آخر.",
  Station_name_do_not_match: "اسم المحطة غير متطابق",
  Target_account_is_required: "الحساب الوجهة مطلوب",
  Station_transferred: "تم نقل المحطة",
  Select_a_target_account: "اختر حساب وجهة",

  Type_password_proceed: "أدخل كلمة المرور للمتابعة بهذا الإجراء.",

  Welcome: "أهلاً بك",

  account_welcome_title_html: "مرحباً <b>@name</b>",
  account_welcome_message_1_html: "أهلاً بك في <b>@brand</b>",
  account_welcome_message_2_html: "أنت الآن مالك لحسابك الجديد",
  account_welcome_message_3_html: "لبدء البث الآن، أضف محطتك الأولى إلى حسابك",

  Create_my_first_station: "إنشاء محطتي الأولى",

  Your_email: "بريدك الإلكتروني",
  Your_password: "كلمة المرور الخاصة بك",

  "0_listeners": "0 مستمعين",
  "1_listener": "مستمع واحد",
  "n_listeners": "@n مستمعين",

  Enable_master_relay_redirect_mode: "تفعيل وضع إعادة التوجيه في الريلاي الرئيسي",
  External_relay_error: "خطأ في الريلاي الخارجي",

  player: {
    Relay: "الريلاي",
    Live_Streaming: "البث المباشر",
    Playlist: "قائمة التشغيل",
  },

  This_action_is_permanent: "هذا الإجراء دائم.",

  api_keys: {
    API_Keys: "مفاتيح API",
    API_key_deleted: "تم حذف مفتاح API",
    API_keys_page_message: "أنشئ مفاتيح API للوصول إلى حساباتك برمجياً أو لمنح الوصول لتطبيقات وخدمات الطرف الثالث.",
    Create_a_new_API_key: "إنشاء مفتاح API جديد",
    Remove_API_key: "حذف مفتاح API",
    API_key_title: "عنوان مفتاح API",
    API_key_title_explain: "سيتم استخدام العنوان من قبلك لتحديد هذا المفتاح",
    Copy_contents_message: "انسخ محتويات مفتاح API. لن يتم عرض هذا الرمز مرة أخرى.",
    API_key_contents: "محتويات مفتاح API",
  }
}

export default locale;