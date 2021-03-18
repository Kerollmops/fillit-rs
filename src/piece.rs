use std::fmt;

#[derive(Copy, Clone, Eq)]
pub union Piece {
    pub parts: [u16; 4],
    pub full: u64,
}

impl Piece {
    pub const fn uninit() -> Piece {
        Piece { full: 0 }
    }

    #[inline]
    pub fn shift_right(&mut self, shift: usize) {
        unsafe { self.full >>= shift }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Piece) -> bool {
        unsafe { self.full == other.full }
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for part in unsafe { &self.parts } {
            writeln!(f, "{:04b}", part)?;
        }
        Ok(())
    }
}
