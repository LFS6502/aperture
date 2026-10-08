use crate::{BitBoard, BitBoardSet, Board, CastleRights, Color, ZobristBoard};

impl Board {
    fn from_fen(fen: &str) -> Result<Self, String> {
        let mut board = Self::new();
        let mut words = fen.split_whitespace();

        // ===== POSITION ====

        let Some(position) = words.next() else {
            return Err("Missing position field in FEN string".to_owned());
        };

        let mut square_index = 56_u32; // TODO Replace this with an enum
        for char in position.chars() {
            match char {
                '/' => {
                    if square_index > 0 && square_index.rem_euclid(8) != 0 {
                        return Err("Invalid amount per rank".to_owned());
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

        if square_index != 64 {
            return Err("Total does not add up".to_owned());
        }

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
        }

        Ok(board)
    }
}

#[cfg(test)]
mod fen_test {
    use super::*;

    #[test]
    #[expect(clippy::unwrap_used)]
    fn starting_position() {
        let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let starting_board = Board {
            inner: ZobristBoard {
                position: BitBoardSet {
                    pawns: BitBoard::from([
                        0b_00000000,
                        0b_11111111,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_11111111,
                        0b_00000000,
                    ]),
                    knights: BitBoard::from([
                        0b_01000010,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_01000010,
                    ]),
                    bishops: BitBoard::from([
                        0b_00100100,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00100100,
                    ]),
                    rooks: BitBoard::from([
                        0b_10000001,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_10000001,
                    ]),
                    queens: BitBoard::from([
                        0b_00010000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00010000,
                    ]),
                    kings: BitBoard::from([
                        0b_00001000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00001000,
                    ]),
                    white: BitBoard::from([
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_11111111,
                        0b_11111111,
                    ]),
                    black: BitBoard::from([
                        0b_11111111,
                        0b_11111111,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                        0b_00000000,
                    ]),
                },
                side_to_move: Color::White,
                castle_rights: CastleRights {
                    white_short: true,
                    white_long: true,
                    black_short: true,
                    black_long: true,
                },
                en_passant: None,
            },
            fullmoves_clock: 1,
            halfmove_clock: 0,
        };

        assert_eq!(Board::from_fen(starting_fen).unwrap(), starting_board);
    }
}
