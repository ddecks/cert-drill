// Terminal rendering utilities
// Kept as a separate module for future TUI expansion (ratatui, etc.)

use colored::Colorize;

pub fn header(text: &str) {
    println!("\n{}", text.bold());
    println!("{}", "─".repeat(text.len()));
}

pub fn success(text: &str) {
    println!("{} {}", "✓".green(), text);
}

pub fn error(text: &str) {
    println!("{} {}", "✗".red(), text);
}

pub fn info(text: &str) {
    println!("{} {}", "ℹ".blue(), text);
}
