type Rgba = [u8; 4];

pub const BLACK: Rgba = [0, 0, 0, 255];
const RED: Rgba = [176, 0, 0, 255];
const GREEN: Rgba = [0, 176, 0, 255];
const YELLOW: Rgba = [176, 88, 0, 255];
const BLUE: Rgba = [0, 0, 176, 255];
const MAGENTA: Rgba = [176, 0, 176, 255];
const CYAN: Rgba = [0, 176, 176, 255];
const WHITE: Rgba = [176, 176, 176, 255];
const BRIGHT_BLACK: Rgba = [88, 88, 88, 255];
const BRIGHT_RED: Rgba = [255, 88, 88, 255];
const BRIGHT_GREEN: Rgba = [88, 255, 88, 255];
const BRIGHT_YELLOW: Rgba = [255, 255, 88, 255];
const BRIGHT_BLUE: Rgba = [88, 88, 255, 255];
const BRIGHT_MAGENTA: Rgba = [255, 88, 255, 255];
const BRIGHT_CYAN: Rgba = [88, 255, 255, 255];
const BRIGHT_WHITE: Rgba = [255, 255, 255, 255];

pub const EGA_PALETTE: [Rgba; 16] = [
    BLACK,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE,
    BRIGHT_BLACK,
    BRIGHT_RED,
    BRIGHT_GREEN,
    BRIGHT_YELLOW,
    BRIGHT_BLUE,
    BRIGHT_MAGENTA,
    BRIGHT_CYAN,
    BRIGHT_WHITE,
];
