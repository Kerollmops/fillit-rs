use crate::Piece;

pub const VERTICAL_BAR: Piece = Piece { parts: [
    0b1000_0000_0000_0000u16,
    0b1000_0000_0000_0000u16,
    0b1000_0000_0000_0000u16,
    0b1000_0000_0000_0000u16,
] };

pub const HORIZONTAL_BAR: Piece = Piece { parts: [
    0b1111_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const SQUARE: Piece = Piece { parts: [
    0b1100_0000_0000_0000u16,
    0b1100_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const NORMAL_L: Piece = Piece { parts: [
    0b1000_0000_0000_0000u16,
    0b1000_0000_0000_0000u16,
    0b1100_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const NORMAL_L_ROTATE90: Piece = Piece { parts: [
    0b0010_0000_0000_0000u16,
    0b1110_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const NORMAL_L_ROTATE180: Piece = Piece { parts: [
    0b1100_0000_0000_0000u16,
    0b0100_0000_0000_0000u16,
    0b0100_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const NORMAL_L_ROTATE270: Piece = Piece { parts: [
    0b1110_0000_0000_0000u16,
    0b1000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const MIRROR_L: Piece = Piece { parts: [
    0b0100_0000_0000_0000u16,
    0b0100_0000_0000_0000u16,
    0b1100_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const MIRROR_L_ROTATE90: Piece = Piece { parts: [
    0b1110_0000_0000_0000u16,
    0b0010_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const MIRROR_L_ROTATE180: Piece = Piece { parts: [
    0b1100_0000_0000_0000u16,
    0b1000_0000_0000_0000u16,
    0b1000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const MIRROR_L_ROTATE270: Piece = Piece { parts: [
    0b1000_0000_0000_0000u16,
    0b1110_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const NORMAL_STAIRS: Piece = Piece { parts: [
    0b0110_0000_0000_0000u16,
    0b1100_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const NORMAL_STAIRS_ROTATE90: Piece = Piece { parts: [
    0b1000_0000_0000_0000u16,
    0b1100_0000_0000_0000u16,
    0b0100_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const MIRROR_STAIRS: Piece = Piece { parts: [
    0b1100_0000_0000_0000u16,
    0b0110_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const MIRROR_STAIRS_ROTATE90: Piece = Piece { parts: [
    0b0100_0000_0000_0000u16,
    0b1100_0000_0000_0000u16,
    0b1000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const PODIUM: Piece = Piece { parts: [
    0b0100_0000_0000_0000u16,
    0b1110_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const PODIUM_ROTATE90: Piece = Piece { parts: [
    0b0100_0000_0000_0000u16,
    0b1100_0000_0000_0000u16,
    0b0100_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const PODIUM_ROTATE180: Piece = Piece { parts: [
    0b1110_0000_0000_0000u16,
    0b0100_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };

pub const PODIUM_ROTATE270: Piece = Piece { parts: [
    0b1000_0000_0000_0000u16,
    0b1100_0000_0000_0000u16,
    0b1000_0000_0000_0000u16,
    0b0000_0000_0000_0000u16,
] };