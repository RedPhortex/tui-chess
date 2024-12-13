use std::fmt::{ Display, Formatter, Result };
use pleco::{ Player, SQ };

use super::{ get_file, get_rank };

/// Coordinate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct Coord {
    /// Row of the coordinate.
    pub row: i8,
    /// Column of the coordinate.
    pub col: i8,
    // Whether the Coord is active.
    pub active: bool,
}

/// Coordinate event.
/// Used to handle the movement of the coordinate.
#[derive(Clone, Copy, Debug)]
pub enum CoordEvent {
    /// Move the cursor up.
    UP,
    /// Move the cursor down.
    DOWN,
    /// Move the cursor left.
    LEFT,
    /// Move the cursor right.
    RIGHT,
}

impl Coord {
    /// Creates a new `Coord` instance with the specified row and column.
    ///
    /// # Arguments
    ///
    /// * `row` - The row value for the `Coord`.
    /// * `col` - The column value for the `Coord`.
    ///
    /// # Returns
    ///
    /// A new `Coord` instance with the specified row and column.
    pub fn new(col: i8, row: i8, active: bool) -> Self {
        Coord { row, col, active }
    }

    /// Handles the event of the coordinate.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to handle.
    pub fn handle_event(&mut self, event: CoordEvent) {
        match event {
            CoordEvent::UP => self.move_by(-1, 0, 7, 0),
            CoordEvent::DOWN => self.move_by(1, 0, 7, 0),
            CoordEvent::LEFT => self.move_by(0, -1, 7, 0),
            CoordEvent::RIGHT => self.move_by(0, 1, 7, 0),
        }
    }

    /// Moves the `Coord` to a specified row and columns.
    ///
    /// # Arguments
    ///
    /// * `row` - The row to move the `Coord` to.
    /// * `col` - The column to move the `Coord` to.
    pub fn move_to(&mut self, row: i8, col: i8) {
        self.row = row;
        self.col = col;
    }

    /// Moves the `Coord` by a specified amount of rows and columns.
    ///
    /// # Arguments
    ///
    /// * `row` - The amount of rows to move the `Coord` by.
    /// * `col` - The amount of columns to move the `Coord` by.
    /// * `max` - The maximum value for the `Coord` to move to.
    /// * `min` - The minimum value for the `Coord` to move to.
    pub fn move_by(&mut self, row: i8, col: i8, max: i8, min: i8) {
        self.row = self.row.saturating_add(row).min(max).max(min);
        self.col = self.col.saturating_add(col).min(max).max(min);
    }

    /// Toggles the active state of the `Coord`.
    pub fn toggle_active(&mut self) {
        self.active = !self.active;
    }

    /// Creates a `pleco::SQ` from the `Coord`.
    ///
    /// # Arguments
    ///
    /// * `main_player` - The current player.
    ///
    /// # Returns
    ///
    /// A `pleco::SQ` representing the `Coord`.
    pub fn to_square(&self, main_player: Player) -> pleco::SQ {
        SQ::make(get_file(self.col as u8), get_rank(self.row as u8, main_player))
    }

    /// Checks whether the Coord is on the specified row and column.
    ///
    /// # Arguments
    ///
    /// * `row` - The row to check.
    /// * `col` - The column to check.
    ///
    /// # Returns
    ///
    /// `true` if `self` is on the specified row and column, otherwise `false`.
    pub fn is_on(&self, row: i8, col: i8) -> bool {
        self.row == row && self.col == col
    }

    /// Checks whether the Coord is valid as a chess board coordinate.
    ///
    /// # Returns
    ///
    /// `true` if `self` is valid, otherwise `false`.
    pub fn is_valid(&self) -> bool {
        (0..8).contains(&self.col) && (0..8).contains(&self.row)
    }
}

impl Default for Coord {
    /// Creates a new `Coord` instance.
    ///
    /// # Returns
    ///
    /// A new `Coord` instance.
    fn default() -> Self {
        Self { row: 7, col: 0, active: true }
    }
}

impl Display for Coord {
    /// Formats the `Coord` instance as a string.
    ///
    /// # Returns
    ///
    /// A string representation of the `Coord` instance.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.col, self.row)
    }
}

#[cfg(test)]
mod tests {
    use pleco::{ File, Rank };

    use super::*;

    #[test]
    fn test_new() {
        let coord = Coord::new(4, 3, true);
        assert_eq!(coord.row, 3);
        assert_eq!(coord.col, 4);
        assert_eq!(coord.active, true);
    }

    #[test]
    fn test_handle_event() {
        let mut coord = Coord::new(4, 3, true);
        coord.handle_event(CoordEvent::UP);
        assert_eq!(coord.row, 2);
        assert_eq!(coord.col, 4);

        coord.handle_event(CoordEvent::DOWN);
        assert_eq!(coord.row, 3);
        assert_eq!(coord.col, 4);

        coord.handle_event(CoordEvent::LEFT);
        assert_eq!(coord.row, 3);
        assert_eq!(coord.col, 3);

        coord.handle_event(CoordEvent::RIGHT);
        assert_eq!(coord.row, 3);
        assert_eq!(coord.col, 4);
    }

    #[test]
    fn test_move_to() {
        let mut coord = Coord::new(6, 5, false);
        coord.move_to(3, 4);
        assert_eq!(coord.row, 3);
        assert_eq!(coord.col, 4);
    }

    #[test]
    fn test_move_by() {
        let mut coord = Coord::new(6, 5, false);
        assert_eq!(coord.active, false);
        coord.move_by(3, 4, i8::MAX, 0);
        assert_eq!(coord.row, 8);
        assert_eq!(coord.col, 10);
    }

    #[test]
    fn test_toggle_active() {
        let mut coord = Coord::new(6, 5, true);
        coord.toggle_active();
        assert_eq!(coord.active, false);
        coord.toggle_active();
        assert_eq!(coord.active, true);
    }

    #[test]
    fn test_to_square() {
        let coord = Coord::new(6, 5, true);
        assert_eq!(coord.to_square(Player::White), SQ::make(File::G, Rank::R3));
        assert_eq!(coord.to_square(Player::Black), SQ::make(File::G, Rank::R6));
    }

    #[test]
    fn test_is_on() {
        let coord = Coord::new(6, 5, true);
        assert!(coord.is_on(5, 6));
        assert!(!coord.is_on(5, 7));
        assert!(!coord.is_on(6, 6));
    }

    #[test]
    fn test_is_valid() {
        assert!(Coord::new(0, 0, true).is_valid());
        assert!(Coord::new(7, 7, false).is_valid());
        assert!(!Coord::new(8, 0, true).is_valid());
        assert!(!Coord::new(0, 8, false).is_valid());
    }

    #[test]
    fn test_display() {
        let coord = Coord::new(3, 2, true);
        assert_eq!(format!("{}", coord), "3 2");
    }
}
