use crate::inclusive_split::{self, SplitPart};

/// Convert strings to pig latin.
///
/// The first consonant of each word is moved to the end of the word and ay is
/// added, so first becomes irst-fay.
///
/// Words that start with a vowel have hay added to the end instead (apple
/// becomes apple-hay).
///
/// The letter 'y' is a treated as a consonant if it appears as the first letter
/// of a word (yard -> ard-yay), but as a vowel if it appears at an interior
/// location (crybaby -> ybaby-cray).
/// (Best-effort stolen from https://www.eecis.udel.edu/~saunders/nov02/piglatin.html)
///
/// Keep in mind the details about UTF-8 encoding!
pub fn piggy(input: &str) -> String {
    inclusive_split::Iter::new(input, |c: &char| c.is_alphabetic())
        .fold(String::new(), |mut accum, sp| {
            match sp {
                SplitPart::NotMatch(delim) => accum.push_str(delim),
                SplitPart::Match(word) => accum.push_str(&piggy_word(word)),
            };
            accum
        })
        .to_owned()
}

fn is_consonant(c: &char) -> bool {
    !matches!(c, 'A' | 'a' | 'E' | 'e' | 'I' | 'i' | 'O' | 'o' | 'U' | 'u')
}

fn is_vowel_or_y(c: &char) -> bool {
    matches!(
        c,
        'A' | 'a' | 'E' | 'e' | 'I' | 'i' | 'O' | 'o' | 'U' | 'u' | 'Y' | 'y'
    )
}

fn piggy_word(word: &str) -> String {
    if word.is_empty() {
        String::new()
    } else {
        let mut chunks = inclusive_split::Iter::new(
            word,
            inclusive_split::ComplexPattern {
                match_start: is_consonant,
                match_end: is_vowel_or_y,
            },
        );
        match chunks.next() {
            None => String::new(),
            Some(part) => match part {
                SplitPart::NotMatch(_vowel) => format!("{}-hay", word),
                SplitPart::Match(consonant_cluster) => {
                    let rest = chunks.map(|p| p.unwrap()).collect::<String>();
                    format!("{}-{}ay", rest, consonant_cluster)
                }
            },
        }
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
    fn y_is_wierd() {
        assert_eq!(piggy("crybaby yard"), String::from("ybaby-cray ard-yay"));
    }

    #[test]
    fn sentences() {
        assert_eq!(
            piggy("sphinx of black quartz, judge my vow!"),
            String::from("inx-sphay of-hay ack-blay uartz-qay, udge-jay y-may ow-vay!")
        )
    }
}
