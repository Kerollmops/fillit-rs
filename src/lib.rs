use anyhow::{ensure, Context};

mod tetrimino;

use self::tetrimino::Tetrimino;

pub fn parse_tetriminos(text: &str) -> anyhow::Result<Vec<Tetrimino>> {
    let tetriminos: anyhow::Result<Vec<_>> = text.split("\n\n").enumerate().map(|(i, block)| {
        Tetrimino::from_text(block, '.', '#').with_context(|| format!("number {}", i))
    }).collect();

    let tetriminos = tetriminos?;
    ensure!(tetriminos.len() <= 26, "too much tetriminos (max is 26)");
    Ok(tetriminos)
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
