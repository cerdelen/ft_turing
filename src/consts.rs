pub const RESET_CHAR: &str = "\u{1b}[0m";
pub const BOLD_CHAR: &str = "\u{1b}[1m";
pub const CYAN_CHAR: &str = "\u{1b}[36m";
pub const GREEN_CHAR: &str = "\u{1b}[32m";
pub const BOLD_YELLOW_CHAR: &str = "\u{1b}[1;33m";
pub const BOLD_GREEN_CHAR: &str = "\u{1b}[1;32m";
pub const BOLD_PINK_CHAR: &str = "\u{1b}[1;35m";
pub const BOLD_RED_CHAR: &str = "\u{1b}[1;31m";
pub const ON_PINK_CHAR: &str = "\u{1b}[45m";
pub const V_BORDER: &str = "*                                                                              *";
pub const H_BORDER: &str = "********************************************************************************";

pub fn format_error_message(message: &str, file_path: &String) -> String {

    format!("\n{}\n\n{}{}: {} \"{}\" {}\n\n{}\n\n",
            H_BORDER, BOLD_RED_CHAR, message, BOLD_YELLOW_CHAR, file_path, RESET_CHAR, H_BORDER)
}