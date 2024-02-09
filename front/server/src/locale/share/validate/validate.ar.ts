/// file: validate.ar.ts
const locale: import("./validate.locale.js").ValidateLocale = {
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

  "css_color": "يجب أن يكون هذا الحقل لون CSS صالح، على سبيل المثال: #ffffff أو rgba(0,0,0,0)",

  "twitter_url": "يجب أن يكون هذا الحقل عنوان URL لتويتر صالح، انتقل إلى صفحتك على تويتر وانسخ العنوان الكامل من هناك",
  "facebook_url": "يجب أن يكون هذا الحقل عنوان URL لفيسبوك صالح، انتقل إلى صفحتك على فيسبوك وانسخ العنوان الكامل من هناك",
  "instagram_url": "يجب أن يكون هذا الحقل عنوان URL لإنستغرام صالح، انتقل إلى صفحتك على إنستغرام وانسخ العنوان الكامل من هناك",
  "threads_url": "يجب أن يكون هذا الحقل عنوان URL لـ Threads صالح، انتقل إلى صفحتك على Threads وانسخ العنوان الكامل من هناك",
  "youtube_url": "يجب أن يكون هذا الحقل عنوان URL لـ Youtube صالح، انتقل إلى صفحتك على Youtube وانسخ العنوان الكامل من هناك",
  "twitch_url": "يجب أن يكون هذا الحقل عنوان URL لـ Twitch صالح، انتقل إلى صفحتك على Twitch وانسخ العنوان الكامل من هناك",
  "tiktok_url": "يجب أن يكون هذا الحقل عنوان URL لـ TikTok صالح، انتقل إلى صفحتك على TikTok وانسخ العنوان الكامل من هناك",
  "spotify_url": "يجب أن يكون هذا الحقل عنوان URL لـ Spotify صالح، انتقل إلى صفحتك على Spotify وانسخ العنوان الكامل من هناك",
  "radiocut_url": "يجب أن يكون هذا الحقل عنوان URL لـ RadioCut صالح، انتقل إلى صفحتك على RadioCut وانسخ العنوان الكامل من هناك",
  "google_play_url": "يجب أن يكون هذا الحقل عنوان URL لـ Google Play صالح، انتقل إلى صفحة التطبيق الخاص بك على Google Play وانسخ العنوان الكامل من هناك",
  "app_store_url": "يجب أن يكون هذا الحقل عنوان URL لـ App Store صالح، انتقل إلى صفحة التطبيق الخاص بك على App Store وانسخ العنوان الكامل من هناك",
}

export default locale;