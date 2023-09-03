use crate::piece::Piece;

//#[derive(Clone, Copy)]
//pub enum Tile {
    //Piece,
    //Empty,
//}
//impl Tile {
//    fn to_char(self) -> char {
//       match self {
//            
//        }
//   } 
//}


//impl std::fmt::Display for Tile {
    //fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //write!(f, "(value a: , value b: )\n(test)")
    //}
//}





pub struct Board {
    tiles: [[Option<Piece>;8];8],
}

impl Board {
    pub fn build_from_fen(fen: String) -> Board{
        

        let parts : Vec<_> = fen.trim().split_whitespace().collect();
        //default board is given upon invalid fen notation
        if parts.len() != 6 {
            println!("Invalid FEN string: {fen}, generating default board instead.");
            return Board::new();
        }

        let ranks : Vec<_> = parts[0].split("/").collect();
        if ranks.len() != 8 {
            println!("Invalid FEN string: {fen}, generating default board instead.");
            return Board::new();
        }

        
        let mut tiles:[[Option<Piece>;8];8] = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None]
        ];
        for i in 0..8{
            tiles[i] = rank_from_str("asdl");
        }
        
        let b  = Board {tiles: tiles};
        println!("{}", b); 
        
        return b;
    }

    
    pub fn new() -> Board {
        let default = String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1") ;
        return Self::build_from_fen(default);
    }
    
    
}


impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut b = String::from("");
        for rank in &self.tiles {
            for tile in rank {
                match tile {
                    Some(p) => b.push('t'),//push(&p.fen()), 
                    None => b.push('.'),
                };
            }
            b.push('\n');
        }

        write!(f, "{}",b)
    }
}

pub fn rank_from_str(rank: &str) -> [Option<Piece>; 8] {
    return [
        None, None, 
        None, None, 
        None, None, 
        None, None, 
    ]
}
