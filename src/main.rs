mod cal;
mod cli;
mod parser;
mod puente;

use chrono::{Datelike, Local, NaiveDate};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::cal::print_calendar;
use crate::parser::parse_month;
use crate::puente::print_puente_days;

#[derive(Debug, Serialize, Deserialize)]
struct Holiday {
    name: String,
    date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
struct HolidaysByYear {
    #[serde(flatten)]
    years: HashMap<String, Vec<Holiday>>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let matches = cli::cli().get_matches();

    let mu_holidays: HolidaysByYear = reqwest::Client::new()
        .get("https://raw.githubusercontent.com/nicolasstrands/data-konzer/main/data/public-holidays.json")
        .send()
        .await?
        .json()
        .await?;

    let current_year = Local::now().year();

    match matches.subcommand() {
        Some(("bridge", sub_matches)) => {
            let month = sub_matches.get_one::<String>("month");
            let year = sub_matches
                .get_one::<String>("year")
                .and_then(|y| y.parse::<i32>().ok())
                .unwrap_or(current_year); // Fallback to current year if not provided

            // Retrieve holidays for the given year
            if let Some(holidays) = mu_holidays.years.get(&year.to_string()) {
                match month.and_then(|m| parse_month(&m)) {
                    Some(month) => {
                        // Filter holidays for the selected month
                        let holidays_for_month: Vec<&Holiday> = holidays
                            .iter()
                            .filter(|holiday| holiday.date.month() == month)
                            .collect();
                        print_puente_days(Some(month), year, &holidays_for_month);
                    }
                    None => {
                        // If no specific month is provided, handle all holidays for the year
                        let holiday_refs: Vec<&Holiday> = holidays.iter().collect();
                        print_puente_days(None, year, &holiday_refs);
                    }
                }
            } else {
                println!("{}", bridge_pun(year));
            }
        }
        Some(("calendar", sub_matches)) => {
            let month = sub_matches.get_one::<String>("month");
            let month = month
                .and_then(|m| parse_month(m))
                .unwrap_or_else(|| chrono::Local::now().month());
            let year = sub_matches
                .get_one::<String>("year")
                .and_then(|y| y.parse::<i32>().ok())
                .unwrap_or(current_year);

            if let Some(holidays) = mu_holidays.years.get(&year.to_string()) {
                let holidays_for_month: Vec<&Holiday> = holidays
                    .iter()
                    .filter(|holiday| holiday.date.month() == month)
                    .collect();
                print_calendar(month, year, &holidays_for_month);
            } else {
                println!("No holidays found for the year {}.", year);
            }
        }

        _ => {
            println!("Test")
        }
    }

    Ok(())
}

fn bridge_pun(year: i32) -> String {
    // Create a list of pun lines
    let pun_lines = [
        format!("No holidays found for the year {}... looks like we'll have to bridge the gap to next year!", year),
        format!("No holidays found for the year {}... you'll have to find another bridge to escape!", year),
        format!("No holidays found for the year {}... guess it's time to build a new bridge to take a break!", year),
        format!("No holidays found for the year {}... looks like the holiday bridge is under construction!", year),
        format!("No holidays found for the year {}... looks like all bridges to time off are closed!", year),
        format!("No holidays found for the year {}... it’s a long road with no bridges in sight!", year),
        format!("No holidays found for the year {}... guess the bridge to holidays has been washed away!", year),
    ];

    // Select a random pun line
    let random_pun = pun_lines.choose(&mut rand::thread_rng()).unwrap();

    // Return the random pun line
    random_pun.to_string() // Convert to String for return
}
