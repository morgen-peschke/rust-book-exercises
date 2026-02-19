use std::fmt::{Display, Write};

use rand::Rng;

pub struct Rule {
    pub bounce: bool,
    pub partial_destroy: bool,
}
impl Rule {
    pub fn collide(&self, moving: &Moving, destination: &Cell) -> CollisionResult {
        match (moving, destination) {
            (m @ Moving::Left(_), Cell::Empty) => CollisionResult::MovingWon(m.clone()),
            (m @ Moving::Right(_), Cell::Empty) => CollisionResult::MovingWon(m.clone()),
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

    fn move_left(
        cells: &mut Vec<Cell>,
        index: usize,
        weight: i8,
        was_bumped: bool,
        rule: &Rule,
    ) -> usize {
        match if index == 0 {
            None
        } else {
            cells.get(index - 1)
        } {
            None => {
                // Hit a side wall, bounce
                if rule.bounce {
                    cells[index] = Cell::InMotion(Moving::Right(weight));
                } else {
                    cells[index] = Cell::Stationary(weight);
                }
            }
            Some(destination) => {
                match rule.collide(&Moving::Left(weight), destination) {
                    CollisionResult::BothDestroyed => {
                        if !was_bumped {
                            cells[index] = Cell::Empty;
                        }
                        cells[index - 1] = Cell::Empty;
                    }
                    CollisionResult::MovingWon(cell) => {
                        if !was_bumped {
                            cells[index] = Cell::Empty;
                        }
                        cells[index - 1] = Cell::InMotion(cell);
                    }
                    CollisionResult::DestinationWon(cell) => {
                        if !was_bumped {
                            cells[index] = Cell::Empty;
                        }
                        cells[index - 1] = cell;
                    }
                    CollisionResult::DestinationEscapes { moved, bumped } => {
                        if !was_bumped {
                            cells[index] = Cell::Empty;
                        }
                        cells[index - 1] = Cell::InMotion(moved);
                        // Need to propagate the motion backwards
                        State::move_left(cells, index - 1, bumped, true, rule);
                    }
                }
            }
        }
        index + 1
    }

    fn move_right(
        cells: &mut Vec<Cell>,
        index: usize,
        weight: i8,
        was_bumped: bool,
        rule: &Rule,
    ) -> usize {
        match cells.get(index + 1) {
            None => {
                // Hit a side wall, bounce
                if rule.bounce {
                    cells[index] = Cell::InMotion(Moving::Left(weight));
                } else {
                    cells[index] = Cell::Stationary(weight);
                }
                index + 1
            }
            Some(destination) => {
                match rule.collide(&Moving::Right(weight), destination) {
                    CollisionResult::BothDestroyed => {
                        if !was_bumped {
                            cells[index] = Cell::Empty;
                        }
                        cells[index + 1] = Cell::Empty;
                        index + 2
                    }
                    CollisionResult::MovingWon(cell) => {
                        //println!("{cell:?} @ {index}");
                        if !was_bumped {
                            cells[index] = Cell::Empty;
                        }
                        cells[index + 1] = Cell::InMotion(cell);
                        index + 2
                    }
                    CollisionResult::DestinationWon(cell) => {
                        if !was_bumped {
                            cells[index] = Cell::Empty;
                        }
                        cells[index + 1] = cell;
                        index + 1
                    }
                    CollisionResult::DestinationEscapes { moved, bumped } => {
                        if !was_bumped {
                            cells[index] = Cell::Empty;
                        }
                        cells[index + 1] = Cell::InMotion(moved);
                        // Need to propagate the motion forwards
                        State::move_right(cells, index + 1, bumped, true, rule)
                    }
                }
            }
        }
    }

    pub fn next(&self, rule: &Rule) -> Option<State> {
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
