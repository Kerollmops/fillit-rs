use std::fmt::Write;
use std::{fmt, str};

use anyhow::{ensure, Context};

mod boolean_maps;
mod piece;
mod playground;
mod tetrimino;

pub use self::piece::Piece;
pub use self::playground::Playground;
pub use self::tetrimino::Tetrimino;

use BacktrackResult::*;

pub fn parse_tetriminos(text: &str) -> anyhow::Result<Vec<Tetrimino>> {
    let tetriminos: anyhow::Result<Vec<_>> = text.split("\n\n").enumerate().map(|(i, block)| {
        Tetrimino::from_text(block, '.', '#').with_context(|| format!("number {}", i))
    }).collect();

    let tetriminos = tetriminos?;
    ensure!(tetriminos.len() <= 26, "too much tetriminos (max is 26)");
    Ok(tetriminos)
}

struct Tetriminos {
    types: [usize; 26],
    jump_columns: [usize; 26],
    sizes: [Position; 26],
    pieces: [Piece; 26],
    is_first_occurence: [bool; 26],
    is_last_piece_type: [bool; 26],
    count: usize,
}

impl Tetriminos {
    fn from_tetriminos(tetriminos: &[Tetrimino]) -> Tetriminos {
        let mut pieces = [Piece::uninit(); 26];
        let mut sizes = [Position::default(); 26];
        let mut types = [0; 26];
        let mut jump_columns = [0; 26];
        let mut is_first_occurence = [false; 26];
        let mut is_last_piece_type = [false; 26];

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

#[derive(Debug, Clone, PartialEq, Eq)]
enum BacktrackResult {
    SolutionFound,
    NeedNewMap,
    Continue,
}

fn backtrack(
    tetriminos: &Tetriminos,
    i: usize,
    pg: &mut Playground,
    wastable: usize,
    solution: &mut [Position],
    farthest: &mut [Position],
) -> BacktrackResult
{
    let (solution, tail_solution) = match solution.split_first_mut() {
        Some((first, tail)) => (first, tail),
        None => return BacktrackResult::NeedNewMap,
    };

    let ttype = tetriminos.types[i];
    let tsize = tetriminos.sizes[i];
    let tpiece = tetriminos.pieces[i];
    let is_last_piece_type = tetriminos.is_last_piece_type[i];
    let saved_farthest = farthest[ttype];

    // We use the previously found farthest position for this tetriminos type
    // to start searching for the next position.
    let mut pos = farthest[ttype];

    while pg.size().checked_sub(tsize.row).map_or(false, |s| pos.row <= s) {
        while pg.size().checked_sub(tsize.col).map_or(false, |s| pos.col <= s) {
            // If we waste too much tiles it means that this map is not more solvable.
            if i <= 9 && is_last_piece_type && wasted(tetriminos, pg.size(), farthest) > wastable {
                return NeedNewMap;
            }

            if pg.can_write_piece(tpiece, &pos) {
                pg.xor_piece(tpiece, &pos);

                // We saved the farthest position found for this tetrimino type.
                farthest[ttype] = Position {
                    row: pos.row,
                    col: pos.col + tetriminos.jump_columns[i],
                };

                if i + 1 == tetriminos.count {
                    *solution = pos;
                    return SolutionFound;
                }

                match backtrack(tetriminos, i + 1, pg, wastable, tail_solution, farthest) {
                    NeedNewMap => return NeedNewMap,
                    SolutionFound => {
                        *solution = pos;
                        return SolutionFound;
                    },
                    Continue => (),
                }

                pg.xor_piece(tpiece, &pos);
            }
            pos.col += 1;
        }
        pos.row += 1;
        pos.col = 0;
    }

    // We write back the previously found fartest position for this tetrimino type,
    // as we were not able to find a solution with our best position.
    farthest[ttype] = saved_farthest;

    if i == 0 { NeedNewMap } else { Continue }
}

fn wasted(tetriminos: &Tetriminos, pg_size: usize, farthest: &[Position]) -> usize {
    let pos = farthest.iter().zip(&tetriminos.is_first_occurence)
        .take(tetriminos.count)
        .filter_map(|(far, ifo)| ifo.then(|| *far))
        .min()
        .unwrap_or_else(Position::default);

    pos.row.saturating_sub(1) * pg_size + pos.col
}

fn compute_wastable(pg_size: usize, tetriminos_count: usize) -> usize {
    pg_size * pg_size - tetriminos_count * Tetrimino::TILE_COUNT
}

pub fn find_best_fit(raw_tetriminos: &[Tetrimino]) -> VisualMap {
    let tetriminos_count = raw_tetriminos.len();
    let mut solution = [Position::default(); 26];
    let mut pg = Playground::from_number_tetriminos(tetriminos_count);
    // The farthest position for a given piece type.
    let mut farthest = [Position::default(); Tetrimino::variant_count()];
    let mut wastable = compute_wastable(pg.size(), tetriminos_count);
    let tetriminos = Tetriminos::from_tetriminos(raw_tetriminos);

    eprintln!("Try to fit {} tetriminos in a {} sized map.", tetriminos_count, pg.size());
    while backtrack(&tetriminos, 0, &mut pg, wastable, &mut solution[..tetriminos_count], &mut farthest) == NeedNewMap {
        pg = Playground::from_size(pg.size() + 1);
        wastable = compute_wastable(pg.size(), tetriminos_count);
        farthest.fill_with(Position::default);
        eprintln!("Try to fit {} tetriminos in a {} sized map.", tetriminos_count, pg.size());
    }

    let solution = raw_tetriminos.iter().copied().zip(solution.iter().copied()).collect();
    VisualMap::new(solution, pg.size())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    const ANSWER_1: &str =               include_str!("../tests/answers/valid_1.answer");
    const ANSWER_2: &str =               include_str!("../tests/answers/valid_2.answer");
    const ANSWER_3: &str =               include_str!("../tests/answers/valid_3.answer");
    const ANSWER_4: &str =               include_str!("../tests/answers/valid_4.answer");
    const ANSWER_EACH_PIECE: &str =      include_str!("../tests/answers/valid_each_piece.answer");
    const ANSWER_HARD: &str =            include_str!("../tests/answers/valid_hard.answer");
    const ANSWER_HARD_FORUM: &str =      include_str!("../tests/answers/valid_hard_forum.answer");
    const ANSWER_HARD_FORUM_10: &str =   include_str!("../tests/answers/valid_hard_forum_10.answer");
    const ANSWER_HARD_FORUM_11: &str =   include_str!("../tests/answers/valid_hard_forum_11.answer");
    const ANSWER_HARD_FORUM_12: &str =   include_str!("../tests/answers/valid_hard_forum_12.answer");
    const ANSWER_HARD_FORUM_13: &str =   include_str!("../tests/answers/valid_hard_forum_13.answer");
    const ANSWER_HARD_FORUM_14: &str =   include_str!("../tests/answers/valid_hard_forum_14.answer");
    const ANSWER_HARD_FORUM_15: &str =   include_str!("../tests/answers/valid_hard_forum_15.answer");
    const ANSWER_HARD_FORUM_16: &str =   include_str!("../tests/answers/valid_hard_forum_16.answer");
    const ANSWER_HARD_FORUM_17: &str =   include_str!("../tests/answers/valid_hard_forum_17.answer");
    const ANSWER_HARD_FORUM_18: &str =   include_str!("../tests/answers/valid_hard_forum_18.answer");
    const ANSWER_HARD_FORUM_19: &str =   include_str!("../tests/answers/valid_hard_forum_19.answer");
    const ANSWER_HARD_FORUM_20: &str =   include_str!("../tests/answers/valid_hard_forum_20.answer");
    const ANSWER_HARD_FORUM_21: &str =   include_str!("../tests/answers/valid_hard_forum_21.answer");
    const ANSWER_HARD_FORUM_22: &str =   include_str!("../tests/answers/valid_hard_forum_22.answer");
    const ANSWER_HARD_FORUM_23: &str =   include_str!("../tests/answers/valid_hard_forum_23.answer");
    const ANSWER_I_16: &str =            include_str!("../tests/answers/valid_I_16.answer");
    const ANSWER_I_26: &str =            include_str!("../tests/answers/valid_I_26.answer");

    const EASY_MAPS_ANSWERS: &[(&str, &str)] = &[
        (VALID_1, ANSWER_1),
        (VALID_2, ANSWER_2),
        (VALID_3, ANSWER_3),
        (VALID_4, ANSWER_4),
        (VALID_EACH_PIECE, ANSWER_EACH_PIECE),
        (VALID_HARD, ANSWER_HARD),
        (VALID_HARD_FORUM_10, ANSWER_HARD_FORUM_10),
        (VALID_HARD_FORUM_11, ANSWER_HARD_FORUM_11),
        (VALID_HARD_FORUM_12, ANSWER_HARD_FORUM_12),
        (VALID_HARD_FORUM_13, ANSWER_HARD_FORUM_13),
        (VALID_HARD_FORUM_14, ANSWER_HARD_FORUM_14),
        (VALID_HARD_FORUM_15, ANSWER_HARD_FORUM_15),
        (VALID_HARD_FORUM_16, ANSWER_HARD_FORUM_16),
        (VALID_HARD_FORUM_17, ANSWER_HARD_FORUM_17),
        (VALID_HARD_FORUM_21, ANSWER_HARD_FORUM_21),
        (VALID_HARD_FORUM_22, ANSWER_HARD_FORUM_22),
        (VALID_I_16, ANSWER_I_16),
        (VALID_I_26, ANSWER_I_26),
    ];

    const HARD_MAPS_ANSWERS: &[(&str, &str)] = &[
        (VALID_HARD_FORUM, ANSWER_HARD_FORUM),
        (VALID_HARD_FORUM_18, ANSWER_HARD_FORUM_19),
        (VALID_HARD_FORUM_18, ANSWER_HARD_FORUM_18),
        (VALID_HARD_FORUM_20, ANSWER_HARD_FORUM_20),
        (VALID_HARD_FORUM_23, ANSWER_HARD_FORUM_23),
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

    #[test]
    fn valid_easy_answer_maps() {
        for (i, (map, answer)) in EASY_MAPS_ANSWERS.iter().enumerate() {
            eprintln!("testing map #{}", i);
            let tetriminos = parse_tetriminos(map).unwrap();
            let map = find_best_fit(&tetriminos);
            assert_eq!(&map.to_string(), answer);
        }
    }

    #[test]
    #[ignore]
    fn valid_hard_answer_maps() {
        for (i, (map, answer)) in HARD_MAPS_ANSWERS.iter().enumerate() {
            eprintln!("testing map #{}", i);
            let tetriminos = parse_tetriminos(map).unwrap();
            let map = find_best_fit(&tetriminos);
            assert_eq!(&map.to_string(), answer);
        }
    }
}

