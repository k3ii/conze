use crate::Holiday;
use chrono::Month;
use chrono::{Datelike, NaiveDate, Weekday};
use colored::Colorize;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets, Cell, Color, ContentArrangement, Table};
use num_traits::FromPrimitive;

pub fn print_puente_days(month: Option<u32>, year: i32, holidays: &[&Holiday]) {
    let mut table = Table::new();

    table
        .set_header(vec![
            Cell::new("Holiday\nDate").fg(Color::Blue),
            Cell::new("Holiday\nDay").fg(Color::Blue),
            Cell::new("Holiday\nName").fg(Color::Blue),
            Cell::new("Puente\nDate").fg(Color::Green),
            Cell::new("Puente\nDay").fg(Color::Green),
        ])
        .load_preset(presets::UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(120);

    match month {
        Some(m) => println!(
            "\n{}",
            format!(
                "🌉 Puente days for {} {}:",
                Month::from_u32(m).unwrap().name(),
                year
            )
            .bold()
            .yellow()
        ),
        None => println!(
            "\n{}",
            format!("🌉 Puente days for the year {}:", year)
                .bold()
                .yellow()
        ),
    }

    let mut puente_count = 0;
    let holiday_count = holidays.len();

    for (i, holiday) in holidays.iter().enumerate() {
        let holiday_date = holiday.date;
        let weekday = holiday_date.weekday();

        // First rule: puente when holiday is Tuesday or Thursday
        let (puente_date, puente_day) = if weekday == Weekday::Tue {
            (holiday_date.pred_opt().unwrap(), "Monday")
        } else if weekday == Weekday::Thu {
            (holiday_date.succ_opt().unwrap(), "Friday")
        } else if weekday == Weekday::Mon {
            // Second rule: puente when holiday is Monday, previous Friday is puente
            (
                holiday_date.pred_opt().unwrap().pred_opt().unwrap(),
                "Friday",
            )
        } else if weekday == Weekday::Fri {
            // Third rule: puente when holiday is Friday, next Monday is puente
            (
                holiday_date.succ_opt().unwrap().succ_opt().unwrap(),
                "Monday",
            )
        } else {
            continue;
        };

        // Fourth rule: puente when two holidays have a weekday in between
        if i < holiday_count - 1 {
            let next_holiday = holidays[i + 1];
            let next_holiday_date = next_holiday.date;
            let day_diff = next_holiday_date
                .signed_duration_since(holiday_date)
                .num_days();
            if day_diff == 2 {
                let puente_in_between = holiday_date.succ_opt().unwrap();
                add_row_to_table(
                    &mut table,
                    holiday_date,
                    weekday,
                    &holiday.name,
                    puente_in_between,
                    weekday_to_string(puente_in_between.weekday()).as_str(),
                );
                puente_count += 1;
            }
        }

        // Filter by month if applicable
        if let Some(m) = month {
            if holiday_date.month() == m {
                add_row_to_table(
                    &mut table,
                    holiday_date,
                    weekday,
                    &holiday.name,
                    puente_date,
                    puente_day,
                );
                puente_count += 1;
            }
        } else {
            add_row_to_table(
                &mut table,
                holiday_date,
                weekday,
                &holiday.name,
                puente_date,
                puente_day,
            );
            puente_count += 1;
        }
    }

    if puente_count > 0 {
        println!("{table}");
        println!(
            "\n{}",
            format!("🎯 Found {} puente opportunities!", puente_count)
                .bold()
                .green()
        );
    } else {
        println!("\n{}", "😢 No puente days found.".bold().red());
    }

    println!(
        "{}",
        format!("📅 Total holidays: {}", holiday_count)
            .bold()
            .blue()
    );
    println!(); // Add a newline for better spacing
}

fn add_row_to_table(
    table: &mut Table,
    holiday_date: NaiveDate,
    weekday: Weekday,
    holiday_name: &str,
    puente_date: NaiveDate,
    puente_day: &str,
) {
    table.add_row(vec![
        Cell::new(holiday_date.format("%Y-%m-%d").to_string()).fg(Color::Cyan),
        Cell::new(weekday_to_string(weekday)).fg(Color::Cyan),
        Cell::new(holiday_name).fg(Color::Cyan),
        Cell::new(puente_date.format("%Y-%m-%d").to_string()).fg(Color::Green),
        Cell::new(puente_day).fg(Color::Green),
    ]);
}

fn weekday_to_string(weekday: Weekday) -> String {
    match weekday {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
    .to_string()
}
