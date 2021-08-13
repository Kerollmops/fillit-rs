use std::fmt::Write;
use std::{fmt, str};

use crate::{Position, Tetrimino};

pub struct VisualMap {
    tetriminos: Vec<(Tetrimino, Position)>,
    size: usize,
}

impl VisualMap {
    pub fn new(tetriminos: Vec<(Tetrimino, Position)>, size: usize) -> VisualMap {
        VisualMap { tetriminos, size }
    }
}

impl fmt::Display for VisualMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = vec![b'.'; self.size * self.size];

        for ((t, p), c) in self.tetriminos.iter().zip(b'A'..) {
            let tetrimino_map = t.boolean_map();
            for (line, tline) in map.chunks_mut(self.size).skip(p.row).zip(&tetrimino_map) {
                for (tile, full) in line.iter_mut().skip(p.col).zip(tline) {
                    if *full {
                        *tile = c
                    }
                }
            }
        }

        for line in map.chunks(self.size) {
            let line = str::from_utf8(line).unwrap();
            f.write_str(line)?;
            f.write_char('\n')?;
        }

        Ok(())
    }
}
