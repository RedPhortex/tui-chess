use ratatui::layout::{ Constraint, Direction, Layout, Rect };
use pleco::{ BitMove, Board, File, Piece, Player, Rank, SQ };
use std::{ iter::{ once, repeat }, rc::Rc };

/// Creates a board layout with specified rows/columns and borders.
///
/// # Arguments
///
/// * `direction` - The direction in which the layout is split (Vertical for rows, Horizontal for columns).
/// * `board_size` - The number of sections (rows or columns) in the board.
/// * `border_size` - The size of the border around the board.
/// * `available_size` - The total size of the board in the chosen direction.
/// * `area` - The `Rect` that represents the entire area available for the board.
///
/// # Returns
///
/// A reference-counted slice of `Rect` elements representing the split board layout.
pub fn create_board_layout(
    direction: Direction,
    available_size: u16,
    board_size: usize,
    border_size: u16,
    area: Rect
) -> Rc<[Rect]> {
    // Maybe add Vec::with_capacity() for more efficient memory allocation
    return Layout::new(
        direction,
        once(Constraint::Length(border_size)) // Top/left border
            .chain(repeat(Constraint::Length(available_size / (board_size as u16))).take(board_size))
            .chain(once(Constraint::Length(border_size))) // Bottom/right border
    ).split(area);
}

/// Returns all legal moves that can be made from the given square.
///
/// # Arguments
///
/// * `square` - The square to generate moves from.
/// * `board` - The board to generate moves from.
///
/// # Returns
///
/// A vector of all legal moves that can be made from the given square.
pub fn moves_from_square(square: SQ, board: &pleco::Board) -> Vec<BitMove> {
    board
        .generate_moves()
        .into_iter()
        .filter(|move_| move_.get_src() == square)
        .collect()
}

/// Returns the first move with the given destination in the given moves.
///
/// # Arguments
///
/// * `dest` - The destination to check.
/// * `moves` - The moves to check.
///
/// # Returns
///
/// The first move with the given destination in the given moves.
pub fn move_to_square(square: SQ, moves: &Vec<BitMove>) -> BitMove {
    *moves
        .iter()
        .filter(|move_| move_.get_dest() == square)
        .next()
        .unwrap()
}

/// Checks if there is a move with the given destination in the given moves.
///
/// # Arguments
///
/// * `dest` - The destination to check.
/// * `moves` - The moves to check.
///
/// # Returns
///
/// A boolean indicating whether the destination is in the given moves.
pub fn dest_in_moves(dest: SQ, moves: &Vec<BitMove>) -> bool {
    moves.iter().any(|move_| move_.get_dest() == dest)
}

/// Converts a piece to a character.
///
/// # Arguments
///
/// * `piece` - The piece to convert.
///
/// # Returns
///
/// A character representing the piece.
pub fn piece_to_char(piece: Piece) -> &'static str {
    match piece {
        Piece::WhiteKing => "♔",
        Piece::WhiteQueen => "♕",
        Piece::WhiteRook => "♖",
        Piece::WhiteBishop => "♗",
        Piece::WhiteKnight => "♘",
        Piece::WhitePawn => "♙",
        Piece::BlackKing => "♚",
        Piece::BlackQueen => "♛",
        Piece::BlackRook => "♜",
        Piece::BlackBishop => "♝",
        Piece::BlackKnight => "♞",
        Piece::BlackPawn => "♟",
        Piece::None => "",
    }
}

/// Converts a square to a string.
///
/// # Arguments
///
/// * `square` - The square to convert.
///
/// # Returns
///
/// A string representing the square.
pub fn square_to_string(square: SQ) -> &'static str {
    match square {
        SQ::A1 => "A1",
        SQ::B1 => "B",
        SQ::C1 => "C",
        SQ::D1 => "D",
        SQ::E1 => "E",
        SQ::F1 => "F",
        SQ::G1 => "G",
        SQ::H1 => "H",
        SQ::A2 => "2",
        SQ::A3 => "3",
        SQ::A4 => "4",
        SQ::A5 => "5",
        SQ::A6 => "6",
        SQ::A7 => "7",
        SQ::A8 => "8",
        _ => "",
    }
}

/// Gets the file from a column.
///
/// # Arguments
///
/// * `col` - The column to get the file from.
///
/// # Returns
///
/// The file from the column.
pub fn get_file(col: u8) -> File {
    match col {
        0 => File::A,
        1 => File::B,
        2 => File::C,
        3 => File::D,
        4 => File::E,
        5 => File::F,
        6 => File::G,
        7 => File::H,
        _ => unreachable!(),
    }
}

/// Gets the rank from a row.
///
/// # Arguments
///
/// * `row` - The row to get the rank from.
/// * `main_player` - The current player.
///
/// # Returns
///
/// The rank from the row.
pub fn get_rank(row: u8, main_player: Player) -> Rank {
    match main_player {
        Player::Black =>
            match row {
                0 => Rank::R1,
                1 => Rank::R2,
                2 => Rank::R3,
                3 => Rank::R4,
                4 => Rank::R5,
                5 => Rank::R6,
                6 => Rank::R7,
                7 => Rank::R8,
                _ => unreachable!(),
            }
        Player::White =>
            match row {
                0 => Rank::R8,
                1 => Rank::R7,
                2 => Rank::R6,
                3 => Rank::R5,
                4 => Rank::R4,
                5 => Rank::R3,
                6 => Rank::R2,
                7 => Rank::R1,
                _ => unreachable!(),
            }
    }
}

/// Gets the current player based on the number of moves played.
///
/// # Arguments
///
/// * `moves_played` - The number of moves played.
///
/// # Returns
///
/// The current player based on the number of moves played.
pub fn get_current_player(moves_played: u16) -> Player {
    match moves_played % 2 {
        0 => Player::Black,
        1 => Player::White,
        _ => unreachable!(),
    }
}

/// Check if the game is over.
///
/// # Arguments
///
/// * `board` - The board to check.
///
/// # Returns
///
/// `true` if the game is over, otherwise `false`.
pub fn is_game_over(board: &Board) -> bool {
    board.checkmate() || board.stalemate()
}

#[cfg(test)]
mod tests {
    use pleco::Board;

    use super::*;

    #[test]
    fn test_create_board_layout() {
        let layout = create_board_layout(Direction::Vertical, 10, 3, 1, Rect::new(0, 0, 10, 10));
        assert_eq!(layout.len(), 5);
    }

    #[test]
    fn test_moves_from_square() {
        let board = Board::default();
        let moves = moves_from_square(SQ::A2, &board);
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_moves_to_rank() {
        let board = Board::default();
        let moves = move_to_square(SQ::A4, &moves_from_square(SQ::A2, &board));
        assert_eq!(moves.get_dest(), SQ::A4);
        assert_eq!(moves.get_src(), SQ::A2);
    }

    #[test]
    fn test_dest_in_moves() {
        let board = Board::default();
        let moves = moves_from_square(SQ::A2, &board);
        assert!(dest_in_moves(SQ::A3, &moves));
        assert!(!dest_in_moves(SQ::A5, &moves));
    }

    #[test]
    fn test_piece_to_char() {
        assert_eq!(piece_to_char(Piece::WhiteKing), "♔");
        assert_eq!(piece_to_char(Piece::WhiteQueen), "♕");
        assert_eq!(piece_to_char(Piece::WhiteRook), "♖");
        assert_eq!(piece_to_char(Piece::WhiteBishop), "♗");
        assert_eq!(piece_to_char(Piece::WhiteKnight), "♘");
        assert_eq!(piece_to_char(Piece::WhitePawn), "♙");
        assert_eq!(piece_to_char(Piece::BlackKing), "♚");
        assert_eq!(piece_to_char(Piece::BlackQueen), "♛");
        assert_eq!(piece_to_char(Piece::BlackRook), "♜");
        assert_eq!(piece_to_char(Piece::BlackBishop), "♝");
        assert_eq!(piece_to_char(Piece::BlackKnight), "♞");
        assert_eq!(piece_to_char(Piece::BlackPawn), "♟");
        assert_eq!(piece_to_char(Piece::None), "");
    }

    #[test]
    fn test_square_to_string() {
        assert_eq!(square_to_string(SQ::A1), "A1");
        assert_eq!(square_to_string(SQ::B1), "B");
        assert_eq!(square_to_string(SQ::C1), "C");
        assert_eq!(square_to_string(SQ::D1), "D");
        assert_eq!(square_to_string(SQ::E1), "E");
        assert_eq!(square_to_string(SQ::F1), "F");
        assert_eq!(square_to_string(SQ::G1), "G");
        assert_eq!(square_to_string(SQ::H1), "H");
        assert_eq!(square_to_string(SQ::A2), "2");
        assert_eq!(square_to_string(SQ::A3), "3");
        assert_eq!(square_to_string(SQ::A4), "4");
        assert_eq!(square_to_string(SQ::A5), "5");
        assert_eq!(square_to_string(SQ::A6), "6");
        assert_eq!(square_to_string(SQ::A7), "7");
        assert_eq!(square_to_string(SQ::A8), "8");
    }

    #[test]
    fn test_get_file() {
        assert_eq!(get_file(0), File::A);
        assert_eq!(get_file(1), File::B);
        assert_eq!(get_file(2), File::C);
        assert_eq!(get_file(3), File::D);
        assert_eq!(get_file(4), File::E);
        assert_eq!(get_file(5), File::F);
        assert_eq!(get_file(6), File::G);
        assert_eq!(get_file(7), File::H);
    }

    #[test]
    fn test_get_rank() {
        assert_eq!(get_rank(0, Player::Black), Rank::R1);
        assert_eq!(get_rank(1, Player::Black), Rank::R2);
        assert_eq!(get_rank(2, Player::Black), Rank::R3);
        assert_eq!(get_rank(3, Player::Black), Rank::R4);
        assert_eq!(get_rank(4, Player::Black), Rank::R5);
        assert_eq!(get_rank(5, Player::Black), Rank::R6);
        assert_eq!(get_rank(6, Player::Black), Rank::R7);
        assert_eq!(get_rank(7, Player::Black), Rank::R8);

        assert_eq!(get_rank(0, Player::White), Rank::R8);
        assert_eq!(get_rank(1, Player::White), Rank::R7);
        assert_eq!(get_rank(2, Player::White), Rank::R6);
        assert_eq!(get_rank(3, Player::White), Rank::R5);
        assert_eq!(get_rank(4, Player::White), Rank::R4);
        assert_eq!(get_rank(5, Player::White), Rank::R3);
        assert_eq!(get_rank(6, Player::White), Rank::R2);
        assert_eq!(get_rank(7, Player::White), Rank::R1);
    }

    #[test]
    fn test_get_current_player() {
        assert_eq!(get_current_player(1), Player::White);
        assert_eq!(get_current_player(2), Player::Black);
        assert_eq!(get_current_player(3), Player::White);
        assert_eq!(get_current_player(4), Player::Black);
    }
}
