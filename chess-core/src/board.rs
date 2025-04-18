#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Cell {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

pub type Bitboard = u64;

pub struct Consts;

impl Consts {
    pub const PIECE_COUNT: usize = 6;
    pub const SIDE_COUNT: usize = 2;
    pub const CELLS_COUNT: usize = 64;
}

pub struct Board {
    pub pieces: [[Bitboard; Consts::PIECE_COUNT]; Consts::SIDE_COUNT],
    pub side: [Bitboard; Consts::SIDE_COUNT],
    pub piece_list: [Piece; Consts::CELLS_COUNT],
}

impl Board {
    pub fn get_pieces(&self, side: Side, piece: Piece) -> Bitboard {
        self.pieces[side as usize][piece as usize]
    }
}
