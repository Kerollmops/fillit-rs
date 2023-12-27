use enum_ordinalize::Ordinalize;

use crate::{Piece, Position, Tetrimino, NUMBER_TETRIMINOS};

pub struct Tetriminos {
    pub types: [usize; NUMBER_TETRIMINOS],
    pub jump_columns: [usize; NUMBER_TETRIMINOS],
    pub sizes: [Position; NUMBER_TETRIMINOS],
    pub pieces: [Piece; NUMBER_TETRIMINOS],
    pub count: usize,
}

impl Tetriminos {
    pub fn from_tetriminos(tetriminos: &[Tetrimino]) -> Tetriminos {
        let mut pieces = [Piece::uninit(); NUMBER_TETRIMINOS];
        let mut sizes = [Position::default(); NUMBER_TETRIMINOS];
        let mut types = [0; NUMBER_TETRIMINOS];
        let mut jump_columns = [0; NUMBER_TETRIMINOS];

        pieces.iter_mut().zip(tetriminos).for_each(|(p, tet)| *p = tet.piece());
        types.iter_mut().zip(tetriminos).for_each(|(t, tet)| *t = tet.ordinal());
        sizes.iter_mut().zip(tetriminos).for_each(|(s, tet)| *s = tet.size());
        jump_columns.iter_mut().zip(tetriminos).for_each(|(j, tet)| *j = tet.jump_columns());

        Tetriminos { types, jump_columns, sizes, pieces, count: tetriminos.len() }
    }
}
