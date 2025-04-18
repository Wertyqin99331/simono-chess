use crate::board::{Consts, Piece, Side};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

const RANDOMS_COUNT: u64 = const {
    Consts::PIECE_COUNT * Consts::CELLS_COUNT * Consts::SIDE_COUNT
        + Consts::CASTLING_PERMISSIONS
        + Consts::SIDE_COUNT
        + Consts::CELLS_COUNT
        + 1
};

#[derive(Default)]
pub struct ZobristRandoms([u64; RANDOMS_COUNT]);

impl ZobristRandoms {
    pub fn new() -> Self {
        let mut randoms = ZobristRandoms::default();

        let mut rng = ChaCha20Rng::from_seed(10);

        randoms.0.iter_mut().for_each(|r| {
            *r = rng.next_u64();
        });

        randoms
    }

    pub fn get_key_by_piece(&self, piece: Piece, cell: u64, side: Side) -> u64 {
        self.0[piece as u64 * cell * side as u64]
    }
}
