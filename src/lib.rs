use std::fmt::Write;
use std::{fmt, str};

use anyhow::{ensure, Context};

mod boolean_maps;
mod pieces;
mod tetrimino;

pub use self::tetrimino::{Tetrimino, Piece};
use BacktrackResult::*;

pub fn parse_tetriminos(text: &str) -> anyhow::Result<Vec<Tetrimino>> {
    let tetriminos: anyhow::Result<Vec<_>> = text.split("\n\n").enumerate().map(|(i, block)| {
        Tetrimino::from_text(block, '.', '#').with_context(|| format!("number {}", i))
    }).collect();

    let tetriminos = tetriminos?;
    ensure!(tetriminos.len() <= 26, "too much tetriminos (max is 26)");
    Ok(tetriminos)
}

struct Sandbox {
    /// The farthest position for a given piece type.
    far: [Position; Tetrimino::variant_count()],
    buff: [u16; 16],
    size: usize,
}

impl Sandbox {
    pub fn new(count: usize) -> Option<Sandbox> {
        fn minimum_sandbox(nb_tetriminos: usize) -> Option<usize> {
            let sqrt_n_x_4 = [0, 2, 3, 4, 4, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 8,
                              9, 9, 9, 9, 10, 10, 10, 10, 10, 11];
            sqrt_n_x_4.get(nb_tetriminos).copied()
        }

        let mut sandbox = Sandbox {
            far: Default::default(),
            buff: [u16::max_value(); 16],
            size: minimum_sandbox(count)?,
        };
        sandbox.generate_fences();
        Some(sandbox)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn increase_size(&mut self) -> bool {
        if self.size <= 21 { // TODO ????
            self.size += 1;
            self.far.fill(Position::new(0, 0));
            self.generate_fences();
            true
        } else {
            false
        }
    }

    fn generate_fences(&mut self) {
        self.buff.fill(u16::max_value());
        for line in self.buff.iter_mut().take(self.size) {
            *line >>= self.size;
        }
    }
}

impl fmt::Debug for Sandbox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.buff {
            writeln!(f, "{:016b}", line)?;
        }
        Ok(())
    }
}

struct Tetriminos {
    types: [usize; 26],
    jump_columns: [usize; 26],
    sizes: [Position; 26],
    pieces: [Piece; 26],
    count: usize,
}

impl Tetriminos {
    fn from_tetriminos(tetriminos: &[Tetrimino]) -> Tetriminos {
        let mut pieces = [Piece::uninit(); 26];
        let mut sizes = [Position::default(); 26];
        let mut types = [0; 26];
        let mut jump_columns = [0; 26];

        pieces.iter_mut().zip(tetriminos).for_each(|(p, tet)| *p = tet.piece());
        types.iter_mut().zip(tetriminos).for_each(|(t, tet)| *t = tet.ordinal());
        sizes.iter_mut().zip(tetriminos).for_each(|(s, tet)| *s = tet.size());
        jump_columns.iter_mut().zip(tetriminos).for_each(|(j, tet)| *j = tet.jump_columns());

        Tetriminos {
            types,
            jump_columns,
            sizes,
            pieces,
            count: tetriminos.len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BacktrackResult {
    SolutionFound,
    NeedNewMap,
    Continue,
}

fn can_write_tetriminos(mut piece: Piece, pos: &Position, sandbox: &Sandbox) -> bool {
    piece.shift_right(pos.col);
    unsafe {
           (piece.parts[0] & sandbox.buff[pos.row + 0]) == 0
        && (piece.parts[1] & sandbox.buff[pos.row + 1]) == 0
        && (piece.parts[2] & sandbox.buff[pos.row + 2]) == 0
        && (piece.parts[3] & sandbox.buff[pos.row + 3]) == 0
    }
}

fn xor_piece(mut piece: Piece, pos: &Position, sandbox: &mut Sandbox) {
    piece.shift_right(pos.col);
    unsafe {
        sandbox.buff[pos.row + 0] ^= piece.parts[0];
        sandbox.buff[pos.row + 1] ^= piece.parts[1];
        sandbox.buff[pos.row + 2] ^= piece.parts[2];
        sandbox.buff[pos.row + 3] ^= piece.parts[3];
    }
}

fn backtrack(
    tetriminos: &Tetriminos,
    i: usize,
    sandbox: &mut Sandbox,
    solution: &mut Vec<Position>,
) -> BacktrackResult
{
    let ttype = tetriminos.types[i];
    let tsize = tetriminos.sizes[i];
    let tpiece = tetriminos.pieces[i];
    let saved_farthest = sandbox.far[ttype];
    let mut pos = sandbox.far[ttype];

    while sandbox.size.checked_sub(tsize.row).map_or(false, |s| pos.row <= s) {
        while sandbox.size.checked_sub(tsize.col).map_or(false, |s| pos.col <= s) {
            if can_write_tetriminos(tpiece, &pos, sandbox) {
                xor_piece(tpiece, &pos, sandbox);
                sandbox.far[ttype].row = pos.row;
                sandbox.far[ttype].col = pos.col + tetriminos.jump_columns[i];
                if i + 1 == tetriminos.count || backtrack(tetriminos, i + 1, sandbox, solution) == SolutionFound {
                    solution.push(pos);
                    return SolutionFound;
                }
                xor_piece(tpiece, &pos, sandbox);
            }
            pos.col += 1;
        }
        pos.row += 1;
        pos.col = 0;
    }
    sandbox.far[ttype] = saved_farthest;

    if i == 0 { NeedNewMap } else { Continue }
}

pub fn find_best_fit(raw_tetriminos: &[Tetrimino]) -> Option<VisualMap> {
    let mut solution = Vec::with_capacity(raw_tetriminos.len());
    let mut sandbox = Sandbox::new(raw_tetriminos.len())?;
    let tetriminos = Tetriminos::from_tetriminos(raw_tetriminos);

    while backtrack(&tetriminos, 0, &mut sandbox, &mut solution) == NeedNewMap {
        // eprintln!("increase sandbox size to {}", sandbox.size() + 1);
        if !sandbox.increase_size() {
            return None;
        }
    }

    let solution = raw_tetriminos.iter().copied().zip(solution.iter().rev().copied()).collect();
    Some(VisualMap::new(solution, sandbox.size()))
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Position {
    pub fn new(col: usize, row: usize) -> Position {
        Position { col, row }
    }
}

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
                    if *full { *tile = c }
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

#[cfg(test)]
mod tests {
    use super::*;

    const NOT_VALID_0: &str =           include_str!("../tests/not_valid_0.fillit");
    const NOT_VALID_BAD_ENDLINE: &str = include_str!("../tests/not_valid_bad_endline.fillit");
    const NOT_VALID_BAD_PIECES: &str =  include_str!("../tests/not_valid_bad_pieces.fillit");
    const NOT_VALID_COMMA: &str =       include_str!("../tests/not_valid_comma.fillit");
    const NOT_VALID_EMPTY: &str =       include_str!("../tests/not_valid_empty.fillit");
    const NOT_VALID_ENDLINE: &str =     include_str!("../tests/not_valid_endline.fillit");
    const NOT_VALID_LINE_LEN: &str =    include_str!("../tests/not_valid_line_len.fillit");
    const NOT_VALID_TOO_BIG_27: &str =  include_str!("../tests/not_valid_too_big_27.fillit");
    const NOT_VALID_X: &str =           include_str!("../tests/not_valid_X.fillit");
    const NOT_VALIDS: &[&str] = &[
        NOT_VALID_0,
        NOT_VALID_BAD_ENDLINE,
        NOT_VALID_BAD_PIECES,
        NOT_VALID_COMMA,
        NOT_VALID_EMPTY,
        NOT_VALID_ENDLINE,
        NOT_VALID_LINE_LEN,
        NOT_VALID_TOO_BIG_27,
        NOT_VALID_X,
    ];

    const VALID_1: &str =               include_str!("../tests/valid_1.fillit");
    const VALID_2: &str =               include_str!("../tests/valid_2.fillit");
    const VALID_3: &str =               include_str!("../tests/valid_3.fillit");
    const VALID_4: &str =               include_str!("../tests/valid_4.fillit");
    const VALID_EACH_PIECE: &str =      include_str!("../tests/valid_each_piece.fillit");
    const VALID_HARD: &str =            include_str!("../tests/valid_hard.fillit");
    const VALID_HARD_FORUM: &str =      include_str!("../tests/valid_hard_forum.fillit");
    const VALID_HARD_FORUM_10: &str =   include_str!("../tests/valid_hard_forum_10.fillit");
    const VALID_HARD_FORUM_11: &str =   include_str!("../tests/valid_hard_forum_11.fillit");
    const VALID_HARD_FORUM_12: &str =   include_str!("../tests/valid_hard_forum_12.fillit");
    const VALID_HARD_FORUM_13: &str =   include_str!("../tests/valid_hard_forum_13.fillit");
    const VALID_HARD_FORUM_14: &str =   include_str!("../tests/valid_hard_forum_14.fillit");
    const VALID_HARD_FORUM_15: &str =   include_str!("../tests/valid_hard_forum_15.fillit");
    const VALID_HARD_FORUM_16: &str =   include_str!("../tests/valid_hard_forum_16.fillit");
    const VALID_HARD_FORUM_17: &str =   include_str!("../tests/valid_hard_forum_17.fillit");
    const VALID_HARD_FORUM_18: &str =   include_str!("../tests/valid_hard_forum_18.fillit");
    const VALID_HARD_FORUM_19: &str =   include_str!("../tests/valid_hard_forum_19.fillit");
    const VALID_HARD_FORUM_20: &str =   include_str!("../tests/valid_hard_forum_20.fillit");
    const VALID_HARD_FORUM_21: &str =   include_str!("../tests/valid_hard_forum_21.fillit");
    const VALID_HARD_FORUM_22: &str =   include_str!("../tests/valid_hard_forum_22.fillit");
    const VALID_HARD_FORUM_23: &str =   include_str!("../tests/valid_hard_forum_23.fillit");
    const VALID_I_16: &str =            include_str!("../tests/valid_I_16.fillit");
    const VALID_I_25: &str =            include_str!("../tests/valid_I_25.fillit");
    const VALID_I_26: &str =            include_str!("../tests/valid_I_26.fillit");
    const VALIDS: &[&str] = &[
        VALID_1,
        VALID_2,
        VALID_3,
        VALID_4,
        VALID_EACH_PIECE,
        VALID_HARD,
        VALID_HARD_FORUM,
        VALID_HARD_FORUM_10,
        VALID_HARD_FORUM_11,
        VALID_HARD_FORUM_12,
        VALID_HARD_FORUM_13,
        VALID_HARD_FORUM_14,
        VALID_HARD_FORUM_15,
        VALID_HARD_FORUM_16,
        VALID_HARD_FORUM_17,
        VALID_HARD_FORUM_18,
        VALID_HARD_FORUM_19,
        VALID_HARD_FORUM_20,
        VALID_HARD_FORUM_21,
        VALID_HARD_FORUM_22,
        VALID_HARD_FORUM_23,
        VALID_I_16,
        VALID_I_25,
        VALID_I_26,
    ];

    #[test]
    fn valid_maps() {
        for (i, map) in VALIDS.iter().enumerate() {
            eprintln!("testing map #{}", i);
            parse_tetriminos(map).unwrap();
        }
    }

    #[test]
    fn invalid_maps() {
        for (i, map) in NOT_VALIDS.iter().enumerate() {
            eprintln!("testing map #{}", i);
            parse_tetriminos(map).unwrap_err();
        }
    }
}

