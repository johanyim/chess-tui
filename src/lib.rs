pub mod board;
pub mod piece;



#[cfg(test)]
mod tests {
    use crate::piece::*;

    #[test]
    fn single_piece_fen() {
        let p = Piece {unit: Unit::Rook, color: Color::Black };
        let fen : char = p.fen();
        assert_eq!(fen, 'r');

        let p = Piece {unit: Unit::Queen, color: Color::White };
        let fen : char = p.fen();
        assert_eq!(fen, 'Q');

        

    }
}
