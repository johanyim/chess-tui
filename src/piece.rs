pub enum Unit {
    Pawn, Bishop, Knight, Rook, Queen, King,
}

pub enum Color {
    Black,
    White,
}

pub struct Piece {
    pub unit: Unit,
    //moves: Vec<Move>,
    //value: u8,  
    pub color: Color,
}


impl Piece {
    pub fn fen(self) -> char {
        use Unit::{Pawn, Bishop, Knight, Rook, Queen, King};
        let letter: char  = match self.unit {
            Pawn => 'p',
            Bishop => 'b',
            Knight => 'n',
            Rook => 'r',
            Queen => 'q',
            King => 'k',
        };
        match self.color {
            Color::White => return letter.to_ascii_uppercase(),
            Color::Black => return letter,
        };
    }
}


