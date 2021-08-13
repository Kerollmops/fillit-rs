use crate::{NUMBER_TETRIMINOS, Tetrimino, Position, Piece};

pub struct Tetriminos {
    pub types: [usize; NUMBER_TETRIMINOS],
    pub jump_columns: [usize; NUMBER_TETRIMINOS],
    pub sizes: [Position; NUMBER_TETRIMINOS],
    pub pieces: [Piece; NUMBER_TETRIMINOS],
    pub is_first_occurence: [bool; NUMBER_TETRIMINOS],
    pub is_last_piece_type: [bool; NUMBER_TETRIMINOS],
    pub count: usize,
}

impl Tetriminos {
    pub fn from_tetriminos(tetriminos: &[Tetrimino]) -> Tetriminos {
        let mut pieces = [Piece::uninit(); NUMBER_TETRIMINOS];
        let mut sizes = [Position::default(); NUMBER_TETRIMINOS];
        let mut types = [0; NUMBER_TETRIMINOS];
        let mut jump_columns = [0; NUMBER_TETRIMINOS];
        let mut is_first_occurence = [false; NUMBER_TETRIMINOS];
        let mut is_last_piece_type = [false; NUMBER_TETRIMINOS];

        pieces.iter_mut().zip(tetriminos).for_each(|(p, tet)| *p = tet.piece());
        types.iter_mut().zip(tetriminos).for_each(|(t, tet)| *t = tet.ordinal());
        sizes.iter_mut().zip(tetriminos).for_each(|(s, tet)| *s = tet.size());
        jump_columns.iter_mut().zip(tetriminos).for_each(|(j, tet)| *j = tet.jump_columns());

        // Store a boolean that tell for each piece if it's the last occurence of this type.
        is_first_occurence.iter_mut().zip(&types).enumerate().for_each(|(i, (ifo, t))| {
            *ifo = !types[..i].iter().any(|ot| ot == t);
        });

        // Find the last tetriminos type of the list.
        if let Some(idx) = is_first_occurence.iter().rposition(|x| *x) {
            is_last_piece_type[idx] = true;
        }

        Tetriminos {
            types,
            jump_columns,
            sizes,
            pieces,
            is_first_occurence,
            is_last_piece_type,
            count: tetriminos.len(),
        }
    }
}
