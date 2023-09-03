pub mod board;
pub mod piece;



#[cfg(test)]
mod tests {
    use crate::piece::*;
    use crate::board::*;
    #[test]
    fn single_piece_fen() {
        let p = Piece {unit: Unit::Rook, color: Color::Black };
        let fen : char = p.fen();
        assert_eq!(fen, 'r');

        let p = Piece {unit: Unit::Queen, color: Color::White };
        let fen : char = p.fen();
        assert_eq!(fen, 'Q');
    }
    
    #[test]
    fn build_default_board() {
        let b = Board::new();
        
    }
    
    #[test]
    fn build_board_from_fen() {
        let b = Board::build_from_fen(String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"));
    }

    #[test]
    fn invalid_fen() {
        let b = Board::build_from_fen(String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w"));
    }


}
