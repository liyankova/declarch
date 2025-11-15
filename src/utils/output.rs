use colored::Colorize;

// ============================================================================
// BASIC OUTPUT
// ============================================================================

/// Success message
pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

/// Error message
pub fn error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg);
}

/// Warning message
pub fn warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg);
}

/// Info message
pub fn info(msg: &str) {
    println!("{} {}", "ℹ".blue().bold(), msg);
}

/// Separator line
pub fn separator() {
    println!("{}", "─".repeat(60).bright_black());
}

// ============================================================================
// STRUCTURED OUTPUT
// ============================================================================

/// Header for major sections
pub fn header(title: &str) {
    println!();
    println!("{}", title.cyan().bold());
    println!("{}", "─".repeat(title.len()).bright_black());
}

/// Subheader for subsections
pub fn subheader(title: &str) {
    println!("{}", title.bright_black().italic());
}

/// Key-value pair (compact)
pub fn keyval(key: &str, value: &str) {
    println!("  {}{} {}", key.bold(), ":".bright_black(), value);
}

/// Item in list
pub fn item(text: &str) {
    println!("  • {}", text);
}

/// Item with emphasis
pub fn item_bold(text: &str) {
    println!("  • {}", text.bold());
}

/// Indented text
pub fn indent(text: &str, level: usize) {
    let spaces = "  ".repeat(level);
    println!("{}{}", spaces, text);
}

/// Tag/badge display
pub fn tag(label: &str, value: &str) {
    println!(
        "  {} {}",
        format!("[{}]", label).cyan().bold(),
        value
    );
}

// ============================================================================
// INTERACTIVE PROMPTS
// ============================================================================

use std::io::{self, Write};

/// Yes/No prompt
pub fn prompt_yes_no(question: &str) -> bool {
    print!("\n{} {} {} ", "?".cyan().bold(), question, "[y/N]".bright_black());
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Multiple choice prompt
pub fn prompt_choice(question: &str, choices: &[&str]) -> Option<usize> {
    println!("\n{} {}", "?".cyan().bold(), question);
    
    for (i, choice) in choices.iter().enumerate() {
        println!("  {} {}", format!("({})", i + 1).cyan(), choice);
    }

    print!("\n{} ", "→".cyan().bold());
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    input
        .trim()
        .parse::<usize>()
        .ok()
        .and_then(|n| if n > 0 && n <= choices.len() { Some(n - 1) } else { None })
}

/// Text input prompt
pub fn prompt_input(question: &str, default: &str) -> String {
    print!("\n{} {} {} ", "?".cyan().bold(), question, format!("[{}]", default).bright_black());
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    let trimmed = input.trim();
    if trimmed.is_empty() {
        default.to_string()
    } else {
        trimmed.to_string()
    }
}

// ============================================================================
// STATUS/SUMMARY DISPLAY
// ============================================================================

/// Show status summary
pub fn status_summary(items: &[(&str, &str)]) {
    println!();
    for (key, value) in items {
        keyval(key, value);
    }
}

/// Show compact list
pub fn list_compact(items: &[&str]) {
    for text in items {
        item(text);
    }
}

/// Show count badge
pub fn count_badge(label: &str, count: usize) {
    let badge = if count == 0 {
        format!("{}", count).bright_black()
    } else {
        format!("{}", count).cyan().bold()
    };
    println!("  {} {}", label.bright_black(), badge);
}

/// Debug message (only if verbose)
pub fn debug(msg: &str, verbose: bool) {
    if verbose {
        println!("{} {}", "○".bright_black(), msg.bright_black());
    }
}
