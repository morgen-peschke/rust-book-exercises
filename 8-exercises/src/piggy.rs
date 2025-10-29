use crate::word_iter::{SplitPart, WordIter};

/// Convert strings to pig latin.
///
/// The first consonant of each word is moved to the end of the word and ay is
/// added, so first becomes irst-fay.
///
/// Words that start with a vowel have hay added to the end instead (apple
/// becomes apple-hay).
///
/// Keep in mind the details about UTF-8 encoding!
pub fn piggy(input: &str) -> String {
    WordIter::new(input)
        .fold(String::new(), |mut accum, sp| {
            match sp {
                SplitPart::Delimiter(delim) => accum.push_str(delim),
                SplitPart::Word(word) => accum.push_str(&piggy_word(word)),
            };
            accum
        })
        .to_owned()
}

fn piggy_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => match first {
            'A' | 'a' | 'E' | 'e' | 'I' | 'i' | 'O' | 'o' | 'U' | 'u' => {
                format!("{}{}-hay", first, chars.collect::<String>())
            }
            _ => format!("{}-{}ay", chars.collect::<String>(), first),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_cases() {
        assert_eq!(piggy(""), String::from(""));
        assert_eq!(piggy("a"), String::from("a-hay"));
        assert_eq!(piggy("i"), String::from("i-hay"));
    }

    #[test]
    fn vowel_words() {
        assert_eq!(piggy("apple"), String::from("apple-hay"));
    }

    #[test]
    fn consonant_words() {
        assert_eq!(piggy("first"), String::from("irst-fay"));
    }

    #[test]
    fn sentences() {
        assert_eq!(
            piggy("sphinx of black quartz, judge my vow!"),
            String::from("phinx-say of-hay lack-bay uartz-qay, udge-jay y-may ow-vay!")
        )
    }
}
