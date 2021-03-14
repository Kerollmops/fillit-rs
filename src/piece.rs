#[derive(Copy, Clone)]
pub union Piece {
    pub parts: [u16; 4],
    pub full: u64,
}

impl Piece {
    pub fn uninit() -> Piece {
        Piece { full: 0 }
    }

    #[inline]
    pub fn shift_right(&mut self, shift: usize) {
        unsafe { self.full >>= shift }
    }
}
