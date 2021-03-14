use std::fmt;

use crate::{Position, Tetrimino};

pub struct Playground {
    /// The farthest position for a given piece type.
    pub far: [Position; Tetrimino::variant_count()],
    pub buff: [u16; 16],
    pub size: usize,
}

fn minimum_sandbox(nb_tetriminos: usize) -> usize {
    let sqrt_n_x_4 = [0, 2, 3, 4, 4, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 8,
                      9, 9, 9, 9, 10, 10, 10, 10, 10, 11];
    sqrt_n_x_4.get(nb_tetriminos).copied().unwrap_or(11)
}

impl Playground {
    pub fn from_number_tetriminos(count: usize) -> Playground {
        let size = minimum_sandbox(count);
        Playground::from_size(size)
    }

    pub fn from_size(size: usize) -> Playground {
        assert!(size <= 16 * 16);

        let mut sandbox = Playground {
            far: Default::default(),
            buff: [u16::max_value(); 16],
            size,
        };
        sandbox.generate_fences();
        sandbox
    }

    pub fn size(&self) -> usize {
        self.size
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
