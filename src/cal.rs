use crate::Holiday;
use chrono::{Datelike, Month, NaiveDate};
use colored::Colorize;
use num_traits::FromPrimitive;

pub fn print_calendar(month: u32, year: i32, holidays: &[&Holiday]) {
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
        // Check if the current day is a holiday
        let is_holiday = holidays.iter().any(|holiday| holiday.date.day() == day);
        if (day + start_day_of_week - 1) % 7 == 0 {
            print!("\n");
        }
        // Print the day, coloring it if it's a holiday
        if is_holiday {
            print!("{} ", day.to_string().green());
        } else {
            print!("{:2} ", day);
        }
    }
    println!("\n");

    println!("Holidays this month:");
    for holiday in holidays {
        println!("{}: {}", holiday.date.day(), holiday.name.green());
    }
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
