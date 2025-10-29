use std::fmt::Display;

pub struct WordIter<'a> {
    maybe_state: Option<State<'a>>,
    input: &'a str,
}
impl WordIter<'_> {
    pub fn new<'b>(input: &'b str) -> WordIter<'b> {
        let init = State {
            at_char: '\u{0000}',
            char_start: 0,
            char_end: 0,
            rest: input,
        };
        WordIter {
            maybe_state: init.try_next(),
            input,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SplitPart<'a> {
    Delimiter(&'a str),
    Word(&'a str),
}

impl<'a> Iterator for WordIter<'a> {
    type Item = SplitPart<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.maybe_state {
            None => None,
            Some(state) => {
                let start = state.char_start;
                if state.at_char.is_alphabetic() {
                    let chunk = match state.take_while(|c| !c.is_alphabetic()) {
                        TakeWhile::EndOfString => {
                            self.maybe_state = None;
                            &self.input[start..]
                        }
                        TakeWhile::SuccessUntil(end) => {
                            self.maybe_state = Some(end);
                            &self.input[start..end.char_start]
                        }
                    };
                    Some(SplitPart::Word(chunk))
                } else {
                    let chunk = match state.take_while(|c| c.is_alphabetic()) {
                        TakeWhile::EndOfString => {
                            self.maybe_state = None;
                            &self.input[start..]
                        }
                        TakeWhile::SuccessUntil(end) => {
                            self.maybe_state = Some(end);
                            &self.input[start..end.char_start]
                        }
                    };
                    Some(SplitPart::Delimiter(chunk))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State<'a> {
    at_char: char,
    char_start: usize,
    char_end: usize,
    rest: &'a str,
}

enum TakeWhile<'a> {
    SuccessUntil(State<'a>),
    EndOfString,
}

impl<'a> State<'a> {
    fn try_next(&self) -> Option<State<'a>> {
        let mut iter = self.rest.char_indices();
        match (iter.next(), iter.next()) {
            (None, _) => None,
            (Some((at_idx, at_char)), None) => {
                let next = State {
                    at_char,
                    char_start: self.char_end + at_idx,
                    char_end: self.char_end + self.rest.len(),
                    rest: "",
                };
                Some(next)
            }
            (Some((at_idx, at_char)), Some((next_idx, _))) => {
                let next = State {
                    at_char,
                    char_start: self.char_end + at_idx,
                    char_end: self.char_end + next_idx,
                    rest: &self.rest[next_idx..],
                };
                Some(next)
            }
        }
    }

    fn take_while<P>(&self, predicate: P) -> TakeWhile<'a>
    where
        P: Fn(char) -> bool,
    {
        if predicate(self.at_char) {
            TakeWhile::SuccessUntil(*self)
        } else {
            let mut state = *self;
            loop {
                match state.try_next() {
                    None => break TakeWhile::EndOfString,
                    Some(next) => {
                        if predicate(next.at_char) {
                            break TakeWhile::SuccessUntil(next);
                        } else {
                            state = next;
                            continue;
                        }
                    }
                }
            }
        }
    }
}
impl<'a> Display for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "State(at_char: {:?}, at_idx: {}-{}, rest: {:?})",
            self.at_char, self.char_start, self.char_end, self.rest
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_cases() {
        assert_eq!(
            WordIter::new("").collect::<Vec<SplitPart>>().to_owned(),
            vec![]
        );
        assert_eq!(
            WordIter::new(" ").collect::<Vec<SplitPart>>().to_owned(),
            vec![SplitPart::Delimiter(" ")]
        );
        assert_eq!(
            WordIter::new("a").collect::<Vec<SplitPart>>().to_owned(),
            vec![SplitPart::Word("a")]
        );
    }

    #[test]
    fn single_word() {
        assert_eq!(
            WordIter::new("input")
                .collect::<Vec<SplitPart>>()
                .to_owned(),
            vec![SplitPart::Word("input")]
        );
    }

    #[test]
    fn single_delimiter() {
        assert_eq!(
            WordIter::new(" , ").collect::<Vec<SplitPart>>().to_owned(),
            vec![SplitPart::Delimiter(" , ")]
        );
    }

    #[test]
    fn word_surrounded_by_delimiters() {
        assert_eq!(
            WordIter::new("{input}")
                .collect::<Vec<SplitPart>>()
                .to_owned(),
            vec![
                SplitPart::Delimiter("{"),
                SplitPart::Word("input"),
                SplitPart::Delimiter("}"),
            ]
        );
    }

    #[test]
    fn delimiter_surrounded_by_words() {
        assert_eq!(
            WordIter::new("this that")
                .collect::<Vec<SplitPart>>()
                .to_owned(),
            vec![
                SplitPart::Word("this"),
                SplitPart::Delimiter(" "),
                SplitPart::Word("that"),
            ]
        );
    }

    #[test]
    fn sentence() {
        assert_eq!(
            WordIter::new("sphinx of black quartz, judge my vow!")
                .collect::<Vec<SplitPart>>()
                .to_owned(),
            vec![
                SplitPart::Word("sphinx"),
                SplitPart::Delimiter(" "),
                SplitPart::Word("of"),
                SplitPart::Delimiter(" "),
                SplitPart::Word("black"),
                SplitPart::Delimiter(" "),
                SplitPart::Word("quartz"),
                SplitPart::Delimiter(", "),
                SplitPart::Word("judge"),
                SplitPart::Delimiter(" "),
                SplitPart::Word("my"),
                SplitPart::Delimiter(" "),
                SplitPart::Word("vow"),
                SplitPart::Delimiter("!"),
            ]
        );
    }
}
