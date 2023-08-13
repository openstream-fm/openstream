/// file: validate.ar.ts
const locale: import("./validate.locale").ValidateLocale = {
  "required": "هذا الحقل مطلوب",

  "number": "يجب أن يكون هذا الحقل رقمًا صالحًا",
  "email": "يجب أن يكون هذا الحقل عنوان بريد إلكتروني صالح",

  "min": "يجب أن يكون هذا الحقل @min أو أكثر",
  "max": "يجب أن يكون هذا الحقل @max أو أقل",

  "minlen": "يجب أن يكون هذا الحقل @minlen أحرف أو أكثر",
  "maxlen": "يجب أن يكون هذا الحقل @maxlen أحرف أو أقل",

  "new_password": {
    "minlen": "يجب أن تكون الرقم السري الجديد @minlen أحرف أو أكثر",
    "maxlen": "يجب أن تكون الرقم السري الجديد @maxlen أحرف أو أقل"
  },

  "confirmation_password": "تأكيد الرقم السري غير متطابق",

  "phone": {
    "tel": "يجب أن يكون هذا الحقل رقم هاتف دولي صالح",
    "whatsapp": "يجب أن يكون هذا الحقل رقم واتساب دولي صالح",
  },

  "url": {
    "valid": "يجب أن يكون هذا الحقل عنوان URL صالح",
    "protocol": "يجب أن يبدأ هذا الحقل بـ http:// أو https://",
  },

  "email_registered": "هذا البريد الإلكتروني مسجل بالفعل",

  "twitter_url": "يجب أن يكون هذا الحقل عنوان URL صالح لتويتر، انتقل إلى صفحتك على تويتر وانسخ العنوان الكامل من هناك",
  "facebook_url": "يجب أن يكون هذا الحقل عنوان URL صالح لفيسبوك، انتقل إلى صفحتك على فيسبوك وانسخ العنوان الكامل من هناك",
  "instagram_url": "يجب أن يكون هذا الحقل عنوان URL صالح لإنستغرام، انتقل إلى صفحتك على إنستغرام وانسخ العنوان الكامل من هناك",
  "threads_url": "يجب أن يكون هذا الحقل عنوان URL صالح لثريدز، انتقل إلى صفحتك على ثريدز وانسخ العنوان الكامل من هناك",
  "youtube_url": "يجب أن يكون هذا الحقل عنوان URL صالح ليوتيوب، انتقل إلى صفحتك على يوتيوب وانسخ العنوان الكامل من هناك",
  "twitch_url": "يجب أن يكون هذا الحقل عنوان URL صالح لتويتش، انتقل إلى صفحتك على تويتش وانسخ العنوان الكامل من هناك",
  "tiktok_url": "يجب أن يكون هذا الحقل عنوان URL صالح لتيك توك، انتقل إلى صفحتك على تيك توك وانسخ العنوان الكامل من هناك",
  "spotify_url": "يجب أن يكون هذا الحقل عنوان URL صالح لسبوتيفاي، انتقل إلى صفحتك على سبوتيفاي وانسخ العنوان الكامل من هناك",
  "google_play_url": "يجب أن يكون هذا الحقل عنوان URL صالح لجوجل بلاي، انتقل إلى صفحة التطبيق الخاص بك على جوجل بلاي وانسخ العنوان الكامل من هناك",
  "app_store_url": "يجب أن يكون هذا الحقل عنوان URL صالح لآب ستور، انتقل إلى صفحة التطبيق الخاص بك على آب ستور وانسخ العنوان الكامل من هناك",
}

export default locale;