use crate::CountryHolidays;
use chrono::{Datelike, Month, NaiveDate};
use colored::{Color, ColoredString, Colorize};
use num_traits::FromPrimitive;
use prettytable::{row, Table};
use std::collections::HashMap;

pub fn print_calendar_comparison(month: u32, year: i32, country_holidays: &[CountryHolidays]) {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let num_days_in_month = days_in_month(year, month);
    let start_day_of_week = first_day.weekday().num_days_from_sunday();

    println!(
        "{} {}\nSu Mo Tu We Th Fr Sa",
        Month::from_u32(month).unwrap().name(),
        year
    );

    // Assign a color for each country dynamically
    let colors = assign_colors(country_holidays);

    // Create a HashMap to store holidays for quick lookup
    let mut holiday_map: HashMap<u32, Vec<(&str, &str)>> = HashMap::new();
    for country in country_holidays {
        for holiday in &country.holidays {
            if holiday.date.month() == month && holiday.date.year() == year {
                holiday_map
                    .entry(holiday.date.day())
                    .or_insert_with(Vec::new)
                    .push((&country.country, &holiday.name));
            }
        }
    }

    // Calendar view
    for _ in 0..start_day_of_week {
        print!("   ");
    }

    for day in 1..=num_days_in_month {
        if (day + start_day_of_week - 1) % 7 == 0 {
            print!("\n");
        }

        if let Some(holidays) = holiday_map.get(&day) {
            let colored_day = match holidays.len() {
                1 => colorize_day(day, holidays[0].0, &colors),
                _ => day.to_string().magenta(), // Multiple holidays on the same day
            };
            print!("{:2} ", colored_day);
        } else {
            print!("{:2} ", day);
        }
    }
    println!("\n");

    // Print holidays for each country in a table format
    let mut table = Table::new();
    table.add_row(row!["Country", "Date", "Holiday"]);

    for country in country_holidays {
        let holidays_this_month: Vec<_> = country
            .holidays
            .iter()
            .filter(|holiday| holiday.date.month() == month && holiday.date.year() == year)
            .collect();

        for holiday in holidays_this_month {
            let colored_text = colorize_holiday(&holiday.name, &country.country, &colors);

            table.add_row(row![
                &country.country,
                holiday.date.day().to_string(),
                colored_text
            ]);
        }
    }

    table.printstd(); // Print the table to standard output
}

fn assign_colors(country_holidays: &[CountryHolidays]) -> HashMap<String, Color> {
    let color_choices = vec![
        Color::Green,
        Color::Blue,
        Color::Yellow,
        Color::Red,
        Color::Cyan,
    ];

    let mut color_map = HashMap::new();
    for (i, country) in country_holidays.iter().enumerate() {
        let color = color_choices
            .get(i % color_choices.len())
            .unwrap_or(&Color::White);
        color_map.insert(country.country.clone(), *color);
    }

    color_map
}

fn colorize_day(day: u32, country: &str, colors: &HashMap<String, Color>) -> ColoredString {
    colors
        .get(country)
        .map(|&color| day.to_string().color(color))
        .unwrap_or_else(|| day.to_string().normal())
}

fn colorize_holiday(
    holiday_name: &str,
    country: &str,
    colors: &HashMap<String, Color>,
) -> ColoredString {
    colors
        .get(country)
        .map(|&color| holiday_name.color(color))
        .unwrap_or_else(|| holiday_name.normal())
}

fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .unwrap()
    .pred_opt()
    .unwrap()
    .day()
}
