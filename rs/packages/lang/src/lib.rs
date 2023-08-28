use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "lowercase")]
#[ts(export, export_to = "../../../defs/")]
pub enum LangCode {
  AA, // Afar - aar
  AB, // Abkhazian - abk - also known as Abkhaz
  AE, // Avestan - ave - ancient
  AF, // Afrikaans - afr
  AK, // Akan - aka - macrolanguage, Twi is tw/twi, Fanti is fat
  AM, // Amharic - amh
  AN, // Aragonese - arg
  AR, // Arabic - ara - macrolanguage, Standard Arabic is arb
  AS, // Assamese - asm
  AV, // Avaric - ava - also known as Avar
  AY, // Aymara - aym - macrolanguage
  AZ, // Azerbaijani - aze - macrolanguage, also known as Azeri
  BA, // Bashkir - bak
  BE, // Belarusian - bel
  BG, // Bulgarian - bul
  BI, // Bislama - bis - Language formed from English and Vanuatuan languages, with some French influence.
  BM, // Bambara - bam
  BN, // Bengali - ben - also known as Bangla
  BO, // Tibetan - bod - also known as Standard Tibetan
  BR, // Breton - bre
  BS, // Bosnian - bos
  CA, // Catalan, Valencian - cat
  CE, // Chechen - che
  CH, // Chamorro - cha
  CO, // Corsican - cos
  CR, // Cree - cre - macrolanguage
  CS, // Czech - ces
  CU, // Church Slavonic, Old Slavonic, Old Church Slavonic - chu - ancient, in use by the Eastern Orthodox Church
  CV, // Chuvash - chv
  CY, // Welsh - cym
  DA, // Danish - dan
  DE, // German - deu
  DV, // Divehi, Dhivehi, Maldivian - div
  DZ, // Dzongkha - dzo
  EE, // Ewe - ewe
  EL, // Greek, Modern (1453–) - ell - for Ancient Greek, use the ISO 639-3 code grc
  EN, // English - eng
  EO, // Esperanto - epo - constructed, initially by L.L. Zamenhof in 1887
  ES, // Spanish, Castilian - spa
  ET, // Estonian - est - macrolanguage
  EU, // Basque - eus
  FA, // Persian - fas - macrolanguage, also known as Farsi
  FF, // Fulah - ful - macrolanguage, also known as Fula
  FI, // Finnish - fin
  FJ, // Fijian - fij
  FO, // Faroese - fao
  FR, // French - fra
  FY, // Western Frisian - fry - also known as Frisian
  GA, // Irish - gle
  GD, // Gaelic, Scottish Gaelic - gla
  GL, // Galician - glg
  GN, // Guarani - grn - macrolanguage
  GU, // Gujarati - guj
  GV, // Manx - glv
  HA, // Hausa - hau
  HE, // Hebrew - heb - Modern Hebrew. Code changed in 1989 from original ISO 639:1988, iw.[1]
  HI, // Hindi - hin
  HO, // Hiri Motu - hmo
  HR, // Croatian - hrv
  HT, // Haitian, Haitian Creole - hat
  HU, // Hungarian - hun
  HY, // Armenian - hye - ISO 639-3 code hye is for Eastern Armenian, hyw is for Western Armenian, and xcl is for Classical Armenian
  HZ, // Herero - her
  IA, // Interlingua (International Auxiliary Language Association) - ina - constructed by the International Auxiliary Language Association
  ID, // Indonesian - ind - covered by macrolanguage ms/msa. Changed in 1989 from original ISO 639:1988, in.[1]
  IE, // Interlingue, Occidental - ile - constructed by Edgar de Wahl, first published in 1922
  IG, // Igbo - ibo
  II, // Sichuan Yi, Nuosu - iii - standard form of the Yi languages
  IK, // Inupiaq - ipk - macrolanguage
  IO, // Ido - ido - constructed by De Beaufront, 1907, as variation of Esperanto
  IS, // Icelandic - isl
  IT, // Italian - ita
  IU, // Inuktitut - iku - macrolanguage
  JA, // Japanese - jpn
  JV, // Javanese - jav
  KA, // Georgian - kat
  KG, // Kongo - kon - macrolanguage
  KI, // Kikuyu, Gikuyu - kik
  KJ, // Kuanyama, Kwanyama - kua
  KK, // Kazakh - kaz
  KL, // Kalaallisut, Greenlandic - kal
  KM, // Central Khmer - khm - also known as Khmer or Cambodian
  KN, // Kannada - kan
  KO, // Korean - kor
  KR, // Kanuri - kau - macrolanguage
  KS, // Kashmiri - kas
  KU, // Kurdish - kur - macrolanguage
  KV, // Komi - kom - macrolanguage
  KW, // Cornish - cor
  KY, // Kirghiz, Kyrgyz - kir
  LA, // Latin - lat - ancient
  LB, // Luxembourgish, Letzeburgesch - ltz
  LG, // Ganda - lug
  LI, // Limburgan, Limburger, Limburgish - lim
  LN, // Lingala - lin
  LO, // Lao - lao
  LT, // Lithuanian - lit
  LU, // Luba-Katanga - lub - also known as Luba-Shaba
  LV, // Latvian - lav - macrolanguage
  MG, // Malagasy - mlg - macrolanguage
  MH, // Marshallese - mah
  MI, // Maori - mri - also known as Māori
  MK, // Macedonian - mkd
  ML, // Malayalam - mal
  MN, // Mongolian - mon - macrolanguage
  MR, // Marathi - mar - also known as Marāṭhī
  MS, // Malay - msa - macrolanguage, Standard Malay is zsm, Indonesian is id/ind
  MT, // Maltese - mlt
  MY, // Burmese - mya - also known as Myanmar
  NA, // Nauru - nau - also known as Nauruan
  NB, // Norwegian Bokmål - nob - covered by macrolanguage no/nor
  ND, // North Ndebele - nde - also known as Northern Ndebele
  NE, // Nepali - nep - macrolanguage
  NG, // Ndonga - ndo
  NL, // Dutch, Flemish - nld - Flemish is not to be confused with the closely related West Flemish which is referred to as Vlaams (Dutch for "Flemish") in ISO 639-3 and has the ISO 639-3 code vls
  NN, // Norwegian Nynorsk - nno - covered by macrolanguage no/nor
  NO, // Norwegian - nor - macrolanguage, Bokmål is nb/nob, Nynorsk is nn/nno
  NR, // South Ndebele - nbl - also known as Southern Ndebele
  NV, // Navajo, Navaho - nav
  NY, // Chichewa, Chewa, Nyanja - nya
  OC, // Occitan - oci
  OJ, // Ojibwa - oji - macrolanguage, also known as Ojibwe
  OM, // Oromo - orm - macrolanguage
  OR, // Oriya - ori - macrolanguage, also known as Odia
  OS, // Ossetian, Ossetic - oss
  PA, // Punjabi, Panjabi - pan
  PI, // Pali - pli - ancient, also known as Pāli
  PL, // Polish - pol
  PS, // Pashto, Pushto - pus - macrolanguage
  PT, // Portuguese - por
  QU, // Quechua - que - macrolanguage
  RM, // Romansh - roh
  RN, // Rundi - run - also known as Kirundi
  RO, // Romanian, Moldavian, Moldovan - ron - the identifiers mo and mol for Moldavian are deprecated. They will not be assigned to different items, and recordings using these identifiers will not be invalid.
  RU, // Russian - rus
  RW, // Kinyarwanda - kin
  SA, // Sanskrit - san - ancient
  SC, // Sardinian - srd - macrolanguage
  SD, // Sindhi - snd
  SE, // Northern Sami - sme
  SG, // Sango - sag
  SI, // Sinhala, Sinhalese - sin
  SK, // Slovak - slk
  SL, // Slovenian - slv - also known as Slovene
  SM, // Samoan - smo
  SN, // Shona - sna
  SO, // Somali - som
  SQ, // Albanian - sqi - macrolanguage, called "Albanian Phylozone" in 639-6
  SR, // Serbian - srp - the ISO 639-2/T code srp deprecated the ISO 639-2/B code scc[2]
  SS, // Swati - ssw - also known as Swazi
  ST, // Southern Sotho - sot
  SU, // Sundanese - sun
  SV, // Swedish - swe
  SW, // Swahili - swa - macrolanguage
  TA, // Tamil - tam
  TE, // Telugu - tel
  TG, // Tajik - tgk
  TH, // Thai - tha
  TI, // Tigrinya - tir
  TK, // Turkmen - tuk
  TL, // Tagalog - tgl - note: Filipino (Pilipino) has the code fil
  TN, // Tswana - tsn
  TO, // Tonga (Tonga Islands) - ton - also known as Tongan
  TR, // Turkish - tur
  TS, // Tsonga - tso
  TT, // Tatar - tat
  TW, // Twi - twi - covered by macrolanguage ak/aka
  TY, // Tahitian - tah - one of the Reo Mā`ohi (languages of French Polynesia)[3]
  UG, // Uighur, Uyghur - uig
  UK, // Ukrainian - ukr
  UR, // Urdu - urd
  UZ, // Uzbek - uzb - macrolanguage
  VE, // Venda - ven
  VI, // Vietnamese - vie
  VO, // Volapük - vol - constructed
  WA, // Walloon - wln
  WO, // Wolof - wol
  XH, // Xhosa - xho
  YI, // Yiddish - yid - macrolanguage. Changed in 1989 from original ISO 639:1988, ji.[1]
  YO, // Yoruba - yor
  ZA, // Zhuang, Chuang - zha - macrolanguage
  ZH, // Chinese - zho - macrolanguage
  ZU, // Zulu - zul
}

impl std::hash::Hash for LangCode {
  fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
    hasher.write_u8(*self as u8)
  }
}

impl std::cmp::PartialEq for LangCode {
  fn eq(&self, other: &Self) -> bool {
    *self as u8 == *other as u8
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn country_code_size_of_eq_1() {
    assert_eq!(std::mem::size_of::<LangCode>(), 1);
  }

  #[test]
  pub fn country_code_size_of_option_eq_1() {
    assert_eq!(std::mem::size_of::<Option<LangCode>>(), 1);
  }
}
