#![allow(unused)]

use std::{fmt::Debug, path::Display};
mod fen;

struct Move {
    from: u8,
    to: u8,
}

#[derive(Clone, Copy)]
enum Color {
    White = 0,
    Blach = 1,
}

#[derive(Clone, Copy)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

//Bitboard representation
//
// 8    0  1  2  3  4  5  6  7
// 7    8  9 10 11 12 13 14 15
// 6   16 17 18 19 20 21 22 23
// 5   24 25 26 27 28 29 30 31
// 4   32 33 34 35 36 37 38 39
// 3   40 41 42 43 44 45 46 47
// 2   48 49 50 51 52 53 54 55
// 1   56 57 58 59 60 61 62 63
//
//     A  B  C  D  E  F  G  H

#[derive(Clone, Copy)]
struct BitBoard(u64);

impl BitBoard {
    #[expect(clippy::cast_possible_truncation)]
    fn from_bytes(bytes: [u8; 8]) -> Self {
        let mut result = 0;
        for (pos, b) in bytes.into_iter().enumerate() {
            result |= (b.reverse_bits() as u64).wrapping_shl(64 - (8 * pos as u32));
        }
        Self(result)
    }
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Clone, Copy)]
struct BitBoardSet {
    pawns: BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    rooks: BitBoard,
    queens: BitBoard,
    kings: BitBoard,
    white: BitBoard,
    black: BitBoard,
}

#[derive(Clone, Copy)]
struct CastleRights {
    white_short: bool,
    white_long: bool,
    black_short: bool,
    black_long: bool,
}

#[derive(Clone, Copy)]
struct ZobristBoard {
    position: BitBoardSet,
    side_to_move: Color,
    castle_rights: CastleRights,
    en_passant: Option<u8>,
}

struct Board {
    inner: ZobristBoard,
    halfmove_clock: u8,
    fullmoves_clock: u16,
}

impl Board {
    fn new() -> Self {
        Self {
            inner: ZobristBoard {
                position: BitBoardSet {
                    pawns: BitBoard(0),
                    knights: BitBoard(0),
                    bishops: BitBoard(0),
                    rooks: BitBoard(0),
                    queens: BitBoard(0),
                    kings: BitBoard(0),
                    white: BitBoard(0),
                    black: BitBoard(0),
                },
                side_to_move: Color::White,
                castle_rights: CastleRights {
                    white_short: false,
                    white_long: false,
                    black_short: false,
                    black_long: false,
                },
                en_passant: None,
            },
            fullmoves_clock: 1,
            halfmove_clock: 0,
        }
    }

    fn starting_position() -> Self {
        Self {
            inner: ZobristBoard {
                position: BitBoardSet {
                    pawns: BitBoard(
                        0b_00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000,
                    ),
                    knights: BitBoard(0b_0100000010_00000000_00000000_00000000),
                    bishops: BitBoard(0),
                    rooks: BitBoard(0),
                    queens: BitBoard(0),
                    kings: BitBoard(0),
                    white: BitBoard(0),
                    black: BitBoard(0),
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
        }
    }
}

#[cfg(test)]
mod bitboard_tests {
    use super::*;
    #[test]
    fn from_bytes_white_squares() {
        assert_eq!(
            0b_10101010_01010101_10101010_01010101_10101010_01010101_10101010_01010101,
            BitBoard::from_bytes([
                0b_10101010,
                0b_01010101,
                0b_10101010,
                0b_01010101,
                0b_10101010,
                0b_01010101,
                0b_10101010,
                0b_01010101,
            ])
            .0
        );
    }

    #[test]
    fn from_bytes_black_squares() {
        assert_eq!(
            0b_01010101_10101010_01010101_10101010_01010101_10101010_01010101_10101010,
            BitBoard::from_bytes([
                0b_01010101,
                0b_10101010,
                0b_01010101,
                0b_10101010,
                0b_01010101,
                0b_10101010,
                0b_01010101,
                0b_10101010,
            ])
            .0
        );
    }

    #[test]
    fn from_bytes_e4() {
        assert_eq!(
            1 << 36,
            BitBoard::from_bytes([
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00001000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
            ])
            .0
        );
    }
}
