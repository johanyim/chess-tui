//Board consists of ranks
//ranks consist of tiles
//
//
//
//FEN string notation
//converter from FEN to board state
//storing history as FEN string
//

use chess_tui::board::Board;
fn main() {
    println!("Invalid");
    Board::build_from_fen("askdj/asdkjh".to_string());
    println!("Correct");
    Board::build_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string());
}

