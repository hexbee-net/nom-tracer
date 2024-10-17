//! ANSI escape code constants for terminal text formatting.
//!
//! This module provides a set of constants for ANSI escape codes that can be used
//! to format text output in terminal environments. It includes codes for:
//!
//! - Foreground (text) colors (both normal and bright variants)
//! - Background colors (both normal and bright variants)
//! - Text styles (bold, italic, underline, etc.)
//! - Reset codes to return to default formatting

pub const FG_BLACK: &str = concat!("\x1b[", "30", "m");
pub const FG_RED: &str = concat!("\x1b[", "31", "m");
pub const FG_GREEN: &str = concat!("\x1b[", "32", "m");
pub const FG_YELLOW: &str = concat!("\x1b[", "33", "m");
pub const FG_BLUE: &str = concat!("\x1b[", "34", "m");
pub const FG_MAGENTA: &str = concat!("\x1b[", "35", "m");
pub const FG_CYAN: &str = concat!("\x1b[", "36", "m");
pub const FG_WHITE: &str = concat!("\x1b[", "37", "m");
pub const FG_BRIGHT_BLACK: &str = concat!("\x1b[", "90", "m");
pub const FG_BRIGHT_RED: &str = concat!("\x1b[", "91", "m");
pub const FG_BRIGHT_GREEN: &str = concat!("\x1b[", "92", "m");
pub const FG_BRIGHT_YELLOW: &str = concat!("\x1b[", "93", "m");
pub const FG_BRIGHT_BLUE: &str = concat!("\x1b[", "94", "m");
pub const FG_BRIGHT_MAGENTA: &str = concat!("\x1b[", "95", "m");
pub const FG_BRIGHT_CYAN: &str = concat!("\x1b[", "96", "m");
pub const FG_BRIGHT_WHITE: &str = concat!("\x1b[", "97", "m");

pub const BG_BLACK: &str = concat!("\x1b[", "40", "m");
pub const BG_RED: &str = concat!("\x1b[", "41", "m");
pub const BG_GREEN: &str = concat!("\x1b[", "42", "m");
pub const BG_YELLOW: &str = concat!("\x1b[", "43", "m");
pub const BG_BLUE: &str = concat!("\x1b[", "44", "m");
pub const BG_MAGENTA: &str = concat!("\x1b[", "45", "m");
pub const BG_CYAN: &str = concat!("\x1b[", "46", "m");
pub const BG_WHITE: &str = concat!("\x1b[", "47", "m");
pub const BG_BRIGHT_BLACK: &str = concat!("\x1b[", "100", "m");
pub const BG_BRIGHT_RED: &str = concat!("\x1b[", "101", "m");
pub const BG_BRIGHT_GREEN: &str = concat!("\x1b[", "102", "m");
pub const BG_BRIGHT_YELLOW: &str = concat!("\x1b[", "103", "m");
pub const BG_BRIGHT_BLUE: &str = concat!("\x1b[", "104", "m");
pub const BG_BRIGHT_MAGENTA: &str = concat!("\x1b[", "105", "m");
pub const BG_BRIGHT_CYAN: &str = concat!("\x1b[", "106", "m");
pub const BG_BRIGHT_WHITE: &str = concat!("\x1b[", "107", "m");

pub const TEXT_BOLD: &str = concat!("\x1b[", "1", "m");
pub const TEXT_ITALIC: &str = concat!("\x1b[", "3", "m");
pub const TEXT_UNDERLINE: &str = concat!("\x1b[", "4", "m");
pub const TEXT_INVERSE: &str = concat!("\x1b[", "7", "m");
pub const TEXT_HIDDEN: &str = concat!("\x1b[", "8", "m");
pub const TEXT_STRIKETHROUGH: &str = concat!("\x1b[", "9", "m");

pub const TEXT_BOLD_RESET: &str = concat!("\x1b[", "22", "m");
pub const TEXT_ITALIC_RESET: &str = concat!("\x1b[", "23", "m");
pub const TEXT_UNDERLINE_RESET: &str = concat!("\x1b[", "24", "m");
pub const TEXT_INVERSE_RESET: &str = concat!("\x1b[", "27", "m");
pub const TEXT_HIDDEN_RESET: &str = concat!("\x1b[", "28", "m");
pub const TEXT_STRIKETHROUGH_RESET: &str = concat!("\x1b[", "29", "m");

pub const RESET: &str = concat!("\x1b[", "0", "m");

pub const FG_DEFAULT: &str = concat!("\x1b[", "39", "m");
pub const BG_DEFAULT: &str = concat!("\x1b[", "49", "m");
