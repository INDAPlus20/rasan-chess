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
    state: GameState,
    board: HashMap<Position, Piece>,
    active_color: Color,

    white_king: &Piece,
    black_King: &Piece
}

impl Game {

    // Initialises a new board with pieces.
    pub fn new() -> Game {

        //Board
        let mut _board: HashMap<Position, Piece> = HashMap::new();

        // Insert pawns
        for x in 1..9 {
            Self::insert_piece(&mut _board, Color::White, Role::Pawn, Position {row: 2, column: x});
            Self::insert_piece(&mut _board, Color::Black, Role::Pawn, Position {row: 7, column: x});
        }

        // Insert rooks
        Self::insert_piece(&mut _board, Color::Black, Role::Rook, Position {row: 8, column: 1});
        Self::insert_piece(&mut _board, Color::Black, Role::Rook, Position {row: 8, column: 8});
        Self::insert_piece(&mut _board, Color::White, Role::Rook, Position {row: 1, column: 1});
        Self::insert_piece(&mut _board, Color::White, Role::Rook, Position {row: 1, column: 8});

        // Insert knights
        Self::insert_piece(&mut _board, Color::Black, Role::Knight, Position {row: 8, column: 2});
        Self::insert_piece(&mut _board, Color::Black, Role::Knight, Position {row: 8, column: 7});
        Self::insert_piece(&mut _board, Color::White, Role::Knight, Position {row: 1, column: 2});
        Self::insert_piece(&mut _board, Color::White, Role::Knight, Position {row: 1, column: 7});

        // Insert bishops
        Self::insert_piece(&mut _board, Color::Black, Role::Bishop, Position {row: 8, column: 3});
        Self::insert_piece(&mut _board, Color::Black, Role::Bishop, Position {row: 8, column: 6});
        Self::insert_piece(&mut _board, Color::White, Role::Bishop, Position {row: 1, column: 3});
        Self::insert_piece(&mut _board, Color::White, Role::Bishop, Position {row: 1, column: 6});

        // Insert kings
        Self::insert_piece(&mut _board, Color::Black, Role::King, Position {row: 8, column: 4});
        Self::insert_piece(&mut _board, Color::White, Role::King, Position {row: 1, column: 5});

        // Set references
        white_king = _board.get(Position {row: 8, column: 4});
        black_king = _board.get(Position {row: 1, column: 5});

        // Insert queens
        Self::insert_piece(&mut _board, Color::Black, Role::Queen, Position {row: 8, column: 5});
        Self::insert_piece(&mut _board, Color::White, Role::Queen, Position {row: 1, column: 4});

        // Initialize Game
        return Game {
            state: GameState::InProgress,
            board: _board,
            active_color: Color::White
        }
    }

    // Function to insert pieces
    fn insert_piece(board: &mut HashMap<Position, Piece>,_color: Color, _role: Role, _position: Position) {
        board.insert(
            _position.clone(),
            Piece {
                color: _color,
                role: _role,
                position: _position,
                has_moved: false
            }
        );
    }

    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {

        // Game is either over or waiting for promotion
        if self.state == GameState::GameOver || self.state == GameState::Promotion {
            return Some(self.state);
        }

        // Convert String _from to Position
        let from_pos: Position; 
        match Position::new(_from) {
            Some(position) => from_pos = position,
            None => return Some(self.state)
        };

        // Convert String _to to Position
        let to_pos: Position; 
        match Position::new(_to) {
            Some(position) => to_pos = position,
            None => return Some(self.state)
        };

        // Get piece at position
        match self.board.get(&from_pos) {
            Some(piece) => {

                // Check if piece is of correct color
                if piece.color != self.active_color { 
                    println!("Not your turn");
                    return Some(self.state);
                }

                // Get possible moves
                match piece.get_possible_moves(&self.board) {
                    Some(moves) => {

                        // Check if desired move is possible
                        if moves.contains(&to_pos) {
                            
                            // Moves piece and possibly removes another piece
                            self.board.remove(&to_pos);
                            let mut _piece: piece::Piece = self.board.remove(&from_pos).unwrap();

                            // Modifies piece
                            if _piece.has_moved != true {
                                _piece.has_moved = true;
                            };

                            // Sets new place (inside piece)
                            _piece.position = to_pos.clone();

                            // Check if piece is pawn ready to be promoted
                            if _piece.role == Role::Pawn && (_piece.position.row == 1 || _piece.position.row == 8) {
                                self.state = GameState::Promotion;
                            }

                            // Inserts piece in board
                            self.board.insert(to_pos, _piece);

                            // Switches active color
                            self.active_color = match self.active_color {
                                Color::White => Color::Black,
                                Color::Black => Color::White
                            };

                        } else {
                            println!("Illegal move");
                            return Some(self.state);
                        };
                    },
                    None => {
                        println!("No possible moves");
                        return Some(self.state);
                    }
                }
            },
            None => {
                println!("No piece at speficied position");
                return Some(self.state);
            }
        }

        // Check if state should be changed
        match active_color {
            Color::Black => (
                if !board.contains(black_king) {
                    self.state = GameState::GameOver;
                }
            ),
            Color::White => (
                if !board.contains(white_king) {
                    self.state = GameState::GameOver;
                }
            )
        }

        // Check game state
        return Some(self.state);
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _pos: String, _role: Role) -> () {
        if self.state != GameState::Promotion || _role == Role::Pawn || _role == Role::King {
            println!("Promotion not allowed");
            return;
        }

        // Convert String _to to Position
        let pos: Position; 
        match Position::new(_pos) {
            Some(position) => pos = position,
            None => return 
        };

        // Check if piece exists at position, If, remove it to replace it
        match self.board.remove(&pos) {
            Some(piece) => {
                if piece.color == self.active_color && piece.role == Role::Pawn && (piece.position.row == 1 || piece.position.row == 8) {

                    // Insert new piece
                    self.board.insert(piece.position.clone(), Piece {
                        color: piece.color,
                        role: _role,
                        position: piece.position,
                        has_moved: true
                    });

                } else {
                    println!("Not a valid piece")
                };
            },
            None => println!("Not a valid position")
        }

        // Continue game
        self.state = GameState::InProgress;
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

        // Convert String _to to Position
        let _pos: Position; 
        match Position::new(_position) {
            Some(position) => _pos = position,
            None => return None
        };

        // Check position
        match self.board.get(&_pos) {
            Some(piece) => return piece.get_possible_moves(&self.board),
            None => return None
        };
    }
}

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

        // Output string
        let mut output: String = Default::default();

        for x in (1..9).rev() {
            for y in 1..9 {
                let piece: &str = match self.board.get( &Position { row: x, column: y} ) {
                    Some(piece) => {
                        match piece.role {
                            Role::King => "K ",
                            Role::Queen => "Q ",
                            Role::Rook => "R ",
                            Role::Bishop => "B ",
                            Role::Knight => "Kn",
                            Role::Pawn => "P "
                        }
                    },
                    None => "* "
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

        let mut game = Game::new();
        println!("{:?}", game);

        game.make_move("b1".to_string(), "a3".to_string());
        println!("{:?}", game);

        game.make_move("b8".to_string(), "a6".to_string());
        println!("{:?}", game);

        game.make_move("c2".to_string(), "c3".to_string());
        println!("{:?}", game);

        game.make_move("a6".to_string(), "c5".to_string());
        println!("{:?}", game);

        game.make_move("d2".to_string(), "d4".to_string());
        println!("{:?}", game);

        game.make_move("d7".to_string(), "d5".to_string());
        println!("{:?}", game);

        game.make_move("d4".to_string(), "c5".to_string());
        println!("{:?}", game);

        println!("{:?}", game.get_possible_moves("d5".to_string()));

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}