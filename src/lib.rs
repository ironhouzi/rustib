pub mod table;

#[cfg(test)]
mod tests {
    use super::maybe_sanskrit;
    use super::letter_partition;
    use super::vowel_indices;
    use table;

    #[test]
    fn test_letter_partition() {
        assert_eq!(letter_partition("sangs", &table::W_SORTED_ALPHABET),
                   vec!["s", "a", "ng", "s"]);
        assert_eq!(letter_partition("'tshags", &table::W_SORTED_ALPHABET),
                   vec!["'", "tsh", "a", "g", "s"]);
    }

    #[test]
    fn test_quickcheck() {
        let sanskrit: [&'static str; 6] = ["sarva", "ai", "au", "akṣye", "vajra", "kyai"];

        for s in &sanskrit {
            assert!(maybe_sanskrit(s.to_string()));
        }
    }

    #[test]
    fn test_vowel_indexer() {
        assert_eq!(vowel_indices("sarva".to_string(), &table::TIBETAN_VOWELS),
                   vec![1, 4]);
    }
}

fn vowel_indices(string: String, vowels: &[&'static str]) -> Vec<usize> {
    let indices: Vec<usize> = string.chars()
        .enumerate()
        .filter(|&(i, c)| vowels.contains(&c))
        .map(|(i, c)| i)
        .collect();

    indices
}

fn letter_partition(string: &str, alphabet: &[&'static str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut progress = 0;

    while progress < string.len() {
        for c in alphabet {
            if !&string[progress..string.len()].starts_with(c) {
                continue;
            }

            result.push(c.to_string());
            progress += c.len();
            break;
        }
    }

    result
}

fn maybe_sanskrit(string: String) -> bool {
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
        let m: Vec<&str> = string.matches(v).collect();
        vowel_count += m.len();
    }

    // achung
    !string.contains(table::W_ROOTLETTERS[22]) && vowel_count > 1
}
