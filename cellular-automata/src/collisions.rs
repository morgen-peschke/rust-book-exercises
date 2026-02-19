use std::fmt::{Display, Write};

use rand::Rng;

pub struct Rule {
    pub bounce: bool,
    pub partial_destroy: bool,
}
impl Rule {
    /**
     * Resolve collisions
     */
    pub fn collide(&self, moving: &Moving, destination: &Cell) -> CollisionResult {
        match (moving, destination) {
            // Uncontested moves
            (m @ Moving::Left(_), Cell::Empty) => CollisionResult::MovingWon(m.clone()),
            (m @ Moving::Right(_), Cell::Empty) => CollisionResult::MovingWon(m.clone()),

            // Both moving in the same direction
            (m @ Moving::Left(_), Cell::InMotion(Moving::Left(d))) => {
                CollisionResult::DestinationEscapes {
                    bumped: *d,
                    moved: m.clone(),
                }
            }
            (m @ Moving::Right(_), Cell::InMotion(Moving::Right(d))) => {
                CollisionResult::DestinationEscapes {
                    bumped: *d,
                    moved: m.clone(),
                }
            }

            // Actual collisions
            // There's probably a way to clean up the duplication, but I can't
            // be bothered right now to figure out how to abstract the constructors
            // without creating more duplication than I'd be removing.
            (Moving::Left(m), Cell::InMotion(Moving::Right(d))) => {
                let new_weight = if self.partial_destroy {
                    (m - d).abs()
                } else {
                    *m.max(d)
                };
                if m == d {
                    CollisionResult::BothDestroyed
                } else if m > d {
                    CollisionResult::MovingWon(Moving::Left(new_weight))
                } else {
                    CollisionResult::DestinationWon(Cell::InMotion(Moving::Right(new_weight)))
                }
            }
            (Moving::Right(m), Cell::InMotion(Moving::Left(d))) => {
                let new_weight = if self.partial_destroy {
                    (m - d).abs()
                } else {
                    *m.max(d)
                };
                if m == d {
                    CollisionResult::BothDestroyed
                } else if m > d {
                    CollisionResult::MovingWon(Moving::Right(new_weight))
                } else {
                    CollisionResult::DestinationWon(Cell::InMotion(Moving::Left(new_weight)))
                }
            }
            (Moving::Left(m), Cell::Stationary(s)) => {
                let new_weight = if self.partial_destroy {
                    (m - s).abs()
                } else {
                    *m.max(s)
                };
                if m == s {
                    CollisionResult::BothDestroyed
                } else if m > s {
                    CollisionResult::MovingWon(Moving::Left(new_weight))
                } else {
                    CollisionResult::DestinationWon(Cell::Stationary(new_weight))
                }
            }
            (Moving::Right(m), Cell::Stationary(s)) => {
                let new_weight = if self.partial_destroy {
                    (m - s).abs()
                } else {
                    *m.max(s)
                };
                if m == s {
                    CollisionResult::BothDestroyed
                } else if m > s {
                    CollisionResult::MovingWon(Moving::Right(new_weight))
                } else {
                    CollisionResult::DestinationWon(Cell::Stationary(new_weight))
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum CollisionResult {
    BothDestroyed,
    MovingWon(Moving),
    DestinationWon(Cell),
    DestinationEscapes {
        // Only the weight of the bumped, because otherwise the
        // types get weird and we already know the direction from context
        bumped: i8,
        moved: Moving,
    },
}

// Split these out because otherwise the types were getting real messy
#[derive(Debug, Clone)]
pub enum Moving {
    Left(i8),
    Right(i8),
}

#[derive(Debug, Clone)]
pub enum Cell {
    Empty,
    Stationary(i8),
    InMotion(Moving),
}

#[derive(Debug, Clone)]
pub struct State {
    cells: Vec<Cell>,
}

struct ParserState {
    is_left: bool,
    is_right: bool,
    curr: Vec<char>,
}
impl ParserState {
    fn reset(&mut self) {
        self.is_left = false;
        self.is_right = false;
        self.curr = Vec::new();
    }
    fn parse(&self) -> Result<Option<Cell>, String> {
        if !self.curr.is_empty() {
            let raw_str: String = self.curr.iter().collect();
            let value = raw_str
                .parse::<i8>()
                .map_err(|e| format!("Invalid number ({raw_str}): {e}"))?;
            if value > 99 {
                Err("Value must be between 0 and 99".to_string())
            } else if self.is_left && self.is_right {
                Err("Only one prefix is allowed".to_string())
            } else if self.is_left {
                Ok(Some(Cell::InMotion(Moving::Left(value))))
            } else if self.is_right {
                Ok(Some(Cell::InMotion(Moving::Right(value))))
            } else {
                Ok(Some(Cell::Stationary(value)))
            }
        } else {
            Ok(None)
        }
    }
}

impl State {
    /**
     * Generates a random initial state using a weighted-probability algorithm
     */
    pub fn random(
        length: usize,
        weight_stationary: u16,
        weight_left: u16,
        weight_right: u16,
        weight_empty: u16,
    ) -> State {
        let mut r = rand::thread_rng();

        // Weighted random alg stolen from https://stackoverflow.com/a/8435261
        let total_weight = weight_stationary + weight_left + weight_right + weight_empty;
        let spec_stationary = weight_stationary;
        let spec_left = spec_stationary + weight_left;
        let spec_right = spec_left + weight_right;

        let cells: Vec<Cell> = (0..length)
            .map(|_| {
                let choice = r.gen_range(0..=total_weight);
                if choice < spec_stationary {
                    Cell::Stationary(r.gen_range(1..100))
                } else if choice < spec_left {
                    Cell::InMotion(Moving::Left(r.gen_range(1..100)))
                } else if choice < spec_right {
                    Cell::InMotion(Moving::Right(r.gen_range(1..100)))
                } else {
                    Cell::Empty
                }
            })
            .collect();
        State { cells }
    }

    /**
     * Very dumb parsing, nothing to see here
     */
    pub fn from_string(raw: &str) -> Result<State, String> {
        let mut state = ParserState {
            is_left: false,
            is_right: false,
            curr: Vec::new(),
        };
        let mut cells: Vec<Cell> = Vec::new();

        for c in raw.trim().chars() {
            match c {
                ' ' => {
                    if let Some(cell) = state.parse()? {
                        cells.push(cell);
                    }
                    state.reset();
                }
                '-' => {
                    if state.curr.is_empty() {
                        state.is_left = true;
                    } else {
                        return Err("'-' must be at the start of a cell".to_string());
                    }
                }
                '+' => {
                    if state.curr.is_empty() {
                        state.is_right = true;
                    } else {
                        return Err("'+' must be at the start of a cell".to_string());
                    }
                }
                _ if c.is_ascii_digit() => {
                    state.curr.push(c);
                }
                '_' => {
                    if !state.curr.is_empty() {
                        return Err("'_' must be separated from other cells by a space".to_string());
                    }
                    cells.push(Cell::Empty);
                }
                _ => return Err(format!("Unexpected character: '{c}'")),
            };
        }
        if let Some(cell) = state.parse()? {
            cells.push(cell);
        }
        Ok(State { cells })
    }

    /**
     * Probably a way to unify these into a single pass, but the fields are
     * small enough it wasn't worth the trouble
     */
    fn all_empty_or_left(&self) -> bool {
        self.cells
            .iter()
            .all(|c| matches!(c, Cell::Empty | Cell::InMotion(Moving::Left(_))))
    }

    fn all_empty_or_right(&self) -> bool {
        self.cells
            .iter()
            .all(|c| matches!(c, Cell::Empty | Cell::InMotion(Moving::Right(_))))
    }

    fn all_empty_or_stationary(&self) -> bool {
        self.cells
            .iter()
            .all(|c| matches!(c, Cell::Empty | Cell::Stationary(_)))
    }

    /**
     * Move an object leftwards
     *
     * Depending on how collisions resolve, it can recursively shuffle objects
     * further leftward. `was_bumped` is important for this, so we don't accidentally
     * overwrite the value that bumped this one.
     */
    fn move_left(
        cells: &mut Vec<Cell>,
        index: usize,
        weight: i8,
        was_bumped: bool,
        rule: &Rule,
    ) -> usize {
        // The index check is because `usize` is unsigned and underflow sucks
        match if index == 0 {
            None
        } else {
            // Have to clone here or the borrow checker gets annoyed when we
            // mutate things later - but oddly only after pulling the code
            // that sets the cell to empty was pulled out to before the match.
            // If it's inlined with each setting of the previous cell, the
            // borrow checker is fine with it.
            cells.get(index - 1).cloned()
        } {
            None => {
                // Hit a side wall, maybe bounce?
                if rule.bounce {
                    cells[index] = Cell::InMotion(Moving::Right(weight));
                } else {
                    cells[index] = Cell::Stationary(weight);
                }
            }
            Some(destination) => {
                if !was_bumped {
                    cells[index] = Cell::Empty;
                }
                match rule.collide(&Moving::Left(weight), &destination) {
                    CollisionResult::BothDestroyed => {
                        cells[index - 1] = Cell::Empty;
                    }
                    CollisionResult::MovingWon(cell) => {
                        cells[index - 1] = Cell::InMotion(cell);
                    }
                    CollisionResult::DestinationWon(cell) => {
                        cells[index - 1] = cell;
                    }
                    CollisionResult::DestinationEscapes { moved, bumped } => {
                        cells[index - 1] = Cell::InMotion(moved);
                        // Need to propagate the motion backwards
                        State::move_left(cells, index - 1, bumped, true, rule);
                    }
                }
            }
        }
        // This one's simple, since we're moving the object into an already
        // processed part of the cell array, we can just advance one step.
        // Things will not be so easy in `move_right`
        index + 1
    }

    /**
     * Move an object rightwards
     *
     * Depending on how collisions resolve, it can recursively shuffle objects
     * further rightward. `was_bumped` is important for this, so we don't accidentally
     * overwrite the value that bumped this one.
     */
    fn move_right(
        cells: &mut Vec<Cell>,
        index: usize,
        weight: i8,
        was_bumped: bool,
        rule: &Rule,
    ) -> usize {
        // Cloning shenanigans again, see `move_left`
        match cells.get(index + 1).cloned() {
            None => {
                // Hit a side wall, maybe bounce?
                if rule.bounce {
                    cells[index] = Cell::InMotion(Moving::Left(weight));
                } else {
                    cells[index] = Cell::Stationary(weight);
                }
                // Since this only affected the current cell, we only
                // advance once (though this is kind of irrelevant since we're
                // advancing past the end of the vector)
                index + 1
            }
            Some(destination) => {
                if !was_bumped {
                    cells[index] = Cell::Empty;
                }
                match rule.collide(&Moving::Right(weight), &destination) {
                    CollisionResult::BothDestroyed => {
                        cells[index + 1] = Cell::Empty;
                        // Since we destroyed the next cell, we can skip it
                        // as well
                        index + 2
                    }
                    CollisionResult::MovingWon(cell) => {
                        cells[index + 1] = Cell::InMotion(cell);
                        // We moved into the next cell, so it's already been
                        // processed and we can skip it as well
                        index + 2
                    }
                    CollisionResult::DestinationWon(cell) => {
                        cells[index + 1] = cell;
                        // We updated the next cell, but it hasn't been processed
                        // so we can't skip it
                        index + 1
                    }
                    CollisionResult::DestinationEscapes { moved, bumped } => {
                        cells[index + 1] = Cell::InMotion(moved);
                        // Need to propagate the bumped object forwards, so we're
                        // delegating the amount to change index by to the recursive calls.
                        // This wasn't needed in `move_left`, but we're moving into unprocessed
                        // territory and don't want to process objects twice.
                        State::move_right(cells, index + 1, bumped, true, rule)
                    }
                }
            }
        }
    }

    pub fn next(&self, rule: &Rule) -> Option<State> {
        // Stop once collisions are impossible
        if self.all_empty_or_left() || self.all_empty_or_right() || self.all_empty_or_stationary() {
            return None;
        };
        let mut index = 0;
        let mut cells = self.cells.clone();
        while let Some(cell) = cells.get(index).cloned() {
            match cell {
                Cell::Empty => index += 1,
                Cell::Stationary(_) => index += 1,
                Cell::InMotion(Moving::Left(weight)) => {
                    index = State::move_left(&mut cells, index, weight, false, rule);
                }
                Cell::InMotion(Moving::Right(weight)) => {
                    index = State::move_right(&mut cells, index, weight, false, rule);
                }
            }
        }
        Some(State { cells })
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('|')?;
        for c in &self.cells {
            match c {
                Cell::Empty => f.write_char('_'),
                Cell::Stationary(_) => f.write_char('X'),
                Cell::InMotion(Moving::Left(_)) => f.write_char('<'),
                Cell::InMotion(Moving::Right(_)) => f.write_char('>'),
            }?;
        }
        f.write_char('|')?;
        Ok(())
    }
}

/**
 * A wrapper to debug-print the State
 *
 * Make sure that this outputs something that State::from_string can understand
 */
pub struct DebugOutput<'a>(pub &'a State);
impl<'a> Display for DebugOutput<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.0.cells {
            match c {
                Cell::Empty => write!(f, "_ "),
                Cell::Stationary(w) => write!(f, "{w} "),
                Cell::InMotion(Moving::Left(w)) => write!(f, "-{w} "),
                Cell::InMotion(Moving::Right(w)) => write!(f, "+{w} "),
            }?;
        }
        Ok(())
    }
}
