/// file: misc.ar.ts
/// file: wip.ar.ts
const locale: typeof import("./misc.en").default = {
  Relay: "الريلاي",
  Settings_updated: "تم تحديث الإعدادات",
  Master_relay: "ريلاي الرئيسي",
  Enable_master_relay: "تفعيل الريلاي الرئيسي",
  Master_Relay_URL: "URL للريلاي الرئيسي",
  Save: "حفظ",
  delete_station_not_owner_message_html: "فقط مدراء الحساب يمكنهم حذف المحطات.<br/><br/>اتصل بمدراء الحساب إذا كنت ترغب في حذف هذه المحطة.",

  Cancel: "إلغاء",
  OK: "موافق",
  Transfer_station: "نقل المحطة",
  Station_name: "اسم المحطة",
  station_transfer_title: "نقل المحطة @station إلى أحد حساباتك الأخرى",
  station_transfer_message_html: "لنقل المحطة @station إلى أحد حساباتك الأخرى، اكتب اسم المحطة: <b>@station</b> واختر الحساب الهدف.",
  station_transfer_not_owner_message_html: "فقط مدراء الحساب يمكنهم نقل المحطات بين الحسابات. <br/> <br/>اتصل بمدراء الحساب لنقل المحطة إلى حساب آخر.",
  station_transfer_no_targets_message: "يجب أن يكون لديك حق الوصول إلى حساب آخر لتتمكن من نقل هذه المحطة إلى حساب آخر.",
  Station_name_do_not_match: "اسم المحطة لا يتطابق",
  Target_account_is_required: "الحساب الهدف مطلوب",
  Station_transferred: "تم نقل المحطة",
  Select_a_target_account: "اختر حساب هدف",

  Welcome: "مرحبا",

  account_welcome_title_html: "مرحبا <b>@name</b>",
  account_welcome_message_1_html: "مرحبا بك في <b>@brand</b>",
  account_welcome_message_2_html: "أنت الآن صاحب حساب جديد",
  account_welcome_message_3_html: "لبدء البث الآن، أضف أول محطة لك في حسابك",

  Create_my_first_station: "إنشاء أول محطة لي",
}

export default locale;