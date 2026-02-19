use std::fmt::{Display, Write};

#[derive(Debug)]
pub struct Wolfram {
    lookup: Vec<bool>,
}
impl Wolfram {
    /**
     * Uses the standard coding system described here: https://en.wikipedia.org/wiki/Elementary_cellular_automaton#The_numbering_system
     */
    pub fn from_code(code: u8) -> Wolfram {
        let mut mask = 1u8;
        let lookup: Vec<bool> = (0..8)
            .map(|_| {
                let is_set = code & mask;
                mask <<= 1;
                is_set != 0
            })
            .collect();

        Wolfram { lookup }
    }

    pub fn merge(&self, left: &bool, middle: &bool, right: &bool) -> bool {
        // Couldn't think of a better way to conditionally mask off the bits
        // to create the index I wanted
        let l_mask: usize = if *left { 0x4 } else { 0x0 };
        let m_mask: usize = if *middle { 0x2 } else { 0x0 };
        let r_mask: usize = if *right { 0x1 } else { 0x0 };
        *self.lookup.get(l_mask | m_mask | r_mask).unwrap_or(&false)
    }
}

pub struct State {
    cells: Vec<bool>,
}

impl State {
    pub fn new(cells: &[bool]) -> State {
        if cells.len() < 3 {
            let mut cells = cells.to_vec();
            let mut padding = vec![false, false, false];
            cells.append(&mut padding);
            let cells: Vec<bool> = cells.into_iter().take(3).collect();
            State { cells }
        } else {
            State {
                cells: cells.to_vec(),
            }
        }
    }

    /**
     * Whitespace is 'off' and anything else is 'off'
     */
    pub fn from_string(raw: &str) -> State {
        let cells: Vec<bool> = raw.chars().map(|c| !c.is_whitespace()).collect();
        State::new(&cells)
    }

    /**
     * Calculates the next state
     */
    pub fn next(&self, rule: &Wolfram) -> State {
        // We're using cycle to basically prepend the last cell to the iterator
        // There's probably a better way to do this.
        let mut prev_itr = self.cells.iter().cycle();
        prev_itr.nth(self.cells.len() - 2);

        // Cycling this as well to append the first cell to the end
        // Again, probably a better way to do this.
        let mut next_itr = self.cells.iter().cycle();
        next_itr.next();

        let curr_itr = self.cells.iter();

        let cells = prev_itr
            .zip(curr_itr)
            .zip(next_itr)
            .map(|((p, c), n)| rule.merge(p, c, n))
            .collect();

        State { cells }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.cells {
            f.write_char(if *c { 'X' } else { ' ' })?;
        }
        Ok(())
    }
}
