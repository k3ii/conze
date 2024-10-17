use crate::Holiday;
use chrono::Datelike;
use colored::Colorize;
use comfy_table::{Cell, Color, ContentArrangement, Table};

pub fn list_holidays(holidays: &[Holiday], country: &str, year: i32) {
    let mut table = Table::new();
    table
        .set_header(vec![
            Cell::new("Date").fg(Color::Blue),
            Cell::new("Day").fg(Color::Blue),
            Cell::new("Holiday").fg(Color::Blue),
        ])
        .load_preset(comfy_table::presets::UTF8_FULL)
        .apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(100);

    for holiday in holidays.iter().filter(|h| h.date.year() == year) {
        table.add_row(vec![
            Cell::new(holiday.date.format("%d-%m-%Y").to_string()).fg(Color::Cyan),
            Cell::new(weekday_to_string(holiday.date.weekday())).fg(Color::Cyan),
            Cell::new(&holiday.name).fg(Color::Green),
        ]);
    }

    println!(
        "\n{}",
        format!("📅 Holidays for {} in {}", country, year)
            .bold()
            .yellow()
    );
    println!("{table}");
    println!(
        "\n{}",
        format!("Total holidays: {}", holidays.len()).bold().blue()
    );
}

fn weekday_to_string(weekday: chrono::Weekday) -> String {
    match weekday {
        chrono::Weekday::Mon => "Monday",
        chrono::Weekday::Tue => "Tuesday",
        chrono::Weekday::Wed => "Wednesday",
        chrono::Weekday::Thu => "Thursday",
        chrono::Weekday::Fri => "Friday",
        chrono::Weekday::Sat => "Saturday",
        chrono::Weekday::Sun => "Sunday",
    }
    .to_string()
}
