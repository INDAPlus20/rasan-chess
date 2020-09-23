#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Position {
    pub row: i8, 
    pub column: i8 
}

impl Position {
    pub fn new(_pos: String) -> Result<Self, &'static str> {

        // Checks if String is a valid character length
        if _pos.chars().count() == 2 {

            // Creates variables
            let mut _row: i8 = Default::default();
            let mut _column: i8 = Default::default();

            // Loops through characters
            for _char in _pos.chars() {
                let _possible_number = _char.to_digit(10);
                
                // Checks if character is a number 0-9, if not, set _row to character if character is a-h
                match _possible_number {
                    Some(digit) =>
                        if digit + 1 <= 8 {
                            _column = (digit + 1) as i8
                        },
                    None => 
                        if _row == Default::default() && _char.to_digit(18).unwrap() - 10 >= 0 {
                            _row = (_char.to_digit(18).unwrap() - 10) as i8;
                        }
                };
            };

            // Checks if both _row and _column has gotten values
            if _row != Default::default() || _column != Default::default() {
                Self {
                    row: _row, 
                    column: _column 
                };
            };
        }

        // Returns error if String is not valid
        return Err("Not a valid input");
    }
}