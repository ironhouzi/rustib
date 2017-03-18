// Wylie/latin consonants
pub const W_ROOTLETTERS: [&'static str; 30] =
    ["k", "kh", "g", "ng", "c", "ch", "j", "ny", "t", "th", "d", "n", "p", "ph", "b", "m", "ts",
     "tsh", "dz", "w", "zh", "z", "'", "y", "r", "l", "sh", "s", "h", "a"];

pub const W_SORTED_ALPHABET: [&'static str; 34] =
    ["tsh", "kh", "ng", "ch", "ny", "th", "ph", "ts", "dz", "zh", "sh",
    "k", "g", "c", "j", "t", "d", "n", "p", "b", "m", "w", "z", "'", "y",
    "r", "l", "s", "h", "a", "i", "u", "e", "o"];

// Tibetan Unicode consonants
pub const U_ROOTLETTERS: [&'static str; 30] =
    ["\u{0f40}", "\u{0f41}", "\u{0f42}", "\u{0f44}", "\u{0f45}", "\u{0f46}", "\u{0f47}",
     "\u{0f49}", "\u{0f4f}", "\u{0f50}", "\u{0f51}", "\u{0f53}", "\u{0f54}", "\u{0f55}",
     "\u{0f56}", "\u{0f58}", "\u{0f59}", "\u{0f5a}", "\u{0f5b}", "\u{0f5d}", "\u{0f5e}",
     "\u{0f5f}", "\u{0f60}", "\u{0f61}", "\u{0f62}", "\u{0f63}", "\u{0f64}", "\u{0f66}",
     "\u{0f67}", "\u{0f68}"];

// Latin consonants for transliteration of sanskrit (IAST)
// TODO: replace char litterals with codepoints.
pub const SW_ROOTLETTERS: [&'static str; 34] = ["k", "kh", "g", "gh", "ṅ", "c", "ch", "j", "jh",
                                                "ñ", "ṭ", "ṭh", "ḍ", "ḍh", "ṇ", "t",
                                                "th", "d", "dh", "n", "p", "ph", "b", "bh", "m",
                                                "y", "r", "l", "v", "ś", "ṣ", "s", "h", "kṣ"];

pub const SW_SORTED_ALPHABET: [&'static str; 50] = [
    "kh", "gh", "ch", "jh", "ṭh", "ḍh", "th", "dh", "ph",
    "bh", "kṣ", "ai", "au", "k", "g", "ṅ", "c", "j", "ñ",
    "ṭ", "ḍ", "ṇ", "t", "d", "n", "p", "b", "m", "y", "r",
    "l", "v", "ś", "ṣ", "s", "h", "a", "ā", "i",
    "ī", "u", "ū", "e", "o", "ṛ",
    "ṝ", "ḷ", "ḹ", "ṃ", "ḥ"];

// Tibetan Unicode consonants for Tibetan transliteration of Sanskrit
pub const SU_ROOTLETTERS: [&'static str; 34] = [U_ROOTLETTERS[0],
                                                U_ROOTLETTERS[1],
                                                U_ROOTLETTERS[2],
                                                "\u{0f43}",
                                                U_ROOTLETTERS[3],
                                                U_ROOTLETTERS[4],
                                                U_ROOTLETTERS[5],
                                                U_ROOTLETTERS[18],
                                                "\u{0f5c}",
                                                U_ROOTLETTERS[7],
                                                "\u{0f4a}",
                                                "\u{0f4b}",
                                                "\u{0f4c}",
                                                "\u{0f4d}",
                                                "\u{0f4e}",
                                                U_ROOTLETTERS[8],
                                                U_ROOTLETTERS[9],
                                                U_ROOTLETTERS[10],
                                                "\u{0f52}",
                                                U_ROOTLETTERS[11],
                                                U_ROOTLETTERS[12],
                                                U_ROOTLETTERS[13],
                                                U_ROOTLETTERS[14],
                                                "\u{0f57}",
                                                U_ROOTLETTERS[15],
                                                U_ROOTLETTERS[23],
                                                U_ROOTLETTERS[24],
                                                U_ROOTLETTERS[25],
                                                U_ROOTLETTERS[19],
                                                U_ROOTLETTERS[26],
                                                "\u{0f65}",
                                                U_ROOTLETTERS[27],
                                                U_ROOTLETTERS[28],
                                                "\u{0f69}"];

// Wylie/latin vowels
pub const W_VOWELS: [&'static str; 4] = ["i", "u", "e", "o"];

pub const TIBETAN_VOWELS: [&'static str; 5] = ["i", "u", "e", "o", "a"];

// Tibetan Unicode vowels
pub const U_VOWELS: [&'static str; 4] = ["\u{0f72}", "\u{0f74}", "\u{0f7a}", "\u{0f7c}"];

// Latin vowels for transliteration of sanskrit (IAST)
// TODO: replace char litterals with codepoints.
pub const SW_VOWELS: [&'static str; 16] = ["a",
                                           "ā",
                                           W_VOWELS[0],
                                           "ī",
                                           W_VOWELS[1],
                                           "ū",
                                           W_VOWELS[2],
                                           "ai",
                                           W_VOWELS[3],
                                           "au",
                                           "ṛ",
                                           "ṝ",
                                           "ḷ",
                                           "ḹ",
                                           "ṃ",
                                           "ḥ"];

// Tibetan Unicode vowels for Tibetan transliteration of Sanskrit
pub const SU_VOWELS: [&'static str; 16] = ["\u{0f68}",
                                           "\u{0f71}",
                                           U_VOWELS[0],
                                           "\u{0f73}",
                                           U_VOWELS[1],
                                           "\u{0f75}",
                                           U_VOWELS[2],
                                           "\u{0f7b}",
                                           U_VOWELS[3],
                                           "\u{0f7d}",
                                           "\u{0f76}",
                                           "\u{0f77}",
                                           "\u{0f78}",
                                           "\u{0f79}",
                                           "\u{0f7e}",
                                           "\u{0f7f}"];

// TODO: replace char literals with codepoints.
pub const U_OM: &'static str = "oṃ";
pub const U_STACKED_YA: &'static str = "\u{0fbb}";
pub const U_STACKED_RA: &'static str = "\u{0fbc}";

pub const TSHEG: &'static str = "\u{0f0b}";
pub const S_SPACE: &'static str = "\u{00a0}";
pub const S_SHAD: &'static str = "\u{0f0d}";
pub const S_NYIS_SHAD: &'static str = "\u{0f0e}";
pub const S_SNA_LDAN: &'static str = "\u{0f83}";
pub const S_OM: &'static str = "\u{0f00}";
pub const S_DOUBLE_CONSONANTS: [&'static str; 3] = ["gg", "dd", "bb"];
pub const S_BASIC_RULES: [&'static str; 25] =
    ["phyw", "ghr", "hra", "hwa", "tsy", "trw", "rdh", "sye", "n.y", "gh", "dh", "cy", "jh", "nn",
     "mm", "ww", "yy", "rr", "hy", "ty", "tv", "tw", "tz", "bh", "ss"];
