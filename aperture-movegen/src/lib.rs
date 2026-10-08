#![allow(unused)]

use colored::Colorize;
use std::{fmt::Debug, fmt::Write, path::Display};
mod fen;

pub struct Move {
    from: u8,
    to: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[derive(Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

//Bitboard representation
//
// 8   56 57 58 59 60 61 62 63
// 7   48 49 50 51 52 53 54 55
// 6   40 41 42 43 44 45 46 47
// 5   32 33 34 35 36 37 38 39
// 4   24 25 26 27 28 29 30 31
// 3   16 17 18 19 20 21 22 23
// 2    8  9 10 11 12 13 14 15
// 1    0  1  2  3  4  5  6  7
//
//     A  B  C  D  E  F  G  H

#[derive(Clone, Copy, PartialEq)]
pub struct BitBoard(u64);

impl BitBoard {
    fn square(&self, square: u32) -> bool {
        (self.0 & 1_u64 << square) > 0
    }
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for square in 0..64_u32 {
            let colored_text = if self.square(square) { " @ " } else { "   " }
                .truecolor(0, 255, 0)
                .bold();
            let full_text = if (square + square / 8).rem_euclid(2) == 0 {
                colored_text.on_truecolor(130, 130, 130)
            } else {
                colored_text.on_truecolor(70, 70, 70)
            };
            write!(f, "{full_text}");
            if square.rem_euclid(8) == 7 {
                writeln!(f);
            }
        }
        Ok(())
    }
}

impl From<[u8; 8]> for BitBoard {
    fn from(bytes: [u8; 8]) -> Self {
        Self(u64::from_le_bytes(bytes).reverse_bits())
    }
}

impl From<u64> for BitBoard {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct BitBoardSet {
    pawns: BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    rooks: BitBoard,
    queens: BitBoard,
    kings: BitBoard,
    white: BitBoard,
    black: BitBoard,
}

fn display_two_bitboards(left_name: &str, left: BitBoard, right_name: &str, right: BitBoard) {
    let mut buffer = String::new();

    writeln!(buffer, "{left_name:<24}      {right_name}");
    let left_display = format!("{:?}", left);
    let right_display = format!("{:?}", right);

    for (left_line, right_line) in left_display.lines().zip(right_display.lines()) {
        writeln!(buffer, "{0}      {1}", left_line, right_line);
    }

    println!("{}", buffer);
}

impl Debug for BitBoardSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_two_bitboards("pawns", self.pawns, "knights", self.knights);
        display_two_bitboards("bishops", self.bishops, "rooks", self.rooks);
        display_two_bitboards("queens", self.queens, "kings", self.kings);
        display_two_bitboards("white", self.white, "black", self.black);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CastleRights {
    white_short: bool,
    white_long: bool,
    black_short: bool,
    black_long: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZobristBoard {
    position: BitBoardSet,
    side_to_move: Color,
    castle_rights: CastleRights,
    en_passant: Option<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
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

    // fn starting_position() -> Self {
    //     Self {
    //         inner: ZobristBoard {
    //             position: BitBoardSet {
    //                 pawns: BitBoard(
    //                     0b_00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000,
    //                 ),
    //                 knights: BitBoard(0,
    //                 bishops: BitBoard(0),
    //                 rooks: BitBoard(0),
    //                 queens: BitBoard(0),
    //                 kings: BitBoard(0),
    //                 white: BitBoard(0),
    //                 black: BitBoard(0),
    //             },
    //             side_to_move: Color::White,
    //             castle_rights: CastleRights {
    //                 white_short: true,
    //                 white_long: true,
    //                 black_short: true,
    //                 black_long: true,
    //             },
    //             en_passant: None,
    //         },
    //         fullmoves_clock: 1,
    //         halfmove_clock: 0,
    //     }
    // }
}

#[cfg(test)]
mod bitboard_tests {
    use super::*;

    #[test]
    fn from_bytes_white_squares() {
        assert_eq!(
            0b_01010101_10101010_01010101_10101010_01010101_10101010_01010101_10101010,
            BitBoard::from([
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
            0b_10101010_01010101_10101010_01010101_10101010_01010101_10101010_01010101,
            BitBoard::from([
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
            1 << 28,
            BitBoard::from([
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

    #[test]
    fn from_bytes_h1() {
        assert_eq!(
            1 << 7,
            BitBoard::from([
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000001,
            ])
            .0
        );
    }

    #[test]
    fn from_bytes_a8() {
        assert_eq!(
            1 << 56,
            BitBoard::from([
                0b_10000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
            ])
            .0
        );
    }
}
