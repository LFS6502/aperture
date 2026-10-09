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

use colored::Colorize;
use std::{fmt::Debug, fmt::Write, path::Display};

#[derive(Clone, Copy, PartialEq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub(crate) fn square(&self, square: u32) -> bool {
        (self.0 & 1_u64 << square) > 0
    }
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..8).rev() {
            for column in (0..8) {
                let square = row * 8 + column;

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
        }
        for square in 0..64_u32 {}
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
#[repr(align(64))]
pub struct BitBoardSet {
    pub(crate) pawns: BitBoard,
    pub(crate) knights: BitBoard,
    pub(crate) bishops: BitBoard,
    pub(crate) rooks: BitBoard,
    pub(crate) queens: BitBoard,
    pub(crate) kings: BitBoard,
    pub(crate) white: BitBoard,
    pub(crate) black: BitBoard,
}

pub(crate) fn display_two_bitboards(
    f: &mut std::fmt::Formatter<'_>,
    left_name: &str,
    left: BitBoard,
    right_name: &str,
    right: BitBoard,
) {
    writeln!(f, "{left_name:<24}      {right_name}");
    let left_display = format!("{:?}", left);
    let right_display = format!("{:?}", right);

    for (left_line, right_line) in left_display.lines().zip(right_display.lines()) {
        writeln!(f, "{0}      {1}", left_line, right_line);
    }
}

impl Debug for BitBoardSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_two_bitboards(f, "pawns", self.pawns, "knights", self.knights);
        display_two_bitboards(f, "bishops", self.bishops, "rooks", self.rooks);
        display_two_bitboards(f, "queens", self.queens, "kings", self.kings);
        display_two_bitboards(f, "white", self.white, "black", self.black);
        Ok(())
    }
}
