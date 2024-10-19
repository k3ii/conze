use regex::Regex;

pub fn parse_month(month_str: &str) -> Option<u32> {
    // First, try to parse as a number
    if let Ok(month_num) = month_str.parse::<u32>() {
        if (1..=12).contains(&month_num) {
            return Some(month_num);
        }
    }

    // If not a number, try to match against month names or abbreviations
    let month_str_lower = month_str.to_lowercase();
    let month_regex =
        Regex::new(r"^(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[a-z]*$").unwrap();

    if let Some(captures) = month_regex.captures(&month_str_lower) {
        match captures.get(1).unwrap().as_str() {
            "jan" => Some(1),
            "feb" => Some(2),
            "mar" => Some(3),
            "apr" => Some(4),
            "may" => Some(5),
            "jun" => Some(6),
            "jul" => Some(7),
            "aug" => Some(8),
            "sep" => Some(9),
            "oct" => Some(10),
            "nov" => Some(11),
            "dec" => Some(12),
            _ => None,
        }
    } else {
        None
    }
}
