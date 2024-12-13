use ratatui::crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use color_eyre::Result;
use pleco::PieceType;

use crate::{ utils::CoordEvent, App };

pub fn handle_key_event(key_event: KeyEvent, app: &mut App) -> Result<()> {
    match (key_event.modifiers, key_event.code) {
        // Universal commands
        (_, KeyCode::Char('r')) => app.reset(),
        | (_, KeyCode::Esc | KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
        // Block all non-universal key events while block_inputs is true
        _ if app.block_inputs => {}
        // Movement of the cursor
        (_, KeyCode::Up | KeyCode::Char('w')) => app.cursor_coord.handle_event(CoordEvent::UP),
        (_, KeyCode::Down | KeyCode::Char('s')) => app.cursor_coord.handle_event(CoordEvent::DOWN),
        (_, KeyCode::Left | KeyCode::Char('a')) => app.cursor_coord.handle_event(CoordEvent::LEFT),
        (_, KeyCode::Right | KeyCode::Char('d')) => app.cursor_coord.handle_event(CoordEvent::RIGHT),
        // Selection of the cursor and moves
        (_, KeyCode::Enter | KeyCode::Char(' ')) => app.update_selected_coord(),
        // Promotion piece
        (_, KeyCode::Char('1')) => app.set_promotion_piece(PieceType::Q),
        (_, KeyCode::Char('2')) => app.set_promotion_piece(PieceType::R),
        (_, KeyCode::Char('3')) => app.set_promotion_piece(PieceType::B),
        (_, KeyCode::Char('4')) => app.set_promotion_piece(PieceType::N),
        _ => {}
    }
    Ok(())
}

pub fn handle_resize_event(app: &mut App, width: u16, height: u16) -> Result<()> {
    app.terminal_too_small = width < 106 || height < 24;
    Ok(())
}
