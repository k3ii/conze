pub fn parse_month(month_str: &str) -> Option<u32> {
    if let Ok(month_num) = month_str.parse::<u32>() {
        if (1..=12).contains(&month_num) {
            return Some(month_num);
        }
    }

    let month_str_lower = month_str.to_lowercase();
    match month_str_lower.as_str() {
        "january" | "jan" => Some(1),
        "february" | "feb" => Some(2),
        "march" | "mar" => Some(3),
        "april" | "apr" => Some(4),
        "may" => Some(5),
        "june" | "jun" => Some(6),
        "july" | "jul" => Some(7),
        "august" | "aug" => Some(8),
        "september" | "sep" | "sept" => Some(9),
        "october" | "oct" => Some(10),
        "november" | "nov" => Some(11),
        "december" | "dec" => Some(12),
        _ => None,
    }
}
