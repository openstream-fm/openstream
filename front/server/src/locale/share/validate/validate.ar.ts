/// file: validate.ar.ts
const locale: import("./validate.locale").ValidateLocale = {
  "required": "هذا الحقل مطلوب",
  
  "number": "يجب أن يكون هذا الحقل رقمًا صحيحًا",
  "email": "يجب أن يكون هذا الحقل عنوان بريد إلكتروني صحيح",
  
  "min": "يجب أن يكون هذا الحقل @min أو أكثر",
  "max": "يجب أن يكون هذا الحقل @max أو أقل",
  
  "minlen": "يجب أن يحتوي هذا الحقل على @minlen أحرف أو أكثر",
  "maxlen": "يجب أن يحتوي هذا الحقل على @maxlen أحرف أو أقل",

  "new_password": {
    "minlen": "يجب أن تحتوي كلمة المرور الجديدة على @minlen أحرف أو أكثر",
    "maxlen": "يجب أن تحتوي كلمة المرور الجديدة على @maxlen أحرف أو أقل"
  },

  "confirmation_password": "تأكيد كلمة المرور غير متطابق",
  
  "phone": {
    "tel": "يجب أن يكون هذا الحقل رقم هاتف دولي صحيح",
    "whatsapp": "يجب أن يكون هذا الحقل رقم واتساب دولي صحيح",
  },

  "url": {
    "valid": "يجب أن يكون هذا الحقل عنوان URL صحيح",
    "protocol": "يجب أن يبدأ هذا الحقل بـ http:// أو https://",
  },

  "email_registered": "هذا البريد الإلكتروني مسجل بالفعل",

  "twitter_url": "يجب أن يكون هذا الحقل عنوان URL تويتر صحيح، انتقل إلى صفحتك على تويتر وانسخ العنوان الكامل من هناك",
  "facebook_url": "يجب أن يكون هذا الحقل عنوان URL فيسبوك صحيح، انتقل إلى صفحتك على فيسبوك وانسخ العنوان الكامل من هناك",
  "instagram_url": "يجب أن يكون هذا الحقل عنوان URL إنستجرام صحيح، انتقل إلى صفحتك على إنستجرام وانسخ العنوان الكامل من هناك",
  "youtube_url": "يجب أن يكون هذا الحقل عنوان URL يوتيوب صحيح، انتقل إلى صفحتك على يوتيوب وانسخ العنوان الكامل من هناك",
  "twitch_url": "يجب أن يكون هذا الحقل عنوان URL تويتش صحيح، انتقل إلى صفحتك على تويتش وانسخ العنوان الكامل من هناك",
  "tiktok_url": "يجب أن يكون هذا الحقل عنوان URL تيك توك صحيح، انتقل إلى صفحتك على تيك توك وانسخ العنوان الكامل من هناك",
  "google_play_url": "يجب أن يكون هذا الحقل عنوان URL جوجل بلاي صحيح، انتقل إلى صفحة تطبيقك على جوجل بلاي وانسخ العنوان الكامل من هناك",
  "app_store_url": "يجب أن يكون هذا الحقل عنوان URL متجر التطبيقات صحيح، انتقل إلى صفحة تطبيقك على متجر التطبيقات وانسخ العنوان الكامل من هناك",
}

export default locale;