use colored::Colorize;

/// Success message (green checkmark)
pub fn success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

/// Error message (red X)
pub fn error(message: &str) {
    eprintln!("{} {}", "✗".red(), message);
}

/// Warning message (yellow ⚠)
pub fn warning(message: &str) {
    println!("{} {}", "⚠".yellow(), message);
}

/// Info message (blue ℹ)
pub fn info(message: &str) {
    println!("{} {}", "ℹ".blue(), message);
}

/// Debug message (gray) - only if verbose enabled
pub fn debug(message: &str, verbose: bool) {
    if verbose {
        println!("{} {}", "○".bright_black(), message);
    }
}

/// Separator line
pub fn separator() {
    println!("{}", "─".repeat(60).bright_black());
}

/// Section header
pub fn header(title: &str) {
    println!("\n{}", title.bold().underline());
}

/// Indented text
pub fn indent(message: &str, level: usize) {
    let indent = "  ".repeat(level);
    println!("{}{}", indent, message);
}

/// Suppress output if quiet flag is set
pub fn silent_unless_error(message: &str, quiet: bool) {
    if !quiet {
        println!("{}", message);
    }
}
