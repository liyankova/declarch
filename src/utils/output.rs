use colored::Colorize;

/// Success message (green checkmark)
pub fn success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

/// Error message (red X)
pub fn error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

/// Warning message (yellow ⚠)
pub fn warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message);
}

/// Info message (blue ℹ)
pub fn info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

/// Debug message (gray)
pub fn debug(message: &str, verbose: bool) {
    if verbose {
        println!("{} {}", "○".bright_black(), message.bright_black());
    }
}

/// Separator line
pub fn separator() {
    println!();
    println!("{}", "─".repeat(60).bright_black());
    println!();
}

/// Section header (bold + colored)
pub fn header(title: &str) {
    println!();
    println!("{}", title.bold().cyan());
    println!("{}", "─".repeat(title.len()).bright_black());
}

/// Item listing (bullet + content)
pub fn item(content: &str) {
    println!("{} {}", "•".cyan(), content);
}

/// Item with highlight
pub fn item_highlight(content: &str) {
    println!("{} {}", "*".cyan().bold(), content.bold());
}

/// Indented text
pub fn indent(message: &str, level: usize) {
    let indent = "  ".repeat(level);
    println!("{}{}", indent, message);
}

/// Key-value pair display
pub fn keyval(key: &str, value: &str) {
    println!("  {} {}", format!("{}:", key).bright_black(), value.white());
}

/// Table row (for status display)
pub fn table_row(cells: &[&str]) {
    println!("  {}", cells.join("   "));
}
