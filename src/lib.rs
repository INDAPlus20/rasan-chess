use std::fmt;
use std::error::Error;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

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
            board: HashMap::new(),
            active_color: Color::White
        }
    }

    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {

        // Convert String to Position
        let _from_pos: Position = Position::new(_from);
        

        // Get reference to Piece at Position
        let _active: &Piece = self.board.get(&_from_pos).unwrap();

        // Shuffle Pieces if possible
        /* if _active.get_possible_moves().unwrap().contains(&_to) && _active.color != active_color {
            self.board.remove(&_to);
            let piece = self.board.remove(&_from);
            self.board.insert(piece, _to);
        } */
        None
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _postion: String) -> Option<Vec<String>> {
        None
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
        
        write!(f, "")
    }
}

// --------------------------
// ####### PIECE CODE #######
// --------------------------

#[derive(Hash, Eq, PartialEq, Debug)]
enum Color {
    White,
    Black
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    row: char, 
    column: i8 
}

impl Position {
    fn new(_pos: String) -> Result<Self, Box<dyn Error>> {

        // Checks if String is a valid character length
        if _pos.chars().count() == 2 {

            // Creates variables
            let mut _row: char = Default::default();
            let mut _column: i8 = Default::default();

            // Loops through characters
            for _char in _pos.chars() {
                let _possible_number = _char.to_digit(10);
                
                // Checks if character is a number 0-9, if not, set _row to character if character is a-h
                match _possible_number {
                    Ok(digit) => _column = digit as i8,
                    Err(e) => 
                        if _row == Default::default() && _char.to_digit(18).unwrap() - 10 >= 0 {
                            _row = _char.to_uppercase();
                        } else {
                            None;
                        }
                };
            }

            // Checks if both _row and _column has gotten values
            if _row != Default::default() || _column != Default::default() {
                return Ok(Self {
                    row: _row, 
                    column: _column 
                });
            }
        }

        // Returns error if String is not valid
        return Err("Not a valid input");
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Piece {
    color: Color
}

impl Piece {
    fn get_possible_moves(&self, _postion: String) -> Option<Vec<String>> {
        None
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

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