use std::collections::HashMap;

pub mod role;
pub mod position;
pub mod color;

use role::Role;
use position::Position;
use color::Color;

#[derive(Clone, Debug)]
pub struct Piece {
    pub color: Color,
    pub role: Role,
    pub position: Position,
    pub has_moved: bool
}

impl Piece {
    pub fn get_possible_moves(&self, board: &HashMap<Position, Piece>) -> Option<Vec<Position>> {
    
        // Check self.role and return possible moves for that role at self.position
        let mut moves: Vec<Position> = match self.role {
            Role::King => {
                let mut _moves: Vec<Position> = Vec::new();

                match get_possible_moves_diagonal(self, 1, board) {
                    Some(mut moves) => _moves.append(&mut moves),
                    None => () 
                }

                match get_possible_moves_straight(self, 1, board) {
                    Some(mut moves) => _moves.append(&mut moves),
                    None => ()
                }

                return Some(_moves);
            },
    
            Role::Queen => {
                let mut _moves: Vec<Position> = Vec::new();

                match get_possible_moves_diagonal(self, 7, board) {
                    Some(mut moves) => _moves.append(&mut moves),
                    None => () 
                }

                match get_possible_moves_straight(self, 7, board) {
                    Some(mut moves) => _moves.append(&mut moves),
                    None => ()
                }

                return Some(_moves);
            },
    
            Role::Rook => {
                match get_possible_moves_straight(self, 7, board) {
                    Some(moves) => moves,
                    None => Vec::new(),
                }
            },
    
            Role::Bishop => {
                match get_possible_moves_diagonal(self, 7, board) {
                    Some(moves) => moves,
                    None => Vec::new(),
                }
            },
    
            Role::Knight => {
                match get_possible_moves_knight(self, board) {
                    Some(moves) => moves,
                    None => Vec::new(),
                }
            },
    
            Role::Pawn => {
                match get_possible_moves_pawn(self, board) {
                    Some(moves) => moves,
                    None => Vec::new(),
                }
            }
        };

        return Some(moves);
    }

    fn position_available(&self, _position: Position, board: &HashMap<Position, Piece>) -> bool {
        let mut position_available: bool;
    
        // Check if Position is occupied by same color
        match board.get(&_position) {
            Some(piece) => {
                if piece.color != self.color {
                    position_available = true;
                } else {
                    position_available = false;
                }
            },
            None => position_available = true
        }
    
        // Check if king is vulnerable


        // Return bool
        return position_available;
    }
}

fn get_possible_moves_straight(_piece: &Piece, max_steps: i8, board: &HashMap<Position, Piece>) ->  Option<Vec<Position>> {
    let mut moves: Vec<Position> = Vec::new();

    // Directions
    let mut n_stopped: bool = false;
    let mut w_stopped: bool = false;
    let mut s_stopped: bool = false;
    let mut e_stopped: bool = false;

    // Steps away from piece
    let mut step: i8 = 1;

    // Check all directions step away from piece
    while !n_stopped || !n_stopped || !s_stopped || !s_stopped {

        // If max steps has been reached, break
        if max_steps < step {
            break;
        }

        // Check if blockage has been reached previous iteration of loop
        if !n_stopped {
            if _piece.position_available( Position { row: _piece.position.row - step, column: _piece.position.column }, board){
                moves.push( Position { row: _piece.position.row - step, column: _piece.position.column } );
            } else {
                n_stopped = true;
            }
        }

        // Check if blockage has been reached previous iteration of loop
        if !w_stopped {
            if _piece.position_available( Position { row: _piece.position.row, column: _piece.position.column - step }, board){
                moves.push( Position { row: _piece.position.row, column: _piece.position.column - step } );
            } else {
                w_stopped = true;
            }
        }

        // Check if blockage has been reached previous iteration of loop
        if !s_stopped {
            if _piece.position_available( Position { row: _piece.position.row + step, column: _piece.position.column }, board){
                moves.push( Position { row: _piece.position.row + step, column: _piece.position.column } );
            } else {
                s_stopped = true;
            }
        }

        // Check if blockage has been reached previous iteration of loop
        if !e_stopped {
            if _piece.position_available( Position { row: _piece.position.row, column: _piece.position.column + step }, board){
                moves.push( Position { row: _piece.position.row, column: _piece.position.column + step } );
            } else {
                e_stopped = true;
            }
        }

        // Move a step
        step += 1;
    }

    return Some(moves);
}

fn get_possible_moves_diagonal(_piece: &Piece, max_steps: i8, board: &HashMap<Position, Piece>) ->  Option<Vec<Position>> {
    let mut moves: Vec<Position> = Vec::new();

    // Directions
    let mut nw_stopped: bool = false;
    let mut ne_stopped: bool = false;
    let mut sw_stopped: bool = false;
    let mut se_stopped: bool = false;

    // Steps away from piece
    let mut step: i8 = 1;

    // Check all directions step away from piece
    while !nw_stopped || !ne_stopped || !sw_stopped || !se_stopped {

        // If max steps has been reached, break
        if max_steps < step {
            break;
        }

        // Check if blockage has been reached previous iteration of loop
        if !nw_stopped {
            if _piece.position_available( Position { row: _piece.position.row - step, column: _piece.position.column - step }, board){
                moves.push( Position { row: _piece.position.row - step, column: _piece.position.column - step } );
            } else {
                nw_stopped = true;
            }
        }

        // Check if blockage has been reached previous iteration of loop
        if !ne_stopped {
            if _piece.position_available( Position { row: _piece.position.row - step, column: _piece.position.column + step }, board){
                moves.push( Position { row: _piece.position.row - step, column: _piece.position.column + step } );
            } else {
                nw_stopped = true;
            }
        }

        // Check if blockage has been reached previous iteration of loop
        if !sw_stopped {
            if _piece.position_available( Position { row: _piece.position.row + step, column: _piece.position.column - step }, board){
                moves.push( Position { row: _piece.position.row + step, column: _piece.position.column - step } );
            } else {
                nw_stopped = true;
            }
        }

        // Check if blockage has been reached previous iteration of loop
        if !nw_stopped {
            if _piece.position_available( Position { row: _piece.position.row + step, column: _piece.position.column + step }, board){
                moves.push( Position { row: _piece.position.row + step, column: _piece.position.column + step } );
            } else {
                nw_stopped = true;
            }
        }

        step += 1;
    }

    return Some(moves);
}

fn get_possible_moves_knight(_piece: &Piece, board: &HashMap<Position, Piece>) -> Option<Vec<Position>> {
    let mut moves: Vec<Position> = Vec::new();

    if _piece.position_available( Position { row: _piece.position.row - 2, column: _piece.position.column - 1 }, board){
        moves.push( Position { row: _piece.position.row - 2, column: _piece.position.column - 1 } );
    }

    if _piece.position_available( Position { row: _piece.position.row - 1, column: _piece.position.column - 2 }, board){
        moves.push( Position { row: _piece.position.row - 1, column: _piece.position.column - 2 } );
    }

    if _piece.position_available( Position { row: _piece.position.row - 2, column: _piece.position.column + 1 }, board){
        moves.push( Position { row: _piece.position.row - 2, column: _piece.position.column + 1 } );
    }

    if _piece.position_available( Position { row: _piece.position.row - 1, column: _piece.position.column + 2 }, board){
        moves.push( Position { row: _piece.position.row - 1, column: _piece.position.column + 2 } );
    }

    if _piece.position_available( Position { row: _piece.position.row + 2, column: _piece.position.column - 1 }, board){
        moves.push( Position { row: _piece.position.row + 2, column: _piece.position.column - 1 } );
    }

    if _piece.position_available( Position { row: _piece.position.row + 1, column: _piece.position.column - 2 }, board){
        moves.push( Position { row: _piece.position.row + 1, column: _piece.position.column - 2 } );
    }

    if _piece.position_available( Position { row: _piece.position.row + 2, column: _piece.position.column + 1 }, board){
        moves.push( Position { row: _piece.position.row + 2, column: _piece.position.column + 1 } );
    }

    if _piece.position_available( Position { row: _piece.position.row + 1, column: _piece.position.column + 2 }, board){
        moves.push( Position { row: _piece.position.row + 1, column: _piece.position.column + 2 } );
    }

    return Some(moves);
}

fn get_possible_moves_pawn(_piece: &Piece, board: &HashMap<Position, Piece>) -> Option<Vec<Position>> {
    let mut moves: Vec<Position> = Vec::new();
    let mut direction: i8;

    if _piece.color == Color::White {
        direction = -1;
    } else {
        direction = 1;
    }

    if _piece.position_available( Position { row: _piece.position.row + direction, column: _piece.position.column, }, board) {
        moves.push( Position { row: _piece.position.row + direction, column: _piece.position.column, } );

        if !_piece.has_moved && _piece.position_available( Position { row: _piece.position.row + direction * 2, column: _piece.position.column, }, board ){
            moves.push( Position { row: _piece.position.row + direction * 2, column: _piece.position.column, } );
        }
    }

    match board.get( &Position { row: _piece.position.row + direction, column: _piece.position.column + 1, } ) {
        Some(piece) => {
            if piece.color != _piece.color {
                moves.push( Position { row: _piece.position.row + direction, column: _piece.position.column + 1, } );
            }
        },
        None => ()
    }

    match board.get( &Position { row: _piece.position.row + direction, column: _piece.position.column - 1, } ) {
        Some(piece) => {
            if piece.color != _piece.color {
                moves.push( Position { row: _piece.position.row + direction, column: _piece.position.column - 1, } );
            }
        },
        None => ()
    }

    return Some(moves);
}

