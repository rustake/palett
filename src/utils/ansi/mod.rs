pub mod color_modes;
pub mod console_colors;
pub mod control_codes;
pub mod effects;

pub use color_modes::{BACK, CLR_BACK, CLR_FORE, FORE};
pub use console_colors::{BLACK, BLUE, CYAN, GREEN, GREY, MAGENTA, RED, WHITE, YELLOW};
pub use control_codes::{CLR_ALL, ESC, L, R, SC};
pub use effects::{BOLD, CLR_BOLD, CLR_INVERSE, CLR_ITALIC, CLR_UNDERLINE, INVERSE, ITALIC, UNDERLINE,
                  effect_to_ansi};
