use crate::{BitBoard, BitBoardSet, Board, Color, ZobristBoard};

impl Board {
    fn from_fen(fen: &str) -> Result<Self, String> {
        let mut board = Self::new();
        let mut words = fen.split_whitespace();

        // ===== POSITION ====

        let Some(position) = words.next() else {
            return Err("Missing position field in FEN string".to_owned());
        };

        let mut square_index = 0_u32;
        for char in position.chars() {
            match char {
                '/' => {
                    if square_index > 0 && square_index.rem_euclid(8) != 0 {
                        return Err("Invalid FEN".to_owned());
                    }
                }
                'p' | 'P' | 'n' | 'N' | 'b' | 'B' | 'r' | 'R' | 'q' | 'Q' | 'k' | 'K' => {
                    if char.is_uppercase() {
                        board.inner.position.white.0 |= 1_u64.wrapping_shl(square_index);
                    } else {
                        board.inner.position.black.0 |= 1_u64.wrapping_shl(square_index);
                    }
                    match char {
                        'p' | 'P' => {
                            board.inner.position.pawns.0 |= 1_u64.wrapping_shl(square_index);
                        }
                        'n' | 'N' => {
                            board.inner.position.pawns.0 |= 1_u64.wrapping_shl(square_index);
                        }
                        'b' | 'B' => {
                            board.inner.position.bishops.0 |= 1_u64.wrapping_shl(square_index);
                        }
                        'r' | 'R' => {
                            board.inner.position.rooks.0 |= 1_u64.wrapping_shl(square_index);
                        }
                        'q' | 'Q' => {
                            board.inner.position.queens.0 |= 1_u64.wrapping_shl(square_index);
                        }
                        'k' | 'K' => {
                            board.inner.position.kings.0 |= 1_u64.wrapping_shl(square_index);
                        }
                        _ => unreachable!(),
                    }
                    square_index += 1;
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    #[expect(clippy::unwrap_used)]
                    let number = char.to_digit(10).unwrap();
                    square_index += 1;
                }
                _ => {
                    return Err("Invalid FEN".to_owned());
                }
            }
        }

        if square_index != 64 {
            return Err("Invalid FEN".to_owned());
        }

        // ===== SIDE TO MOVE ====

        let Some(side) = words.next() else {
            return Err("Missing FEN field: side to move".to_owned());
        };

        Ok(board)
    }
}

// #[cfg(test)]
// mod fen_test {
//     use super::*;

//     #[test]
//     fn starting_position() {
//         let starting_board = Board {
//             inner: ZobristBoard { position: BitBoardSet {
//                 panws:
//             }, side_to_move: (), castle_rights: (), en_passant: () }
//         }
//     }
// }
