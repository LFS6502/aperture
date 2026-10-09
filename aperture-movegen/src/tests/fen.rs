use crate::*;
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
                white_kingside: true,
                white_queenside: true,
                black_kingside: true,
                black_queenside: true,
            },
            en_passant: None,
            zobrist_hash: 0,
        },
        fullmove_number: 1,
        halfmove_clock: 0,
        history: Vec::new(),
    };

    assert_eq!(Board::from_fen(starting_fen).unwrap(), starting_board);
}
