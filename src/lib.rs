pub mod table;

#[cfg(test)]
mod tests {
    use super::maybe_sanskrit;
    use super::letter_partition;
    use super::w_letter;
    use super::get_root;
    // use super::vowel_indices;
    // use super::analyze_root;
    // use super::Letter;
    use super::Slice;
    use super::LetterType;
    use super::Word;
    use table;

    // #[test]
    // fn test_root_analyzer() {
    //     let parts = letter_partition("a", &table::W_SORTED_ALPHABET);
    //     assert_eq!(analyze_root("a", parts, 0), vec![Syllable::Root])
    // }

    #[test]
    fn test_get_root() {
        assert_eq!(
            get_root("a", vec![0], vec![Slice{i: 0, len: 1}]),
            vec![LetterType::Root]
        );

        assert_eq!(
            get_root("ba",
                     vec![1],
                     vec![Slice{i: 0, len: 1}, Slice{i: 1, len: 1}]),
            vec![LetterType::Root, LetterType::Vowel]
        );

        assert_eq!(
            get_root("bya",
                     vec![2],
                     vec![Slice{i: 0, len: 1},
                          Slice{i: 1, len: 1},
                          Slice{i: 2, len: 1}]),
            vec![LetterType::Root, LetterType::Subjoined, LetterType::Vowel]
        );

        assert_eq!(
            get_root("rja",
                     vec![2],
                     vec![Slice{i: 0, len: 1},
                          Slice{i: 1, len: 1},
                          Slice{i: 2, len: 1}]),
            vec![LetterType::Super, LetterType::Root, LetterType::Vowel]
        );

        assert_eq!(
            get_root("g.ya",
                     vec![2],
                     vec![Slice{i: 0, len: 2},
                          Slice{i: 2, len: 1},
                          Slice{i: 3, len: 1}]),
            vec![LetterType::Prefix, LetterType::Root, LetterType::Vowel]
        );

        assert_eq!(
            get_root("dba",
                     vec![2],
                     vec![Slice{i: 0, len: 1},
                          Slice{i: 1, len: 1},
                          Slice{i: 2, len: 1}]),
            vec![LetterType::Prefix, LetterType::Root, LetterType::Vowel]
        );

        assert_eq!(
            get_root("srwa",
                     vec![3],
                     vec![Slice{i: 0, len: 1},
                          Slice{i: 1, len: 1},
                          Slice{i: 2, len: 1},
                          Slice{i: 3, len: 1}]),
            vec![
                LetterType::Root, LetterType::Subjoined,
                LetterType::Subjoined2, LetterType::Vowel
            ]
        );

    }

    #[test]
    fn test_letter_slice() {
        let w = "sangs";
        let Word {string: w, root: vowel_indices, letters: slices} = letter_partition(
            &w, &table::W_SORTED_ALPHABET
        );

        assert_eq!(vowel_indices[0], 1);
        assert_eq!(w_letter(w, &slices[0]), "s");
        assert_eq!(w_letter(w, &slices[1]), "a");
        assert_eq!(w_letter(w, &slices[2]), "ng");
        assert_eq!(w_letter(w, &slices[3]), "s");
    }

    #[test]
    fn test_letter_partition() {
        let mut w = "sangs";
        assert_eq!(
            letter_partition(w, &table::W_SORTED_ALPHABET),
            Word { 
                string: w,
                root: vec![1],
                letters: vec![
                    Slice{i: 0, len: 1},
                    Slice{i: 1, len: 1},
                    Slice{i: 2, len: 2},
                    Slice{i: 4, len: 1}
                ]
             }
        );
        w = "'tshags";
        assert_eq!(
            letter_partition(w, &table::W_SORTED_ALPHABET),
            // vec!["'", "tsh", "a", "g", "s"]);
            Word { 
                string: w,
                root: vec![2],
                letters: vec![
                    Slice{i: 0, len: 1},
                    Slice{i: 1, len: 3},
                    Slice{i: 4, len: 1},
                    Slice{i: 5, len: 1},
                    Slice{i: 6, len: 1}
                ]
             }
        );

        w = "g.yag";
        assert_eq!(
            letter_partition(w, &table::W_SORTED_ALPHABET),
            Word {
                string: w,
                root: vec![2],
                letters: vec![
                    Slice{i: 0, len: 2},
                    Slice{i: 2, len: 1},
                    Slice{i: 3, len: 1},
                    Slice{i: 4, len: 1}
                ]
            }
        );
    }

    #[test]
    fn test_quickcheck() {
        let sanskrit: [&'static str; 6] = ["sarva", "ai", "au", "akṣye", "vajra", "kyai"];

        for s in &sanskrit {
            assert!(maybe_sanskrit(s));
        }
    }

    // #[test]
    // fn test_vowel_indexer() {
    //     assert_eq!(vowel_indices("sarva", &table::TIBETAN_VOWELS), vec![1, 4]);
    // }
}

// // #[derive(Copy, Clone, Debug, PartialEq)]
// #[derive(Debug, PartialEq)]
// enum Letter {
//     Consonant,
//     Vowel
// }

// #[derive(Copy, Clone, Debug, PartialEq)]
#[derive(Debug, PartialEq)]
enum LetterType {
    Vowel,
    Prefix,
    Super,
    Root,
    Subjoined,
    Subjoined2,
    // Suffix,
    // Suffix2,
    // Genitive,
    // GenVowel
}

#[derive(Debug, PartialEq)]
struct Slice {
    i: usize,
    len: usize,
}

#[derive(Debug, PartialEq)]
struct Letter {
    slice: Slice,
    category: LetterType
}

#[derive(Debug, PartialEq)]
struct WordInfo {
    root: Vec<usize>,
    letters: Vec<Slice>
}

#[derive(Debug, PartialEq)]
struct Word<'a> {
    string: &'a str,
    root: Vec<usize>,
    letters: Vec<Slice>
}

// impl Letter {
//     pub fn str(self) -> &str {
//         self
//     }
// }

impl<'a> Word<'a> {
    pub fn letter(self, index: usize) -> &'a str {
        w_letter(self.string, &self.letters[index])
    }
    // pub fn to_unicode(self) -> String {
    //     // self.letters.map(|l| l.to_unicode()).collect();
    //     "".to_string()
    // }
}

// // TODO: conjoin neighbouring vowels to count as one vowel..
// fn vowel_indices(string: &str, vowels: &[char]) -> Vec<usize> {
//     let indices: Vec<usize> = string.chars()
//         .enumerate()
//         .filter(|&(_, c)| vowels.contains(&c))
//         .map(|(i, _)| i)
//         .collect();

//     indices
// }

// fn letter(string: &str, slice: (usize, usize)) -> &str {
//     &string[slice.0..slice.0+slice.1]
// }

// fn analyze_root<'a>(string: &str, parts: &'a Vec<Letter>) -> Vec<Letter> {
// }

fn get_root(string: &str, vowel_indices: Vec<usize>, slices: Vec<Slice>) -> Vec<LetterType> {
    let mut result: Vec<LetterType> = Vec::new();

    if vowel_indices[0] == 0 {
        result.push(LetterType::Root);
        return result;
    } else if vowel_indices[0] == 1 {
        if table::W_CONSONANTS.contains(&w_letter(string, &slices[0])) {
            result.push(LetterType::Root);
        } // TODO: raise error on else
    } else if vowel_indices[0] == 2 {
        if is_subscribed(string, vowel_indices[0], &slices) {
            result.push(LetterType::Root);
            result.push(LetterType::Subjoined);
        } else if is_superscribed(string, vowel_indices[0], &slices) {
            result.push(LetterType::Super);
            result.push(LetterType::Root);
        } else if table::PREFIXES.contains(&w_letter(string, &slices[0]))
                && table::W_CONSONANTS.contains(&w_letter(string, &slices[1])) {
            result.push(LetterType::Prefix);
            result.push(LetterType::Root);
        }
    } else if vowel_indices[0] == 3 {
        if w_letter(string, &slices[2]) == "w" && w_letter(string, &slices[1]) == "r" {
            result.push(LetterType::Root);
            result.push(LetterType::Subjoined);
            result.push(LetterType::Subjoined2);
        } else if is_superscribed(string, vowel_indices[0], &slices) {
            result.push(LetterType::Prefix);
            result.push(LetterType::Super);
            result.push(LetterType::Root);
        } else if is_subscribed(string, vowel_indices[0], &slices) {
            result.push(LetterType::Prefix);
            result.push(LetterType::Root);
            result.push(LetterType::Subjoined);
        } else if table::SUPERJOINED.contains(&w_letter(string, &slices[1]))
                && table::W_CONSONANTS.contains(&w_letter(string, &slices[2]))
                && table::SUBJOINED.contains(&w_letter(string, &slices[3])) {
            result.push(LetterType::Super);
            result.push(LetterType::Root);
            result.push(LetterType::Subjoined);
        }
    } else if vowel_indices[0] == 4 {
        if !(table::PREFIXES.contains(&w_letter(string, &slices[0]))
            && table::SUPERJOINED.contains(&w_letter(string, &slices[1]))
            && table::W_CONSONANTS.contains(&w_letter(string, &slices[2]))
            && table::SUBJOINED.contains(&w_letter(string, &slices[3]))) {
            // TODO raise error!
        }

        result.push(LetterType::Prefix);
        result.push(LetterType::Super);
        result.push(LetterType::Root);
        result.push(LetterType::Subjoined);
    }

    result.push(LetterType::Vowel);
    result
}

fn w_letter<'a>(string: &'a str, slice: &'a Slice) -> &'a str {
    &string[slice.i..slice.i+slice.len]
}

fn letter_partition<'a>(string: &'a str, alphabet: &[&'static str]) -> Word<'a> {
    let mut result: Vec<Slice> = Vec::new();
    let mut vowel_indices: Vec<usize> = Vec::new();
    let mut progress = 0;

    while progress < string.len() {
        for (i, letter) in alphabet.iter().enumerate() {
            let slice = &string[progress..];
            let g_prefix_edge_case = slice.starts_with("g.");

            if !(g_prefix_edge_case || slice.starts_with(letter)) {
                if i == alphabet.len() - 1 {
                    // TODO: raise exception invalid tibetan character!
                    progress = string.len();
                }
                continue;
            }

            let letter_length =
                if g_prefix_edge_case {
                    2
                } else {
                    letter.len()
                };

            result.push(Slice{i: progress, len: letter_length});

            if i >= table::W_SORTED_ALPHABET.len() - table::TIBETAN_VOWELS.len() {
                vowel_indices.push(result.len() - 1)
            }

            progress += letter_length;
            break;
        }
    }

    Word {string: string, root: vowel_indices, letters: result}
}

fn maybe_sanskrit(string: &str) -> bool {
    if string.len() == 3 && table::S_DOUBLE_CONSONANTS.contains(&&string[0..2]) {
        return true;
    }

    for r in &table::S_BASIC_RULES {
        if string.starts_with(r) {
            return true;
        }
    }

    if string.contains("ai") || string.contains("au") {
        return true;
    }

    let mut vowel_count = 0;

    for v in &table::TIBETAN_VOWELS {
        let m: Vec<&str> = string.matches(&v.to_string()).collect();
        vowel_count += m.len();
    }

    // achung
    !string.contains(table::W_CONSONANTS[22]) && vowel_count > 1
}

fn is_subscribed(string: &str, vowel_index: usize, slices: &Vec<Slice>) -> bool {
    if vowel_index == 2 {
        !valid_superscribe(w_letter(string, &slices[0]),
                            w_letter(string, &slices[1]))
        && valid_subscribe(w_letter(string, &slices[0]),
                           w_letter(string, &slices[1]))
    } else {    // vowel_index == 3
        table::PREFIXES.contains(&w_letter(string, &slices[0]))
        && !valid_superscribe(w_letter(string, &slices[1]),
                            w_letter(string, &slices[2]))
        && valid_subscribe(w_letter(string, &slices[1]),
                           w_letter(string, &slices[2]))
    }
}

fn is_superscribed(string: &str, vowel_index: usize, slices: &Vec<Slice>) -> bool {
    if vowel_index == 2 {
        valid_superscribe(w_letter(string, &slices[0]),
                          w_letter(string, &slices[1]))
        && !valid_subscribe(w_letter(string, &slices[0]),
                            w_letter(string, &slices[1]))
    } else {    // vowel_index == 3
        table::PREFIXES.contains(&w_letter(string, &slices[0]))
        && valid_superscribe(w_letter(string, &slices[1]),
                             w_letter(string, &slices[2]))
        && !valid_subscribe(w_letter(string, &slices[1]),
                            w_letter(string, &slices[2]))
    }
}

fn valid_superscribe(head_letter: &str, root_letter: &str) -> bool {
    table::SUPERJOINED.contains(&head_letter)
    && table::SUPERJOINABLE.contains(&root_letter)
}

fn valid_subscribe(root_letter: &str, subjoined_letter: &str) -> bool {
    table::SUBJOINED.contains(&subjoined_letter)
    && table::SUBJOINABLE.contains(&root_letter)
}

// fn to_unicode() -> String {
// }
