use std::fmt::Display;

pub struct Iter<'a, SP: SplitPattern> {
    maybe_state: Option<State<'a>>,
    input: &'a str,
    pattern: SP,
}
impl<SP: SplitPattern> Iter<'_, SP> {
    pub fn new<'b>(input: &'b str, pattern: SP) -> Iter<'b, SP> {
        let init = State {
            at_char: '\u{0000}',
            char_start: 0,
            char_end: 0,
            rest: input,
        };
        Iter {
            maybe_state: init.try_next(),
            input,
            pattern,
        }
    }
}

pub trait SplitPattern {
    fn match_start(&self, c: &char) -> bool;
    fn match_end(&self, c: &char) -> bool;
}
impl<F> SplitPattern for F
where
    F: Fn(&char) -> bool,
{
    fn match_start(&self, c: &char) -> bool {
        self(c)
    }

    fn match_end(&self, c: &char) -> bool {
        !self(c)
    }
}
pub struct ComplexPattern<FS, FE>
where
    FS: Fn(&char) -> bool,
    FE: Fn(&char) -> bool,
{
    pub match_start: FS,
    pub match_end: FE,
}
impl<FS, FE> SplitPattern for ComplexPattern<FS, FE>
where
    FS: Fn(&char) -> bool,
    FE: Fn(&char) -> bool,
{
    fn match_start(&self, c: &char) -> bool {
        (self.match_start)(c)
    }

    fn match_end(&self, c: &char) -> bool {
        (self.match_end)(c)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SplitPart<'a> {
    NotMatch(&'a str),
    Match(&'a str),
}
impl<'a> SplitPart<'a> {
    pub fn unwrap(&self) -> &'a str {
        match self {
            SplitPart::NotMatch(s) => s,
            SplitPart::Match(s) => s,
        }
    }
}

impl<'a, SP: SplitPattern> Iterator for Iter<'a, SP> {
    type Item = SplitPart<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.maybe_state {
            None => None,
            Some(state) => {
                let start = state.char_start;
                if self.pattern.match_start(&state.at_char) {
                    let chunk = match state
                        .try_next()
                        .map(|s| s.take_until(|c| self.pattern.match_end(&c)))
                    {
                        Some(TakeUntil::EndOfString) | None => {
                            self.maybe_state = None;
                            &self.input[start..]
                        }
                        Some(TakeUntil::SuccessAt(end)) => {
                            self.maybe_state = Some(end);
                            &self.input[start..end.char_start]
                        }
                    };
                    Some(SplitPart::Match(chunk))
                } else {
                    let chunk = match state.take_until(|c| self.pattern.match_start(&c)) {
                        TakeUntil::EndOfString => {
                            self.maybe_state = None;
                            &self.input[start..]
                        }
                        TakeUntil::SuccessAt(end) => {
                            self.maybe_state = Some(end);
                            &self.input[start..end.char_start]
                        }
                    };
                    Some(SplitPart::NotMatch(chunk))
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

enum TakeUntil<'a> {
    SuccessAt(State<'a>),
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

    fn take_until<P>(&self, predicate: P) -> TakeUntil<'a>
    where
        P: Fn(char) -> bool,
    {
        if predicate(self.at_char) {
            TakeUntil::SuccessAt(*self)
        } else {
            let mut state = *self;
            loop {
                match state.try_next() {
                    None => break TakeUntil::EndOfString,
                    Some(next) => {
                        if predicate(next.at_char) {
                            break TakeUntil::SuccessAt(next);
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

    fn is_alpha(c: &char) -> bool {
        c.is_alphabetic()
    }

    #[test]
    fn base_cases() {
        assert_eq!(
            Iter::new("", is_alpha)
                .collect::<Vec<SplitPart>>()
                .to_owned(),
            vec![]
        );
        assert_eq!(
            Iter::new(" ", is_alpha)
                .collect::<Vec<SplitPart>>()
                .to_owned(),
            vec![SplitPart::NotMatch(" ")]
        );
        assert_eq!(
            Iter::new("a", is_alpha)
                .collect::<Vec<SplitPart>>()
                .to_owned(),
            vec![SplitPart::Match("a")]
        );
    }

    #[test]
    fn single_word() {
        assert_eq!(
            Iter::new("input", is_alpha)
                .collect::<Vec<SplitPart>>()
                .to_owned(),
            vec![SplitPart::Match("input")]
        );
    }

    #[test]
    fn single_delimiter() {
        assert_eq!(
            Iter::new(" , ", is_alpha)
                .collect::<Vec<SplitPart>>()
                .to_owned(),
            vec![SplitPart::NotMatch(" , ")]
        );
    }

    #[test]
    fn word_surrounded_by_delimiters() {
        assert_eq!(
            Iter::new("{input}", is_alpha)
                .collect::<Vec<SplitPart>>()
                .to_owned(),
            vec![
                SplitPart::NotMatch("{"),
                SplitPart::Match("input"),
                SplitPart::NotMatch("}"),
            ]
        );
    }

    #[test]
    fn delimiter_surrounded_by_words() {
        assert_eq!(
            Iter::new("this that", is_alpha)
                .collect::<Vec<SplitPart>>()
                .to_owned(),
            vec![
                SplitPart::Match("this"),
                SplitPart::NotMatch(" "),
                SplitPart::Match("that"),
            ]
        );
    }

    #[test]
    fn sentence() {
        assert_eq!(
            Iter::new("sphinx of black quartz, judge my vow!", is_alpha)
                .collect::<Vec<SplitPart>>()
                .to_owned(),
            vec![
                SplitPart::Match("sphinx"),
                SplitPart::NotMatch(" "),
                SplitPart::Match("of"),
                SplitPart::NotMatch(" "),
                SplitPart::Match("black"),
                SplitPart::NotMatch(" "),
                SplitPart::Match("quartz"),
                SplitPart::NotMatch(", "),
                SplitPart::Match("judge"),
                SplitPart::NotMatch(" "),
                SplitPart::Match("my"),
                SplitPart::NotMatch(" "),
                SplitPart::Match("vow"),
                SplitPart::NotMatch("!"),
            ]
        );
    }

    #[test]
    fn degenerate_bounds() {
        assert_eq!(
            Iter::new(
                "abc",
                ComplexPattern {
                    match_start: |_| true,
                    match_end: |_| true,
                }
            )
            .collect::<Vec<SplitPart>>()
            .to_owned(),
            vec![
                SplitPart::Match("a"),
                SplitPart::Match("b"),
                SplitPart::Match("c")
            ]
        );
    }
}
