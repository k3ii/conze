use crate::CountryHolidays;
use chrono::{Datelike, Month, NaiveDate};
use colored::Colorize;
use num_traits::FromPrimitive;
use prettytable::{color, row, Attr, Table};

pub fn print_calendar_comparison(month: u32, year: i32, country_holidays: &[CountryHolidays]) {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let num_days_in_month = days_in_month(year, month);
    let start_day_of_week = first_day.weekday().num_days_from_sunday();

    println!(
        "{} {}\nSu Mo Tu We Th Fr Sa",
        Month::from_u32(month).unwrap().name(),
        year
    );

    for _ in 0..start_day_of_week {
        print!("   ");
    }

    for day in 1..=num_days_in_month {
        if (day + start_day_of_week - 1) % 7 == 0 {
            print!("\n");
        }

        let mut is_holiday = false;
        let mut holiday_color = None;

        for (i, country) in country_holidays.iter().enumerate() {
            let is_country_holiday = country
                .holidays
                .iter()
                .any(|holiday| holiday.date.month() == month && holiday.date.day() == day);

            if is_country_holiday {
                is_holiday = true;
                holiday_color = Some(match i {
                    0 => "green",  // First country (Mauritius)
                    1 => "blue",   // Second country (South Africa)
                    _ => "yellow", // Additional countries
                });
                break;
            }
        }

        if is_holiday {
            match holiday_color.unwrap() {
                "green" => print!("{:2} ", day.to_string().green()),
                "blue" => print!("{:2} ", day.to_string().blue()),
                "yellow" => print!("{:2} ", day.to_string().yellow()),
                _ => print!("{:2} ", day),
            }
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
            .filter(|holiday| holiday.date.month() == month)
            .collect();

        for holiday in holidays_this_month {
            let colored_text = match country.country.as_str() {
                "MU" => holiday.name.green(),
                "SA" => holiday.name.blue(),
                _ => holiday.name.yellow(),
            };

            table.add_row(row![
                country.country,
                holiday.date.day().to_string(),
                colored_text
            ]);
        }
    }

    table.printstd(); // Print the table to standard output
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
