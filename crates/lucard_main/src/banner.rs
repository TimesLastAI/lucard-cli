use std::io;

use colored::Colorize;
use crossterm::terminal::size;

const BANNER_LOGO: &str = include_str!("banner");

pub struct BannerInfo {
    pub model: String,
    pub provider: String,
    pub version: String,
    pub conversation_id: String,
}

/// Lucard CLI — purple brand palette.
/// Accent: bright purple/magenta. Secondary: light violet.
fn color_logo_line(line: &str, idx: usize) -> colored::ColoredString {
    if idx < 3 {
        // Top group: bright magenta (warmer purple)
        line.bright_magenta().bold()
    } else {
        // Last line: light/pastel purple
        line.bright_purple().bold()
    }
}

pub fn display(info: &BannerInfo) -> io::Result<()> {
    let logo_lines: Vec<&str> = BANNER_LOGO.lines().collect();
    let (term_width, _) = size().unwrap_or((80, 24));
    let term_width = term_width as usize;

    // Lucard accent color (bright magenta = purple-ish)
    let accent = |s: &str| s.bright_magenta().bold();
    let accent2 = |s: &str| s.bright_purple().bold();
    let accent3 = |s: &str| s.purple().bold();

    // Switch to vertical stacked layout for terminals narrower than 80 columns
    if term_width < 80 {
        // Use a reasonable width for the vertical box, but ensure it fits in terminal
        let box_width = term_width.min(50).saturating_sub(2); // -2 for margins
        let inner_width = box_width.saturating_sub(2); // -2 for borders

        // Top Border (purple)
        println!(
            "{}",
            format!("╭{}╮", "─".repeat(inner_width))
                .bright_magenta()
                .bold()
        );

        // Logo Section (Centered) — purple themed
        for (i, line) in logo_lines.iter().enumerate() {
            let styled = color_logo_line(line, i);

            let logo_len = 11; // Logo is fixed width
            let padding_left = (inner_width.saturating_sub(logo_len)) / 2;
            let padding_right = inner_width
                .saturating_sub(logo_len)
                .saturating_sub(padding_left);

            println!(
                "{} {} {}{}{} {}",
                "│".bright_magenta().bold(),
                " ".repeat(padding_left),
                styled,
                " ".repeat(padding_right),
                "",
                "│".bright_magenta().bold()
            );
        }

        // Empty Separator Line
        println!(
            "{}",
            format!("│{}│", " ".repeat(inner_width))
                .bright_magenta()
                .bold()
        );

        // Info Section
        let label_width = 12; // " ● Session: " is 12 chars
        let max_val_len = inner_width.saturating_sub(label_width);

        let print_info_line =
            |label: &str, value: &str, color_fn: fn(&str) -> colored::ColoredString| {
                let truncated_val = truncate(value, max_val_len);
                let colored_val = color_fn(&truncated_val);
                let total_content_len = label.chars().count() + truncated_val.chars().count();
                let padding = inner_width.saturating_sub(total_content_len);

                println!(
                    "{} {}{}{}{}",
                    "│".bright_magenta().bold(),
                    label,
                    colored_val,
                    " ".repeat(padding),
                    "│".bright_magenta().bold()
                );
            };

        print_info_line(" ● Session: ", &info.conversation_id, accent);
        print_info_line(" ● Model:   ", &info.model, accent2);
        print_info_line(" ● Provider:", &info.provider, accent3);

        // Empty Separator Line
        println!(
            "{}",
            format!("│{}│", " ".repeat(inner_width))
                .bright_magenta()
                .bold()
        );

        // Help Suggestion (Centered) — purple help line
        let help_text = "Type /help for commands";
        let help_len = help_text.chars().count();
        let h_pad_left = (inner_width.saturating_sub(help_len)) / 2;
        let h_pad_right = inner_width
            .saturating_sub(help_len)
            .saturating_sub(h_pad_left);
        println!(
            "{} {}{}{}{} {}",
            "│".bright_magenta().bold(),
            " ".repeat(h_pad_left),
            help_text.bright_purple(),
            " ".repeat(h_pad_right),
            "",
            "│".bright_magenta().bold()
        );

        // Bottom Border
        println!(
            "{}",
            format!("╰{}╯", "─".repeat(inner_width))
                .bright_magenta()
                .bold()
        );
        println!();

        return Ok(());
    }

    // Horizontal Layout
    let logo_width = 11;

    let available_width = term_width.saturating_sub(18);
    let content_width = available_width.clamp(40, 80);

    // Header (purple)
    let top_border = format!(
        "╭{}┬{}╮",
        "─".repeat(logo_width + 2),
        "─".repeat(content_width + 2)
    );
    println!("{}", top_border.bright_magenta().bold());

    // Row 0: Logo | Session
    print_row(
        0,
        &logo_lines,
        " ● Session: ",
        &info.conversation_id,
        "magenta1",
        logo_width,
        content_width,
    );

    let max_value_len = content_width.saturating_sub(13);

    // Row 1: Logo | Model
    let model_display = truncate(&info.model, max_value_len);
    print_row(
        1,
        &logo_lines,
        " ● Model:   ",
        &model_display,
        "purple",
        logo_width,
        content_width,
    );

    // Row 2: Logo | Provider
    let provider_display = truncate(&info.provider, max_value_len);
    print_row(
        2,
        &logo_lines,
        " ● Provider:",
        &provider_display,
        "magenta",
        logo_width,
        content_width,
    );

    // Row 3: Suggestion
    print_row(
        3,
        &logo_lines,
        "",
        "Type /help for commands",
        "violet",
        logo_width,
        content_width,
    );

    // Bottom border (purple)
    let bottom_border = format!(
        "╰{}┴{}╯",
        "─".repeat(logo_width + 2),
        "─".repeat(content_width + 2)
    );
    println!("{}", bottom_border.bright_magenta().bold());
    println!();

    Ok(())
}

fn print_row(
    row_idx: usize,
    logo_lines: &[&str],
    label: &str,
    value: &str,
    color: &str,
    logo_width: usize,
    content_width: usize,
) {
    let logo_line = logo_lines.get(row_idx).unwrap_or(&"");

    // Style logo: first 3 lines bright magenta, last line bright purple
    let logo_styled = color_logo_line(logo_line, row_idx);

    // Style value — Lucard purple palette
    let value_styled = match color {
        "magenta1" => value.bright_magenta().bold(),
        "purple" => value.bright_purple().bold(),
        "magenta" => value.bright_magenta(),
        "violet" => value.purple(),
        _ => value.normal(),
    };

    let label_len = label.chars().count();
    let value_len = value.chars().count();
    let space_len = if value_len > 0 { 1 } else { 0 };
    let total_len = label_len + space_len + value_len;

    let padding = content_width.saturating_sub(total_len);

    // Purple borders
    let border = "│".bright_magenta().bold();
    let separator = "│".bright_magenta().bold();

    print!(
        "{} {:<width$} {} {} {}",
        border,
        logo_styled,
        border,
        if value_len > 0 { " " } else { "" },
        label,
        width = logo_width
    );
    print!("{}", value_styled);
    if padding > 0 {
        print!("{}", " ".repeat(padding));
    }
    println!("{}", separator);
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() > max_len {
        let mut chars: String = s.chars().take(max_len - 3).collect();
        chars.push_str("...");
        chars
    } else {
        s.to_string()
    }
}
