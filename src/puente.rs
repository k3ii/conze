use crate::Holiday;
use chrono::{Datelike, Month, NaiveDate, Weekday};
use colored::Colorize;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets, Cell, Color, ContentArrangement, Table};
use num_traits::FromPrimitive;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Eq, PartialEq)]
struct PuenteDay {
    date: NaiveDate,
    related_holidays: Vec<NaiveDate>,
}

fn is_weekday(date: NaiveDate) -> bool {
    let weekday = date.weekday();
    weekday != Weekday::Sat && weekday != Weekday::Sun
}

pub fn print_puente_days(month: Option<u32>, year: i32, holidays: &[&Holiday], country_code: &str) {
    let mut table = Table::new();
    table
        .set_header(vec![
            Cell::new("Holiday\nDates").fg(Color::Blue),
            Cell::new("Holiday\nDays").fg(Color::Blue),
            Cell::new("Holiday\nNames").fg(Color::Blue),
            Cell::new("Bridge\nDates").fg(Color::Green),
            Cell::new("Bridge\nDays").fg(Color::Green),
        ])
        .load_preset(presets::UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(140);

    match month {
        Some(m) => println!(
            "\n{}",
            format!(
                "🌉 Bridge days for {} {} ({}):",
                Month::from_u32(m).unwrap().name(),
                year,
                country_code
            )
            .bold()
            .yellow()
        ),
        None => println!(
            "\n{}",
            format!("🌉 Bridge days for the year {} ({}):", year, country_code)
                .bold()
                .yellow()
        ),
    }

    let mut unique_puentes = HashSet::new();
    let mut puente_days = Vec::new();

    // Collect all holiday dates for easier comparison
    let holiday_dates: HashSet<NaiveDate> = holidays.iter().map(|h| h.date).collect();

    // Create a map of consecutive holidays (only weekdays)
    let mut consecutive_holidays: HashMap<NaiveDate, Vec<NaiveDate>> = HashMap::new();
    let mut sorted_holidays: Vec<NaiveDate> = holiday_dates
        .iter()
        .copied()
        .filter(|&date| is_weekday(date))
        .collect();
    sorted_holidays.sort();

    // Find consecutive holidays (only considering weekdays)
    for window in sorted_holidays.windows(2) {
        if let [date1, date2] = window {
            if date2.signed_duration_since(*date1).num_days() == 1 {
                consecutive_holidays
                    .entry(*date1)
                    .or_insert_with(Vec::new)
                    .push(*date2);
            }
        }
    }

    // Process regular puente scenarios
    for holiday in holidays {
        let holiday_date = holiday.date;
        if !is_weekday(holiday_date) {
            continue; // Skip weekend holidays for regular puente scenarios
        }

        let weekday = holiday_date.weekday();

        // Case 1: Tuesday puente (Monday becomes puente)
        if weekday == Weekday::Tue {
            if let Some(puente_date) = holiday_date.pred_opt() {
                if !holiday_dates.contains(&puente_date) {
                    add_puente(
                        &mut unique_puentes,
                        &mut puente_days,
                        puente_date,
                        vec![holiday_date],
                    );
                }
            }
        }

        // Case 2: Thursday puente (Friday becomes puente)
        if weekday == Weekday::Thu {
            if let Some(puente_date) = holiday_date.succ_opt() {
                if !holiday_dates.contains(&puente_date) {
                    add_puente(
                        &mut unique_puentes,
                        &mut puente_days,
                        puente_date,
                        vec![holiday_date],
                    );
                }
            }
        }

        // Case 3: Monday holiday (previous Friday becomes puente)
        if weekday == Weekday::Mon {
            if let Some(puente_date) = holiday_date
                .pred_opt()
                .and_then(|d| d.pred_opt().and_then(|d| d.pred_opt()))
            {
                if !holiday_dates.contains(&puente_date) {
                    add_puente(
                        &mut unique_puentes,
                        &mut puente_days,
                        puente_date,
                        vec![holiday_date],
                    );
                }
            }
        }

        // Case 4: Friday holiday (next Monday becomes puente)
        if weekday == Weekday::Fri {
            if let Some(puente_date) = holiday_date
                .succ_opt()
                .and_then(|d| d.succ_opt().and_then(|d| d.succ_opt()))
            {
                if !holiday_dates.contains(&puente_date) {
                    add_puente(
                        &mut unique_puentes,
                        &mut puente_days,
                        puente_date,
                        vec![holiday_date],
                    );
                }
            }
        }
    }

    // Case 5: Sandwich days between holidays (only considering weekdays)
    for holiday1 in holidays {
        if !is_weekday(holiday1.date) {
            continue;
        }
        for holiday2 in holidays {
            if !is_weekday(holiday2.date) {
                continue;
            }
            if holiday1.date < holiday2.date {
                if let Some(middle_date) = holiday1.date.succ_opt() {
                    if middle_date < holiday2.date
                        && (holiday2
                            .date
                            .signed_duration_since(holiday1.date)
                            .num_days()
                            == 2)
                        && !holiday_dates.contains(&middle_date)
                        && is_weekday(middle_date)
                    {
                        add_puente(
                            &mut unique_puentes,
                            &mut puente_days,
                            middle_date,
                            vec![holiday1.date, holiday2.date],
                        );
                    }
                }
            }
        }
    }

    // Case 6: Consecutive holidays (already filtered for weekdays only)
    for (first_holiday, consecutive) in consecutive_holidays {
        let related_holidays = std::iter::once(first_holiday)
            .chain(consecutive.iter().copied())
            .collect::<Vec<_>>();

        // Check day before consecutive holidays
        if let Some(before_date) = first_holiday.pred_opt() {
            if !holiday_dates.contains(&before_date) && is_weekday(before_date) {
                add_puente(
                    &mut unique_puentes,
                    &mut puente_days,
                    before_date,
                    related_holidays.clone(),
                );
            }
        }

        // Check day after consecutive holidays
        if let Some(last_holiday) = consecutive.last() {
            if let Some(after_date) = last_holiday.succ_opt() {
                if !holiday_dates.contains(&after_date) && is_weekday(after_date) {
                    add_puente(
                        &mut unique_puentes,
                        &mut puente_days,
                        after_date,
                        related_holidays.clone(),
                    );
                }
            }
        }
    }

    // Sort puente_days by complete date
    puente_days.sort_by(|a, b| b.date.cmp(&a.date));

    // Filter by month if specified
    let filtered_puente_days: Vec<&PuenteDay> = if let Some(m) = month {
        puente_days.iter().filter(|p| p.date.month() == m).collect()
    } else {
        puente_days.iter().collect()
    };

    // Reverse the filtered days to show them in chronological order
    let filtered_puente_days: Vec<_> = filtered_puente_days.into_iter().rev().collect();

    // Add rows to table
    for puente_day in &filtered_puente_days {
        let mut related_holidays = puente_day.related_holidays.clone();
        related_holidays.sort(); // Sort related holidays by date
        add_row_to_table(&mut table, &related_holidays, holidays, puente_day.date);
    }

    if !filtered_puente_days.is_empty() {
        println!("{table}");
        println!(
            "\n{}",
            format!(
                "🎯 Found {} bridge opportunities!",
                filtered_puente_days.len()
            )
            .bold()
            .green()
        );
    } else {
        println!("\n{}", "😢 No bridge days found.".bold().red());
    }

    let total_holidays_message = match month {
        Some(m) => format!(
            "📅 Total holidays for this month: {}",
            holidays.iter().filter(|&h| h.date.month() == m).count()
        ),
        None => format!("📅 Total holidays for this year: {}", holidays.len()),
    };
    println!("{}", total_holidays_message.bold().blue());
    println!();
}

fn add_puente(
    unique_puentes: &mut HashSet<NaiveDate>,
    puente_days: &mut Vec<PuenteDay>,
    puente_date: NaiveDate,
    related_holidays: Vec<NaiveDate>,
) {
    if unique_puentes.insert(puente_date) {
        puente_days.push(PuenteDay {
            date: puente_date,
            related_holidays,
        });
    }
}

fn add_row_to_table(
    table: &mut Table,
    holiday_dates: &[NaiveDate],
    holidays: &[&Holiday],
    puente_date: NaiveDate,
) {
    let holiday_dates_str: String = holiday_dates
        .iter()
        .map(|d| d.format("%Y-%m-%d").to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let holiday_days_str: String = holiday_dates
        .iter()
        .map(|d| weekday_to_string(d.weekday()))
        .collect::<Vec<_>>()
        .join("\n");

    let holiday_names_str: String = holiday_dates
        .iter()
        .filter_map(|date| get_holiday_name(holidays, *date))
        .collect::<Vec<_>>()
        .join("\n");

    table.add_row(vec![
        Cell::new(holiday_dates_str).fg(Color::Cyan),
        Cell::new(holiday_days_str).fg(Color::Cyan),
        Cell::new(holiday_names_str).fg(Color::Cyan),
        Cell::new(puente_date.format("%Y-%m-%d").to_string()).fg(Color::Green),
        Cell::new(weekday_to_string(puente_date.weekday())).fg(Color::Green),
    ]);
}

fn get_holiday_name(holidays: &[&Holiday], date: NaiveDate) -> Option<String> {
    holidays
        .iter()
        .find(|h| h.date == date)
        .map(|h| h.name.clone())
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
