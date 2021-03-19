use std::str;

use anyhow::{anyhow, ensure, Context};
use enum_ordinalize::Ordinalize;

use crate::boolean_maps::*;
use crate::{Position, Piece};
use Tetrimino::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ordinalize)]
#[repr(usize)]
pub enum Tetrimino {
    /// ```text
    /// #...
    /// #...
    /// #...
    /// #...
    /// ```
    VerticalBar,

    /// ```text
    /// ####
    /// ....
    /// ....
    /// ....
    /// ```
    HorizontalBar,

    /// ```text
    /// ##..
    /// ##..
    /// ....
    /// ....
    /// ```
    Square,

    /// ```text
    /// #...
    /// #...
    /// ##..
    /// ....
    /// ```
    NormalL,

    /// ```text
    /// ..#.
    /// ###.
    /// ....
    /// ....
    /// ```
    NormalLRotate90,

    /// ```text
    /// ##..
    /// .#..
    /// .#..
    /// ....
    /// ```
    NormalLRotate180,

    /// ```text
    /// ###.
    /// #...
    /// ....
    /// ....
    /// ```
    NormalLRotate270,

    /// ```text
    /// .#..
    /// .#..
    /// ##..
    /// ....
    /// ```
    MirrorL,

    /// ```text
    /// ###.
    /// ..#.
    /// ....
    /// ....
    /// ```
    MirrorLRotate90,

    /// ```text
    /// ##..
    /// #...
    /// #...
    /// ....
    /// ```
    MirrorLRotate180,

    /// ```text
    /// #...
    /// ###.
    /// ....
    /// ....
    /// ```
    MirrorLRotate270,

    /// ```text
    /// .##.
    /// ##..
    /// ....
    /// ....
    /// ```
    NormalStairs,

    /// ```text
    /// #...
    /// ##..
    /// .#..
    /// ....
    /// ```
    NormalStairsRotate90,

    /// ```text
    /// ##..
    /// .##.
    /// ....
    /// ....
    /// ```
    MirrorStairs,

    /// ```text
    /// .#..
    /// ##..
    /// #...
    /// ....
    /// ```
    MirrorStairsRotate90,

    /// ```text
    /// .#..
    /// ###.
    /// ....
    /// ....
    /// ```
    Podium,

    /// ```text
    /// .#..
    /// ##..
    /// .#..
    /// ....
    /// ```
    PodiumRotate90,

    /// ```text
    /// ###.
    /// .#..
    /// ....
    /// ....
    /// ```
    PodiumRotate180,

    /// ```text
    /// #...
    /// ##..
    /// #...
    /// ....
    /// ```
    PodiumRotate270,
}

impl Tetrimino {
    pub const TILE_COUNT: usize = 4;

    pub fn from_text(text: &str, empty: char, full: char) -> anyhow::Result<Tetrimino> {
        let mut buffer = [[false; 4]; 4];
        let mut last_y = 0;
        let mut last_x = 0;

        for (y, line) in text.lines().map(str::trim).enumerate() {
            last_y = y;
            for (x, c) in line.chars().enumerate() {
                last_x = x;
                ensure!(c == full || c == empty, "invalid tetrimino character ({})", c);
                ensure!(x < 4, "tetrimino line length is too long");
                ensure!(y < 4, "tetrimino number of lines is too big");
                buffer[y][x] = c == full;
            }
            ensure!(last_x == 3, "tetrimino line length is too short");
        }
        ensure!(last_y == 3, "tetrimino number of lines is too short");

        let left_full = buffer.iter().flat_map(|b| b.iter().position(|c| *c)).min();
        let top_full = buffer.iter().position(|b| b.iter().any(|c| *c));

        match top_full.zip(left_full) {
            Some((y, x)) => {
                buffer.rotate_left(y);
                buffer.iter_mut().for_each(|b| b.rotate_left(x));
                Tetrimino::from_buffer_4x4(buffer).context("invalid tetrimino")
            },
            None => Err(anyhow!("empty tetrimino")),
        }
    }

    fn from_buffer_4x4(buffer: [[bool; 4]; 4]) -> Option<Tetrimino> {
        match buffer {
            VERTICAL_BAR => Some(VerticalBar),
            HORIZONTAL_BAR => Some(HorizontalBar),
            SQUARE => Some(Square),
            NORMAL_L => Some(NormalL),
            NORMAL_L_ROTATE90 => Some(NormalLRotate90),
            NORMAL_L_ROTATE180 => Some(NormalLRotate180),
            NORMAL_L_ROTATE270 => Some(NormalLRotate270),
            MIRROR_L => Some(MirrorL),
            MIRROR_L_ROTATE90 => Some(MirrorLRotate90),
            MIRROR_L_ROTATE180 => Some(MirrorLRotate180),
            MIRROR_L_ROTATE270 => Some(MirrorLRotate270),
            NORMAL_STAIRS => Some(NormalStairs),
            NORMAL_STAIRS_ROTATE90 => Some(NormalStairsRotate90),
            MIRROR_STAIRS => Some(MirrorStairs),
            MIRROR_STAIRS_ROTATE90 => Some(MirrorStairsRotate90),
            PODIUM => Some(Podium),
            PODIUM_ROTATE90 => Some(PodiumRotate90),
            PODIUM_ROTATE180 => Some(PodiumRotate180),
            PODIUM_ROTATE270 => Some(PodiumRotate270),
            _otherwise => None,
        }
    }

    pub const fn boolean_map(&self) -> [[bool; 4]; 4] {
        match self {
            VerticalBar => VERTICAL_BAR,
            HorizontalBar => HORIZONTAL_BAR,
            Square => SQUARE,
            NormalL => NORMAL_L,
            NormalLRotate90 => NORMAL_L_ROTATE90,
            NormalLRotate180 => NORMAL_L_ROTATE180,
            NormalLRotate270 => NORMAL_L_ROTATE270,
            MirrorL => MIRROR_L,
            MirrorLRotate90 => MIRROR_L_ROTATE90,
            MirrorLRotate180 => MIRROR_L_ROTATE180,
            MirrorLRotate270 => MIRROR_L_ROTATE270,
            NormalStairs => NORMAL_STAIRS,
            NormalStairsRotate90 => NORMAL_STAIRS_ROTATE90,
            MirrorStairs => MIRROR_STAIRS,
            MirrorStairsRotate90 => MIRROR_STAIRS_ROTATE90,
            Podium => PODIUM,
            PodiumRotate90 => PODIUM_ROTATE90,
            PodiumRotate180 => PODIUM_ROTATE180,
            PodiumRotate270 => PODIUM_ROTATE270,
        }
    }

    pub const fn piece(&self) -> Piece {
        const fn create_part(bs: [bool; 4]) -> u16 {
              (bs[0] as u16) << 15
            | (bs[1] as u16) << 14
            | (bs[2] as u16) << 13
            | (bs[3] as u16) << 12
        }

        let booleans = self.boolean_map();
        let mut parts = [0; 4];
        parts[0] = create_part(booleans[0]);
        parts[1] = create_part(booleans[1]);
        parts[2] = create_part(booleans[2]);
        parts[3] = create_part(booleans[3]);
        Piece { parts }
    }

    /// Defines the amount of columns that we can skip based on
    /// the previously tried piece position.
    pub const fn jump_columns(&self) -> usize {
        match self {
            VerticalBar => 1,
            HorizontalBar => 4,
            Square => 2,
            NormalL => 2,
            NormalLRotate90 => 3,
            NormalLRotate180 => 2,
            NormalLRotate270 => 3,
            MirrorL => 2,
            MirrorLRotate90 => 3,
            MirrorLRotate180 => 2,
            MirrorLRotate270 => 3,
            NormalStairs => 2,
            NormalStairsRotate90 => 2,
            MirrorStairs => 2,
            MirrorStairsRotate90 => 2,
            Podium => 3,
            PodiumRotate90 => 2,
            PodiumRotate180 => 3,
            PodiumRotate270 => 2,
        }
    }

    pub const fn size(&self) -> Position {
        match self {
            VerticalBar => Position { col: 1, row: 4 },
            HorizontalBar => Position { col: 4, row: 1 },
            Square => Position { col: 2, row: 2 },
            NormalL => Position { col: 2, row: 3 },
            NormalLRotate90 => Position { col: 3, row: 2 },
            NormalLRotate180 => Position { col: 2, row: 3 },
            NormalLRotate270 => Position { col: 3, row: 2 },
            MirrorL => Position { col: 2, row: 3 },
            MirrorLRotate90 => Position { col: 3, row: 2 },
            MirrorLRotate180 => Position { col: 2, row: 3 },
            MirrorLRotate270 => Position { col: 3, row: 2 },
            NormalStairs => Position { col: 3, row: 2 },
            NormalStairsRotate90 => Position { col: 2, row: 3 },
            MirrorStairs => Position { col: 3, row: 2 },
            MirrorStairsRotate90 => Position { col: 2, row: 3 },
            Podium => Position { col: 3, row: 2 },
            PodiumRotate90 => Position { col: 2, row: 3 },
            PodiumRotate180 => Position { col: 3, row: 2 },
            PodiumRotate270 => Position { col: 2, row: 3 },
        }
    }
}
