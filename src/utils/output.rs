use anyhow::Error;
use owo_colors::OwoColorize;

#[derive(Clone, Copy)]
pub enum Icon {
    Success,
    Error,
    Warning,
    Info,
}

pub fn paint_icon(icon: Icon) -> String {
    match icon {
        Icon::Success => "✓".green().to_string(),
        Icon::Error => "✗".red().to_string(),
        Icon::Warning => "⚠".yellow().to_string(),
        Icon::Info => "•".blue().to_string(),
    }
}

pub fn print_line(icon: Icon, message: impl AsRef<str>) {
    println!("{} {}", paint_icon(icon), message.as_ref());
}

pub fn print_error(error: &Error) {
    eprintln!("{} {}", paint_icon(Icon::Error), error);

    for cause in error.chain().skip(1) {
        eprintln!("{} {}", paint_icon(Icon::Info), cause);
    }
}
