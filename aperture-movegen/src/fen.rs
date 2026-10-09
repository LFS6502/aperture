use super::*;
use crate::{
    Color, bitboard::BitBoard, bitboard::BitBoardSet, board::Board, board::CastleRights,
    board::ZobristBoard, square::Square,
};

impl super::Board {
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut board = Self::new();
        let mut words = fen.split_whitespace();

        // ===== POSITION ====

        let Some(position) = words.next() else {
            return Err("Missing position field in FEN string".to_owned());
        };

        let mut square_index = Square::A8 as u32; // 56
        assert_eq!(Square::A8 as u8, 56);
        for char in position.chars() {
            match char {
                '/' => {
                    // if square_index > 0 && square_index.rem_euclid(8) != 7 {
                    //     return Err("Invalid amount per rank".to_owned());
                    // }
                    square_index -= 16;
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
                            board.inner.position.knights.0 |= 1_u64.wrapping_shl(square_index);
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
                    square_index += number;
                }
                _ => {
                    return Err("Unknown symbol in position field".to_owned());
                }
            }
        }

        // if square_index != 8 {
        //     return Err("Total does not add up".to_owned());
        // }

        // ===== SIDE TO MOVE ====

        let Some(side) = words.next() else {
            return Err("Missing FEN field: side to move".to_owned());
        };

        match side {
            "w" => {
                board.inner.side_to_move = Color::White;
            }
            "b" => {
                board.inner.side_to_move = Color::Black;
            }
            _ => return Err("Invalid active color".to_owned()),
        };

        // ===== CASTLING RIGHTS ====

        let Some(rights) = words.next() else {
            return Err("Missing FEN field: castling".to_owned());
        };

        board.inner.castle_rights = CastleRights {
            white_kingside: rights.contains('K'),
            white_queenside: rights.contains('Q'),
            black_kingside: rights.contains('k'),
            black_queenside: rights.contains('q'),
        };

        // ===== EN PASSANT ====

        let Some(en_passant) = words.next() else {
            return Err("Missing FEN field: en passant".to_owned());
        };

        board.inner.en_passant = match en_passant {
            "-" => None,
            _ => {
                let Ok(square) = en_passant.parse() else {
                    return Err("Invalid en-passant square".to_owned());
                };
                Some(square)
            }
        };

        // ===== HALFMOVE CLOCK ====

        let Some(halfmove_clock) = words.next() else {
            return Err("Missing FEN field: halfmove clock".to_owned());
        };

        let Ok(halfmove_clock) = halfmove_clock.parse::<u8>() else {
            return Err("Invalid halfmove clock".to_owned());
        };

        board.halfmove_clock = halfmove_clock;

        // ===== FULLMOVE NUMBER ====

        let Some(fullmove_number) = words.next() else {
            return Err("Missing FEN field: halfmove clock".to_owned());
        };

        let Ok(fullmove_number) = fullmove_number.parse::<u16>() else {
            return Err("Invalid fullmove number".to_owned());
        };

        board.fullmove_number = fullmove_number;

        Ok(board)
    }
}

#[cfg(test)]
mod fen_test {
    use super::*;
}
