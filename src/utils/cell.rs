use ratatui::style::{ palette::tailwind::{ BLUE, GRAY, GREEN, ORANGE, RED, WHITE, YELLOW }, Color };
use pleco::{ BitMove, Player, SQ };

use super::{ dest_in_moves, get_file, get_rank, Coord };

/// Cell.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    /// Color of the cell.
    pub color: Color,
    /// Square associated with the cell.
    pub square: SQ,
}

impl Cell {
    /// Creates a new `Cell` instance and initilizes the color.
    ///
    /// # Arguments
    ///
    /// * `moves_from_selected_coord` - The moves from the selected coordinate.
    /// * `selected_coord` - The selected coordinate.
    /// * `cursor_coord` - The cursor coordinate.
    /// * `main_player` - The current player.
    /// * `row` - The row of the cell.
    /// * `col` - The column of the cell.
    ///
    /// # Returns
    ///
    /// A new `Cell` instance.
    pub fn new(
        moves_from_selected_coord: &Vec<BitMove>,
        selected_coord: Coord,
        cursor_coord: Coord,
        main_player: Player,
        row: usize,
        col: usize
    ) -> Self {
        let square = SQ::make(get_file(col as u8), get_rank(row as u8, main_player));
        let mut color = if (row + col) % 2 == 0 { WHITE } else { GRAY.c700 };

        if selected_coord.active {
            if selected_coord.is_on(row as i8, col as i8) {
                color = if cursor_coord.is_on(row as i8, col as i8) { YELLOW.c500 } else { GREEN.c500 };
            } else if dest_in_moves(square, moves_from_selected_coord) {
                color = if cursor_coord.is_on(row as i8, col as i8) {
                    ORANGE.c500
                } else {
                    if (row + col) % 2 == 0 { BLUE.c400 } else { BLUE.c500 }
                };
            } else if cursor_coord.is_on(row as i8, col as i8) {
                color = RED.c500;
            }
        } else if cursor_coord.is_on(row as i8, col as i8) && cursor_coord.active {
            color = RED.c500;
        }

        Cell { color, square }
    }
}
