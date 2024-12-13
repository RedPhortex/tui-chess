use ratatui::{ layout::{ Constraint, Direction, Layout }, Frame };

use crate::{ widgets::{ Board, Info, Log, TerminalTooSmall }, App };

/// Renders the user interface
pub fn render(app: &mut App, frame: &mut Frame) {
    if app.terminal_too_small {
        frame.render_widget(TerminalTooSmall::default(), frame.area());
        return;
    }

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(4, 17), Constraint::Ratio(9, 17), Constraint::Ratio(4, 17)].as_ref())
        .split(frame.area());

    frame.render_widget(Log { log: app.log.clone() }, layout[0]);
    frame.render_widget(Board { app }, layout[1]);
    frame.render_widget(Info { app }, layout[2]);
}
