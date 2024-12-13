use ratatui::{
    layout::Direction,
    prelude::{ Buffer, Rect },
    style::Stylize,
    text::Line,
    widgets::{ Block, Widget },
};

use crate::{ utils::{ create_board_layout, piece_to_char, square_to_string, Cell }, App };

/// Board widget.
#[derive(Debug)]
pub struct Board<'a> {
    // pub selected_coord: Coord,
    // pub cursor_coord: Coord,
    // pub main_player: Player,
    // pub board: pleco::Board,

    // we need the whole app struct, in order to access the app.log() method
    // TODO: find a better way to do this

    /// App struct.
    pub app: &'a mut App,
}

impl Widget for Board<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let piece_locations = self.app.board.get_piece_locations();

        // Create the layout for the rows while centering the board using the border size
        let rows = create_board_layout(Direction::Vertical, area.height, 8, (area.height % 8) / 2, area);

        // Loop over each row to create the individual cells while skipping the top and bottom borders
        for (row, area) in rows.iter().skip(1).take(8).enumerate() {
            // Create the layout for the current cell while centering the board using the border size
            let columns = create_board_layout(
                Direction::Horizontal,
                area.width,
                8,
                (area.width % 8) / 2,
                *area
            );

            // Loop over each cell in a column while skipping the left and right borders
            for (col, square) in columns.iter().skip(1).take(8).enumerate() {
                let cell = Cell::new(
                    &self.app.moves_from_selected_coord,
                    self.app.selected_coord,
                    self.app.cursor_coord,
                    self.app.main_player,
                    row,
                    col
                );

                Block::default()
                    .bg(cell.color)
                    .title_top(
                        Line::from(piece_to_char(piece_locations.piece_at(cell.square))) // TODO: find a better way to display the piece
                            .centered()
                            .black()
                    )
                    .title_bottom(Line::from(square_to_string(cell.square)))
                    .render(*square, buf);
            }
        }
    }
}
