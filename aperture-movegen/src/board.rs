use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CastleRights {
    pub(crate) white_kingside: bool,
    pub(crate) white_queenside: bool,
    pub(crate) black_kingside: bool,
    pub(crate) black_queenside: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZobristBoard {
    pub(crate) position: bitboard::BitBoardSet,
    pub(crate) side_to_move: Color,
    pub(crate) castle_rights: CastleRights,
    pub(crate) en_passant: Option<Square>,
    pub(crate) zobrist_hash: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub(crate) inner: ZobristBoard,
    pub(crate) halfmove_clock: u8,
    pub(crate) fullmove_number: u16,
    pub(crate) history: Vec<u64>, // Zobrist hashes
}

impl Board {
    pub(crate) fn new() -> Self {
        Self {
            inner: ZobristBoard {
                position: bitboard::BitBoardSet {
                    pawns: bitboard::BitBoard(0),
                    knights: bitboard::BitBoard(0),
                    bishops: bitboard::BitBoard(0),
                    rooks: bitboard::BitBoard(0),
                    queens: bitboard::BitBoard(0),
                    kings: bitboard::BitBoard(0),
                    white: bitboard::BitBoard(0),
                    black: bitboard::BitBoard(0),
                },
                side_to_move: Color::White,
                castle_rights: CastleRights {
                    white_kingside: false,
                    white_queenside: false,
                    black_kingside: false,
                    black_queenside: false,
                },
                en_passant: None,
                zobrist_hash: 0,
            },
            fullmove_number: 1,
            halfmove_clock: 0,
            history: Vec::new(),
        }
    }

    fn starting_position() -> Self {
        #[expect(clippy::expect_used)]
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Invalid starting FEN")
    }
}
