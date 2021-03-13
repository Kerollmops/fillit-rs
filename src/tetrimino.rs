use std::str;

use anyhow::{anyhow, ensure, Context};
use Tetrimino::*;

const VERTICAL_BAR: [[bool; 4]; 4] = [
    [true,  false, false, false],
    [true,  false, false, false],
    [true,  false, false, false],
    [true,  false, false, false]
];
const HORIZONTAL_BAR: [[bool; 4]; 4] = [
    [true,  true,  true,  true],
    [false, false, false, false],
    [false, false, false, false],
    [false, false, false, false],
];
const SQUARE: [[bool; 4]; 4] = [
    [true,  true,  false, false],
    [true,  true,  false, false],
    [false, false, false, false],
    [false, false, false, false],
];
const NORMAL_L: [[bool; 4]; 4] = [
    [true,  false, false, false],
    [true,  false, false, false],
    [true,  true,  false, false],
    [false, false, false, false],
];
const NORMAL_L_ROTATE90: [[bool; 4]; 4] = [
    [false, false, true,  false],
    [true,  true,  true,  false],
    [false, false, false, false],
    [false, false, false, false],
];
const NORMAL_L_ROTATE180: [[bool; 4]; 4] = [
    [true,  true,  false, false],
    [false, true,  false, false],
    [false, true,  false, false],
    [false, false, false, false],
];
const NORMAL_L_ROTATE270: [[bool; 4]; 4] = [
    [true,  true,  true,  false],
    [true,  false, false, false],
    [false, false, false, false],
    [false, false, false, false],
];
const MIRROR_L: [[bool; 4]; 4] = [
    [false, true,  false, false],
    [false, true,  false, false],
    [true,  true,  false, false],
    [false, false, false, false],
];
const MIRROR_L_ROTATE90: [[bool; 4]; 4] = [
    [true,  true,  true,  false],
    [false, false, true,  false],
    [false, false, false, false],
    [false, false, false, false],
];
const MIRROR_L_ROTATE180: [[bool; 4]; 4] = [
    [true,  true,  false, false],
    [true,  false, false, false],
    [true,  false, false, false],
    [false, false, false, false],
];
const MIRROR_L_ROTATE270: [[bool; 4]; 4] = [
    [true,  false, false, false],
    [true,  true,  true,  false],
    [false, false, false, false],
    [false, false, false, false],
];
const NORMAL_STAIRS: [[bool; 4]; 4] = [
    [false, true,  true,  false],
    [true,  true,  false, false],
    [false, false, false, false],
    [false, false, false, false],
];
const NORMAL_STAIRS_ROTATE90: [[bool; 4]; 4] = [
    [true,  false, false, false],
    [true,  true,  false, false],
    [false, true,  false, false],
    [false, false, false, false],
];
const MIRROR_STAIRS: [[bool; 4]; 4] = [
    [true,  true,  false, false],
    [false, true,  true,  false],
    [false, false, false, false],
    [false, false, false, false],
];
const MIRROR_STAIRS_ROTATE90: [[bool; 4]; 4] = [
    [false, true,  false, false],
    [true,  true,  false, false],
    [true,  false, false, false],
    [false, false, false, false],
];
const PODIUM: [[bool; 4]; 4] = [
    [false, true,  false, false],
    [true,  true,  true,  false],
    [false, false, false, false],
    [false, false, false, false],
];
const PODIUM_ROTATE90: [[bool; 4]; 4] = [
    [false, true,  false, false],
    [true,  true,  false, false],
    [false, true,  false, false],
    [false, false, false, false],
];
const PODIUM_ROTATE180: [[bool; 4]; 4] = [
    [true,  true,  true,  false],
    [false, true,  false, false],
    [false, false, false, false],
    [false, false, false, false],
];
const PODIUM_ROTATE270: [[bool; 4]; 4] = [
    [true,  false, false, false],
    [true,  true,  false, false],
    [true,  false, false, false],
    [false, false, false, false],
];

#[derive(Debug)]
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

    pub fn boolean_map(&self) -> [[bool; 4]; 4] {
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
}
