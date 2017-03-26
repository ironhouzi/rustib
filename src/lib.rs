pub mod table;

fn sanskrit_quickcheck(string: &str) -> bool {
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

// fn letter_partition<'a>(string: &'a str, alphabet: &[&'static str]) -> &'a [&'a str] {
fn letter_partition<'a>(string: &'a str, alphabet: &[&'static str]) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let mut progress = 0;

    while progress < string.len() {
        for c in alphabet {
            if !&string[progress..string.len()].starts_with(c) {
                continue;
            }

            result.push(c);
            progress += c.len();
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::sanskrit_quickcheck;
    use super::letter_partition;
    use table;

    #[test]
    fn test_letter_partition() {
        assert!(letter_partition("sangs", &table::W_SORTED_ALPHABET) == vec!["s", "a", "ng", "s"]);
    }

    #[test]
    fn test_quickcheck() {
        let sanskrit: [&'static str; 6] = ["sarva", "ai", "au", "akṣye", "vajra", "kyai"];

        for s in &sanskrit {
            assert!(sanskrit_quickcheck(s));
        }
    }
}
