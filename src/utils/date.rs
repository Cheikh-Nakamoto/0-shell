use crate::utils::messages::{NOTHING, TIME_WENT_BACKWARDS};
use std::time::{SystemTime, UNIX_EPOCH};

/**
 * Format a datetime to a string.
 *
 * # Arguments
 * * `time` - The time to format.
 *
 * # Example
 * ```rust
 * use std::time::{SystemTime, UNIX_EPOCH};
 * use shell::utils::date::format_datetime;
 *
 * let time = UNIX_EPOCH;
 * let formatted = format_datetime(time);
 * ```
 */
pub fn format_datetime(time: SystemTime) -> String {
    let duration = time
        .duration_since(UNIX_EPOCH)
        .expect(TIME_WENT_BACKWARDS);

    let secs = duration.as_secs();

    let tm = secs_to_tm(secs);

    format!("{:3} {:2} {:02}:{:02}", month_to_str(tm.tm_mon), tm.tm_mday, tm.tm_hour, tm.tm_min)
}

/**
 * Convert seconds to a `Tm` struct.
 *
 * # Arguments
 * * `secs` - The seconds to convert.
 *
 * # Example
 * ```rust
 * use shell::utils::date::secs_to_tm;
 *
 * let tm = secs_to_tm(0);
 * ```
 */
fn secs_to_tm(secs: u64) -> Tm {
    const SECS_PER_DAY: u64 = 86400;
    const DAYS_PER_YEAR: u64 = 365;
    const DAYS_PER_LEAP_YEAR: u64 = 366;

    let mut days_since_epoch = secs / SECS_PER_DAY;
    let mut year = 1970;

    loop {
        let days_in_year = if is_leap_year(year) {
            DAYS_PER_LEAP_YEAR
        } else {
            DAYS_PER_YEAR
        };

        if days_since_epoch < days_in_year {
            break;
        }

        days_since_epoch -= days_in_year;
        year += 1;
    }

    let mut month = 0;
    let mut day = days_since_epoch + 1;

    for (i, &days_in_month) in DAYS_IN_MONTH[..12].iter().enumerate() {
        let days_in_month = if i == 1 && is_leap_year(year) {
            days_in_month + 1
        } else {
            days_in_month
        };

        if day <= days_in_month {
            month = i;
            break;
        }

        day -= days_in_month;
    }

    let secs_in_day = secs % SECS_PER_DAY;
    let hour = (secs_in_day / 3600) % 24;
    let min = (secs_in_day % 3600) / 60;

    Tm {
        tm_mon: month as i32,
        tm_mday: day as i32,
        tm_hour: hour as i32,
        tm_min: min as i32,
    }
}

/**
 * A struct representing a time.
 */
pub struct Tm {
    pub tm_mon: i32,
    pub tm_mday: i32,
    pub tm_hour: i32,
    pub tm_min: i32,
}

const DAYS_IN_MONTH: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/**
 * Check if a year is a leap year.
 *
 * # Arguments
 * * `year` - The year to check.
 *
 * # Example
 * ```rust
 * use shell::utils::date::is_leap_year;
 *
 * let is_leap = is_leap_year(2020);
 * ```
 */
fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/**
 * Convert a month to a string.
 *
 * # Arguments
 * * `month` - The month to convert.
 *
 * # Example
 * ```rust
 * use shell::utils::date::month_to_str;
 *
 * let month = 0;
 * let month_str = month_to_str(month);
 * ```
 */
fn month_to_str(month: i32) -> &'static str {
    match month {
        0 => "Jan",
        1 => "Feb",
        2 => "Mar",
        3 => "Apr",
        4 => "May",
        5 => "Jun",
        6 => "Jul",
        7 => "Aug",
        8 => "Sep",
        9 => "Oct",
        10 => "Nov",
        11 => "Dec",
        _ => NOTHING,
    }
}