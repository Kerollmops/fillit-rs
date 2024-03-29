use std::fmt;

use crate::{Piece, Position, Tetrimino};

#[derive(Clone)]
pub struct Playground {
    buff: [u16; 16],
    size: usize,
}

fn minimum_sandbox(nb_tetriminos: usize) -> usize {
    (nb_tetriminos as f64 * Tetrimino::TILE_COUNT as f64).sqrt().ceil() as usize
}

impl Playground {
    pub fn from_number_tetriminos(count: usize) -> Playground {
        let size = minimum_sandbox(count);
        Playground::from_size(size)
    }

    pub fn from_size(size: usize) -> Playground {
        assert!(size <= 16);

        let mut sandbox = Playground { buff: [u16::max_value(); 16], size };
        sandbox.generate_fences();
        sandbox
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline(always)]
    pub fn can_write_piece(&self, mut piece: Piece, pos: &Position) -> bool {
        piece.shift_right(pos.col);
        unsafe {
            (piece.parts[0] & self.buff[pos.row + 0]) == 0
                && (piece.parts[1] & self.buff[pos.row + 1]) == 0
                && (piece.parts[2] & self.buff[pos.row + 2]) == 0
                && (piece.parts[3] & self.buff[pos.row + 3]) == 0
        }
    }

    #[inline(always)]
    pub fn xor_piece(&mut self, mut piece: Piece, pos: &Position) {
        piece.shift_right(pos.col);
        unsafe {
            self.buff[pos.row + 0] ^= piece.parts[0];
            self.buff[pos.row + 1] ^= piece.parts[1];
            self.buff[pos.row + 2] ^= piece.parts[2];
            self.buff[pos.row + 3] ^= piece.parts[3];
        }
    }

    fn generate_fences(&mut self) {
        self.buff.fill(u16::max_value());
        for line in self.buff.iter_mut().take(self.size) {
            *line >>= self.size;
        }
    }
}

impl fmt::Debug for Playground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.buff {
            writeln!(f, "{:016b}", line)?;
        }
        Ok(())
    }
}
