use std::fmt;
use std::collections::HashMap;

mod piece;
mod gamestate;

use piece::position::Position;
use piece::color::Color;
use piece::role::Role;
use piece::Piece;
use gamestate::GameState;

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    board: HashMap<Position, Piece>,
    active_color: Color
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            board: setup_board(),
            active_color: Color::White
        }
    }

    fn setup_board() -> HashMap<Position, Piece> {
        let mut board: HashMap<Position, Piece> = HashMap::new();

        // Adds pawns to board
        for x in 1..9 {

            // Insert white pawns
            let white_pawn_position = Position {
                row: 7,
                column: x
            };

            board.insert(
                white_pawn_position,
                Piece {
                    color: Color::White,
                    role: Role::Pawn,
                    position: white_pawn_position,
                    has_moved: false
                }
            );

            // Insert black pawns
            let black_pawn_position = Position {
                row: 2,
                column: x
            };

            board.insert(
                black_pawn_position,
                Piece {
                    color: Color::Black,
                    role: Role::Pawn,
                    position: black_pawn_position,
                    has_moved: false
                }
            );
        }

        return board;
    }

    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {

        // Convert String _from to Position
        let _from_pos: Position; 
        match Position::new(_from) {
            Ok(position) => _from_pos = position,
            Err(e) => {
                println!("{:?}", e);
                return Some(self.state);
            }
        };

        // Convert String _to to Position
        let _to_pos: Position; 
        match Position::new(_to) {
            Ok(position) => _to_pos = position,
            Err(e) => {
                println!("{:?}", e);
                return Some(self.state);
            }
        };

        // Get piece at position
        match self.board.get(&_from_pos) {
            Some(_piece_ref) => 

                // Check if piece is of correct color
                if _piece_ref.color != self.active_color {

                    // Get possible moves
                    match _piece_ref.get_possible_moves(self.board) {
                        Some(moves) => 

                            // Check if desired move is possible
                            match moves.get(&_to_pos) {
                                Some(_piece_pos) => {

                                    // Check if own king becomes threatened
                                
                                    // Moves piece and possibly removes another piece
                                    self.board.remove(&_to_pos);
                                    let mut _piece: piece::Piece = self.board.remove(&_from_pos).unwrap();

                                    // Set has_moved (from original position) to true
                                    if _piece.has_moved != true {
                                        _piece.has_moved = true;
                                    }

                                    self.board.insert(_to_pos, _piece);

                                    // Switches active color
                                    self.active_color = match self.active_color {
                                        Color::White => Color::Black,
                                        Color::Black => Color::White
                                    }
                                },
                                None => println!("Illegal move")
                            },
                        None => println!("No possible moves")
                    };
                },
            None => println!("No piece at speficied position")
        };

        // Check if king is threatened

        // Check game state
        return None;
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        return self.state;
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _position: String) -> Option<Vec<Position>> {

        // Convert String _from to Position
        let position: Position; 
        match Position::new(_position) {
            Ok(position) => position = position,
            Err(e) => {
                println!("{:?}", e);
                return None;
            }
        };

        // Check position
        match self.board.get(&position) {
            Some(piece) => return piece.get_possible_moves(self.board),
            None => return None
        };
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        let mut output: String;
        for x in 1..9 {
            for y in 1..9 {
                let piece: &str = match self.board.get( &Position { row: x, column: y} ) {
                    Some(piece) => {
                        match piece.role {
                            Role::King => "K",
                            Role::Queen => "Q",
                            Role::Rook => "R",
                            Role::Bishop => "B",
                            Role::Knight => "K",
                            Role::Pawn => "P"
                        }
                    },
                    None => "*"
                };

                // Adds unicode character
                output.push_str(&String::from(format!("{} ", piece)));
            }

            // New line
            output.push_str(&String::from(format!("\n")));
        }
        write!(f, "{}", output)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::gamestate::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}