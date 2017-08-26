// Wylie/latin consonants
pub static W_ROOTLETTERS: [&'static str; 30] = [
    "k", "kh", "g", "ng",
    "c", "ch", "j", "ny",
    "t", "th", "d", "n",
    "p", "ph", "b", "m",
    "ts", "tsh", "dz", "w",
    "zh", "z", "'", "y",
    "r", "l", "sh", "s",
    "h", "a"
];

pub static W_SORTED_ALPHABET: [&'static str; 34] = [
    "tsh", "kh", "ng", "ch",
    "ny", "th", "ph", "ts",
    "dz", "zh", "sh", "k",
    "g", "c", "j", "t",
    "d", "n", "p", "b",
    "m", "w", "z", "'",
    "y", "r", "l", "s",
    "h", "a", "i", "u",
    "e", "o"
];

// Tibetan Unicode consonants
pub static U_ROOTLETTERS: [&'static str; 30] = [
    "\u{0f40}", "\u{0f41}", "\u{0f42}", "\u{0f44}",
    "\u{0f45}", "\u{0f46}", "\u{0f47}", "\u{0f49}",
    "\u{0f4f}", "\u{0f50}", "\u{0f51}", "\u{0f53}",
    "\u{0f54}", "\u{0f55}", "\u{0f56}", "\u{0f58}",
    "\u{0f59}", "\u{0f5a}", "\u{0f5b}", "\u{0f5d}",
    "\u{0f5e}", "\u{0f5f}", "\u{0f60}", "\u{0f61}",
    "\u{0f62}", "\u{0f63}", "\u{0f64}", "\u{0f66}",
    "\u{0f67}", "\u{0f68}"
];

// Latin consonants for transliteration of sanskrit (IAST)
// TODO: replace char litterals with codepoints.
pub static SW_ROOTLETTERS: [&'static str; 34] = [
    "k", "kh", "g", "gh", "ṅ", "c", "ch", "j", "jh",
    "ñ", "ṭ", "ṭh", "ḍ", "ḍh", "ṇ", "t",
    "th", "d", "dh", "n", "p", "ph", "b", "bh", "m",
    "y", "r", "l", "v", "ś", "ṣ", "s", "h", "kṣ"
];

pub static SW_SORTED_ALPHABET: [&'static str; 50] = [
    "kh", "gh", "ch", "jh", "ṭh", "ḍh", "th", "dh", "ph",
    "bh", "kṣ", "ai", "au", "k", "g", "ṅ", "c", "j", "ñ",
    "ṭ", "ḍ", "ṇ", "t", "d", "n", "p", "b", "m", "y", "r",
    "l", "v", "ś", "ṣ", "s", "h", "a", "ā", "i",
    "ī", "u", "ū", "e", "o", "ṛ",
    "ṝ", "ḷ", "ḹ", "ṃ", "ḥ"];

// Tibetan Unicode consonants for Tibetan transliteration of Sanskrit
pub static SU_ROOTLETTERS: [&'static str; 34] = [
    "\u{0f40}", "\u{0f41}", "\u{0f42}", "\u{0f43}", "\u{0f44}", "\u{0f45}",
    "\u{0f46}", "\u{0f5b}", "\u{0f5c}", "\u{0f49}", "\u{0f4a}", "\u{0f4b}",
    "\u{0f4c}", "\u{0f4d}", "\u{0f4e}", "\u{0f4f}", "\u{0f50}", "\u{0f51}",
    "\u{0f52}", "\u{0f53}", "\u{0f54}", "\u{0f55}", "\u{0f56}", "\u{0f57}",
    "\u{0f58}", "\u{0f61}", "\u{0f62}", "\u{0f63}", "\u{0f5d}", "\u{0f64}",
    "\u{0f65}", "\u{0f66}", "\u{0f67}", "\u{0f69}"
];

// Wylie/latin vowels
pub static W_VOWELS: [&'static str; 4] = ["i", "u", "e", "o"];

pub static TIBETAN_VOWELS: [&'static str; 5] = ["i", "u", "e", "o", "a"];

// Tibetan Unicode vowels
pub static U_VOWELS: [&'static str; 4] = ["\u{0f72}", "\u{0f74}", "\u{0f7a}", "\u{0f7c}"];

// Latin vowels for transliteration of sanskrit (IAST)
// TODO: replace char litterals with codepoints.
pub static SW_VOWELS: [&'static str; 16] = [
    "a", "ā", "i", "ī", "u", "ū", "e", "ai",
    "o", "au", "ṛ", "ṝ", "ḷ", "ḹ", "ṃ", "ḥ"
];

// Tibetan Unicode vowels for Tibetan transliteration of Sanskrit
pub static SU_VOWELS: [&'static str; 16] = ["\u{0f68}",
                                           "\u{0f71}",
                                           "\u{0f72}",
                                           "\u{0f73}",
                                           "\u{0f74}",
                                           "\u{0f75}",
                                           "\u{0f7a}",
                                           "\u{0f7b}",
                                           "\u{0f7c}",
                                           "\u{0f7d}",
                                           "\u{0f76}",
                                           "\u{0f77}",
                                           "\u{0f78}",
                                           "\u{0f79}",
                                           "\u{0f7e}",
                                           "\u{0f7f}"];

// TODO: replace char literals with codepoints.
pub static U_OM: &'static str = "oṃ";
pub static U_STACKED_YA: &'static str = "\u{0fbb}";
pub static U_STACKED_RA: &'static str = "\u{0fbc}";

pub static TSHEG: &'static str = "\u{0f0b}";
pub static S_SPACE: &'static str = "\u{00a0}";
pub static S_SHAD: &'static str = "\u{0f0d}";
pub static S_NYIS_SHAD: &'static str = "\u{0f0e}";
pub static S_SNA_LDAN: &'static str = "\u{0f83}";
pub static S_OM: &'static str = "\u{0f00}";
pub static S_DOUBLE_CONSONANTS: [&'static str; 3] = ["gg", "dd", "bb"];
pub static S_BASIC_RULES: [&'static str; 25] =
    ["phyw", "ghr", "hra", "hwa", "tsy", "trw", "rdh", "sye", "n.y", "gh", "dh", "cy", "jh", "nn",
     "mm", "ww", "yy", "rr", "hy", "ty", "tv", "tw", "tz", "bh", "ss"];
