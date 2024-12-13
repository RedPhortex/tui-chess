use pleco::PieceType;
use ratatui::{
    buffer::Buffer,
    layout::{ Constraint, Direction, Layout, Rect },
    style::Stylize,
    symbols::border,
    text::{ Line, Text },
    widgets::{ Block, Borders, Paragraph, Widget },
};

use crate::{ utils::{ get_current_player, is_game_over }, App };

/// Info widget.
#[derive(Debug)]
pub struct Info<'a> {
    // its easier to pass the whole app struct instead of the individual components
    // TODO: find a better way to do this

    /// App struct.
    pub app: &'a mut App,
}

impl Widget for Info<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let piece_locations = self.app.board.get_piece_locations();
        let square = self.app.cursor_coord.to_square(self.app.main_player);
        let blank = Line::from("");

        let moves = format!("Move: {}", self.app.board.moves_played() + 1);
        let player = format!("{}", get_current_player(self.app.board.moves_played() + 1));
        let top_line = format!("{} | {}", moves, player).bold().into_centered_line();

        let current_square = format!("Current square: {}", square.to_string().to_uppercase())
            .bold()
            .into_left_aligned_line();

        let current_piece = format!("Current piece: {}", match piece_locations.piece_at(square).type_of() {
            PieceType::None => String::from("None"), // Create a String, because piece_type.to_string() returns a String
            piece_type @ _ => piece_type.to_string(),
        })
            .bold()
            .into_left_aligned_line();

        let info_text = Text::from(Vec::from([top_line, blank.clone(), current_square, current_piece]));
        let checkmate_text = Text::from(
            Vec::from([
                (if self.app.board.checkmate() { "Checkmate!" } else { "Stalemate!" })
                    .bold()
                    .into_centered_line(),
                blank,
                "Press r to reset.".bold().into_centered_line(),
            ])
        );

        let promotion = Text::from(
            Vec::from([
                "Promotion piece: ".bold().into_left_aligned_line(),
                (
                    if self.app.promotion_piece == PieceType::Q {
                        format!("  1: Queen").white()
                    } else {
                        format!("  1: Queen").gray()
                    }
                )
                    .bold()
                    .into_left_aligned_line(),
                (
                    if self.app.promotion_piece == PieceType::R {
                        format!("  2: Rook").white()
                    } else {
                        format!("  2: Rook").gray()
                    }
                )
                    .bold()
                    .into_left_aligned_line(),
                (
                    if self.app.promotion_piece == PieceType::B {
                        format!("  3: Bishop").white()
                    } else {
                        format!("  3: Bishop").gray()
                    }
                )
                    .bold()
                    .into_left_aligned_line(),
                (
                    if self.app.promotion_piece == PieceType::N {
                        format!("  4: Knight").white()
                    } else {
                        format!("  4: Knight").gray()
                    }
                )
                    .bold()
                    .into_left_aligned_line(),
            ])
        );

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
            .margin(1)
            .split(area);

        Paragraph::new(if !is_game_over(&self.app.board) { info_text } else { checkmate_text }).render(
            layout[0],
            buf
        );

        Paragraph::new(promotion).render(layout[1], buf);

        Block::default()
            .title_top(Line::from("Info").centered().bold())
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .render(area, buf);
    }
}
