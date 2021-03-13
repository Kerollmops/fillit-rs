pub const VERTICAL_BAR: [[bool; 4]; 4] = [
    [true,  false, false, false],
    [true,  false, false, false],
    [true,  false, false, false],
    [true,  false, false, false]
];

pub const HORIZONTAL_BAR: [[bool; 4]; 4] = [
    [true,  true,  true,  true],
    [false, false, false, false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const SQUARE: [[bool; 4]; 4] = [
    [true,  true,  false, false],
    [true,  true,  false, false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const NORMAL_L: [[bool; 4]; 4] = [
    [true,  false, false, false],
    [true,  false, false, false],
    [true,  true,  false, false],
    [false, false, false, false],
];

pub const NORMAL_L_ROTATE90: [[bool; 4]; 4] = [
    [false, false, true,  false],
    [true,  true,  true,  false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const NORMAL_L_ROTATE180: [[bool; 4]; 4] = [
    [true,  true,  false, false],
    [false, true,  false, false],
    [false, true,  false, false],
    [false, false, false, false],
];

pub const NORMAL_L_ROTATE270: [[bool; 4]; 4] = [
    [true,  true,  true,  false],
    [true,  false, false, false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const MIRROR_L: [[bool; 4]; 4] = [
    [false, true,  false, false],
    [false, true,  false, false],
    [true,  true,  false, false],
    [false, false, false, false],
];

pub const MIRROR_L_ROTATE90: [[bool; 4]; 4] = [
    [true,  true,  true,  false],
    [false, false, true,  false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const MIRROR_L_ROTATE180: [[bool; 4]; 4] = [
    [true,  true,  false, false],
    [true,  false, false, false],
    [true,  false, false, false],
    [false, false, false, false],
];

pub const MIRROR_L_ROTATE270: [[bool; 4]; 4] = [
    [true,  false, false, false],
    [true,  true,  true,  false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const NORMAL_STAIRS: [[bool; 4]; 4] = [
    [false, true,  true,  false],
    [true,  true,  false, false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const NORMAL_STAIRS_ROTATE90: [[bool; 4]; 4] = [
    [true,  false, false, false],
    [true,  true,  false, false],
    [false, true,  false, false],
    [false, false, false, false],
];

pub const MIRROR_STAIRS: [[bool; 4]; 4] = [
    [true,  true,  false, false],
    [false, true,  true,  false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const MIRROR_STAIRS_ROTATE90: [[bool; 4]; 4] = [
    [false, true,  false, false],
    [true,  true,  false, false],
    [true,  false, false, false],
    [false, false, false, false],
];

pub const PODIUM: [[bool; 4]; 4] = [
    [false, true,  false, false],
    [true,  true,  true,  false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const PODIUM_ROTATE90: [[bool; 4]; 4] = [
    [false, true,  false, false],
    [true,  true,  false, false],
    [false, true,  false, false],
    [false, false, false, false],
];

pub const PODIUM_ROTATE180: [[bool; 4]; 4] = [
    [true,  true,  true,  false],
    [false, true,  false, false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const PODIUM_ROTATE270: [[bool; 4]; 4] = [
    [true,  false, false, false],
    [true,  true,  false, false],
    [true,  false, false, false],
    [false, false, false, false],
];
