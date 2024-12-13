use pleco::{ core::piece_move::{ MoveFlag, PreMoveInfo }, BitMove, PieceType, Player };
use ratatui::{ crossterm::event::KeyEventKind, DefaultTerminal };
use color_eyre::{ eyre::WrapErr, Result };

use crate::{
    event::{ Event, EventHandler },
    handler::{ handle_key_event, handle_resize_event },
    tui::Tui,
    utils::{ dest_in_moves, get_current_player, is_game_over, move_to_square, moves_from_square, Coord },
};

/// Application.
#[derive(Debug)]
pub struct App {
    /// All possible moves from the selected coordinate.
    pub moves_from_selected_coord: Vec<BitMove>,
    /// The piece type to promote to.
    pub promotion_piece: PieceType,
    /// Indicates if the terminal is too small to display the application.
    pub terminal_too_small: bool,
    /// The selected coordinate.
    pub selected_coord: Coord,
    /// The cursor coordinate.
    pub cursor_coord: Coord,
    /// The player out of which perspective the board is viewed
    pub main_player: Player,
    /// The board.
    pub board: pleco::Board,
    /// Whether to block all non-universal key events.
    pub block_inputs: bool,
    /// The log of events.
    pub log: Vec<String>,
    /// Whether the application is running.
    running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            selected_coord: Coord::new(0, 0, false),
            moves_from_selected_coord: Default::default(),
            terminal_too_small: Default::default(),
            cursor_coord: Default::default(),
            promotion_piece: PieceType::Q,
            main_player: Player::White,
            board: Default::default(),
            log: Default::default(),
            block_inputs: false,
            running: true,
        }
    }

    /// runs the application's main loop until the user quits
    pub async fn run(&mut self, terminal: DefaultTerminal) -> Result<()> {
        let events = EventHandler::new(250);
        let mut tui = Tui::new(terminal, events);

        while self.running {
            tui.draw(self)?;
            self.handle_events(tui.events.next().await).wrap_err("Error handling events")?;
        }
        Ok(())
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self, event: Result<Event>) -> Result<()> {
        match event? {
            Event::Tick => self.tick(),
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press =>
                handle_key_event(key_event, self).wrap_err_with(||
                    format!("Handling key event failed:\n{key_event:#?}")
                ),
            Event::Resize(width, height) => handle_resize_event(self, width, height),
            Event::Mouse(_) => { Ok(()) }
            _ => Ok(()),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) -> Result<()> {
        Ok(())
    }

    /// Appends a message to the log.
    pub fn log(&mut self, message: &str) {
        self.log.push(message.to_string());
    }

    /// Resets the application.
    pub fn reset(&mut self) {
        self.cursor_coord.move_to(7, 0);
        self.selected_coord.active = true;
        self.selected_coord.move_to(7, 0);
        self.selected_coord.active = false;
        self.board = Default::default();
        self.log.clear();

        self.block_inputs = false;

        self.log(&format!("Reseted"));
    }

    /// Handles the move of a player.
    pub fn handle_move(&mut self) {
        let mut player_move = move_to_square(
            self.cursor_coord.to_square(self.main_player),
            &self.moves_from_selected_coord
        );

        // Set the correct promotion piece
        if player_move.is_promo() {
            player_move = BitMove::init(PreMoveInfo {
                src: player_move.get_src(),
                dst: player_move.get_dest(),
                flags: MoveFlag::Promotion {
                    capture: player_move.is_capture(),
                    prom: self.promotion_piece,
                },
            });
        }

        self.log(
            &format!("Player Move: {} ({})", player_move, get_current_player(self.board.moves_played() + 1))
        );

        self.board.apply_move(player_move);
        self.selected_coord.toggle_active();

        if is_game_over(&self.board) {
            self.selected_coord.active = false;
            self.cursor_coord.active = false;

            self.block_inputs = true;
        }
    }

    // Functions used for keyevents

    /// Set running to false in order to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Update the selected coordinate and handle moves.
    pub fn update_selected_coord(&mut self) {
        if self.selected_coord.active {
            if dest_in_moves(self.cursor_coord.to_square(self.main_player), &self.moves_from_selected_coord) {
                self.handle_move();
            } else if self.selected_coord.is_on(self.cursor_coord.row, self.cursor_coord.col) {
                self.selected_coord.toggle_active();
            } else {
                self.selected_coord.move_to(self.cursor_coord.row, self.cursor_coord.col);
            }
        } else {
            self.selected_coord.move_to(self.cursor_coord.row, self.cursor_coord.col);
            self.selected_coord.toggle_active();
        }

        // update moves from selected coord
        self.moves_from_selected_coord = moves_from_square(
            self.selected_coord.to_square(self.main_player),
            &self.board
        );
    }

    /// Update the promotion piece.
    pub fn set_promotion_piece(&mut self, piece_type: PieceType) {
        self.promotion_piece = piece_type;
    }
}
