#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Position {
    pub row: i8, 
    pub column: i8 // Represented as a letter in String
}

impl Position {
    pub fn new(_pos: String) -> Option<Self> {

        // Checks if String is a valid character length
        if _pos.chars().count() == 2 {

            // Creates variables
            let mut _row: i8;
            let mut _column: i8;

            // Split String to characters
            let chars: Vec<char> = _pos.chars().collect();

            // Convert letter character into i8 (easier to use in code)
            match chars[0] {
                'a' => _column = 1,
                'b' => _column = 2,
                'c' => _column = 3,
                'd' => _column = 4,
                'e' => _column = 5,
                'f' => _column = 6,
                'g' => _column = 7,
                'h' => _column = 8,
                _ => {
                    print!("Not a vaild position");
                    return None;
                }
            };

            // Convert character
            match chars[1].to_digit(10) {
                Some(value) => {
                    if value > 8 || value < 1 {
                        print!("Not a vaild position");
                        return None;
                    } else {
                        _row = value as i8;
                    };
                },
                None => {
                    print!("Not a vaild position");
                    return None;
                }
            };

            // Return position
            return Some(Position {
                row: _row,
                column: _column
            });

        } else {
            print!("Not a vaild position");
            return None;
        }
    }

    // Convert position back to String
    pub fn to_string(self) -> String {
        return format!("{}{}", self.row.to_string(), self.column.to_string());
    }

    // Check if position is within board constraints
    pub fn is_valid(self) -> bool {
        if (self.row < 1 || self.row > 8 || self.column < 1 || self.column > 8) == true {
            return false;
        } else {
            return true;
        }
    }
}