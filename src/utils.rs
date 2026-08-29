use chrono::{Datelike, Utc};

/// Human duration between a month and today, e.g. "3 years 11 months".
pub fn duration_since(start_year: i32, start_month: u32) -> String {
    let now = Utc::now();
    let months = (now.year() - start_year) * 12 + now.month() as i32 - start_month as i32;
    format_months(months.max(0) as u32)
}

fn format_months(total: u32) -> String {
    let years = total / 12;
    let months = total % 12;
    match (years, months) {
        (0, m) => format!("{} month{}", m, if m == 1 { "" } else { "s" }),
        (y, 0) => format!("{} year{}", y, if y == 1 { "" } else { "s" }),
        (y, m) => format!(
            "{} year{} {} month{}",
            y,
            if y == 1 { "" } else { "s" },
            m,
            if m == 1 { "" } else { "s" }
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::format_months;

    #[test]
    fn plural_forms() {
        assert_eq!(format_months(47), "3 years 11 months");
        assert_eq!(format_months(37), "3 years 1 month");
        assert_eq!(format_months(12), "1 year");
        assert_eq!(format_months(1), "1 month");
        assert_eq!(format_months(25), "2 years 1 month");
    }
}
