mod cal;
mod cli;
mod parser;
mod puente;

use chrono::{Datelike, Local, NaiveDate};
use colored::Colorize;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::parser::parse_month;
use crate::puente::print_puente_days;

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone
struct Holiday {
    name: String,
    date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
struct HolidaysByYear {
    #[serde(flatten)]
    years: HashMap<String, Vec<Holiday>>,
}

// New struct to hold country-specific holiday data
#[derive(Debug)]
struct CountryHolidays {
    country: String,
    holidays: Vec<Holiday>,
}

async fn fetch_holidays(url: &str) -> Result<HolidaysByYear, reqwest::Error> {
    reqwest::Client::new().get(url).send().await?.json().await
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let matches = cli::cli().get_matches();

    let urls = HashMap::from([
        ("MU", "https://raw.githubusercontent.com/nicolasstrands/data-konzer/main/data/public-holidays.json"),
        ("SA", "https://raw.githubusercontent.com/nicolasstrands/data-konzer/main/data/public-holidays-sa.json"),
        ("FR", "https://raw.githubusercontent.com/nicolasstrands/data-konzer/refs/heads/main/data/public-holidays-fr.json"),
    ]);

    let current_year = Local::now().year();

    match matches.subcommand() {
        Some(("bridge", sub_matches)) => {
            let month = sub_matches.get_one::<String>("month");
            let year = sub_matches
                .get_one::<String>("year")
                .and_then(|y| y.parse::<i32>().ok())
                .unwrap_or(current_year); // Fallback to current year if not provided

            let country_code = sub_matches
                .get_one::<String>("country")
                .map(|s| s.as_str())
                .unwrap_or("MU");

            if let Some(url) = urls.get(country_code) {
                if let Ok(holidays_data) = fetch_holidays(url).await {
                    if let Some(holidays) = holidays_data.years.get(&year.to_string()) {
                        match month.and_then(|m| parse_month(&m)) {
                            Some(month) => {
                                let holidays_for_month: Vec<&Holiday> = holidays
                                    .iter()
                                    .filter(|holiday| holiday.date.month() == month)
                                    .collect();
                                print_puente_days(
                                    Some(month),
                                    year,
                                    &holidays_for_month,
                                    country_code,
                                );
                            }
                            None => {
                                let holiday_refs: Vec<&Holiday> = holidays.iter().collect();
                                print_puente_days(None, year, &holiday_refs, country_code);
                            }
                        }
                    } else {
                        println!("{}", bridge_pun(year));
                    }
                } else {
                    println!("Failed to fetch holiday data for {}", country_code);
                }
            } else {
                println!("Unsupported country code: {}", country_code);
            }
        }

        Some(("calendar", sub_matches)) => {
            let month = sub_matches.get_one::<String>("month");
            let month = month
                .and_then(|m| parser::parse_month(m))
                .unwrap_or_else(|| chrono::Local::now().month());
            let year = sub_matches
                .get_one::<String>("year")
                .and_then(|y| y.parse::<i32>().ok())
                .unwrap_or(current_year);

            let compare_country = sub_matches.get_one::<String>("compare");

            let mut country_holidays = Vec::new();
            let mut missing_data = Vec::new();

            // Fetch Mauritius holidays
            if let Ok(mu_holidays) = fetch_holidays(urls["MU"]).await {
                if let Some(holidays) = mu_holidays.years.get(&year.to_string()) {
                    country_holidays.push(CountryHolidays {
                        country: "MU".to_string(),
                        holidays: holidays.clone(),
                    });
                } else {
                    missing_data.push("Mauritius");
                }
            }

            // Fetch comparison country if specified
            if let Some(country_code) = compare_country {
                if let Some(url) = urls.get(country_code.as_str()) {
                    if let Ok(country_data) = fetch_holidays(url).await {
                        if let Some(holidays) = country_data.years.get(&year.to_string()) {
                            country_holidays.push(CountryHolidays {
                                country: country_code.to_string(),
                                holidays: holidays.clone(),
                            });
                        } else {
                            missing_data.push(match country_code.as_str() {
                                "SA" => "South Africa",
                                _ => country_code.as_str(),
                            });
                        }
                    }
                } else {
                    println!("Unsupported country code: {}", country_code);
                    return Ok(());
                }
            }

            if !missing_data.is_empty() {
                println!(
                    "{}\n",
                    format!(
                        "⚠️  No data available for year {} in {}.",
                        year,
                        missing_data.join(" and ")
                    )
                    .bold()
                    .yellow()
                );

                // Print available years for each country with missing data
                for country in missing_data {
                    let url = match country {
                        "Mauritius" => urls["MU"],
                        "South Africa" => urls["SA"],
                        _ => continue,
                    };

                    if let Ok(holiday_data) = fetch_holidays(url).await {
                        let available_years: Vec<_> = holiday_data
                            .years
                            .keys()
                            .map(|y| y.parse::<i32>().unwrap_or(0))
                            .collect();

                        if !available_years.is_empty() {
                            let min_year = available_years.iter().min().unwrap();
                            let max_year = available_years.iter().max().unwrap();
                            println!(
                                "{}",
                                format!(
                                    "📅 Available years for {}: {} to {}",
                                    country, min_year, max_year
                                )
                                .bold()
                                .blue()
                            );
                        }
                    }
                }
                println!(); // Add a blank line for better formatting
            }

            if !country_holidays.is_empty() {
                cal::print_calendar_comparison(month, year, &country_holidays);
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
