#![allow(unused)]

use colored::Colorize;
use std::{fmt::Debug, fmt::Write, path::Display};
mod fen;

mod bitboard;
pub use bitboard::*;

mod square;
pub use square::*;

mod board;
pub use board::*;

#[cfg(test)]
mod tests;

pub struct Move {
    from: Square,
    to: Square,
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
