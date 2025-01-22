use std::time::{SystemTime, UNIX_EPOCH};

pub fn format_datetime(time: SystemTime) -> String {
    let duration = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let secs = duration.as_secs();

    let tm = secs_to_tm(secs);

    format!("{:3} {:2} {:02}:{:02}", month_to_str(tm.tm_mon), tm.tm_mday, tm.tm_hour, tm.tm_min)
}

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

    // Calculer le mois et le jour
    let mut month = 0;
    let mut day = days_since_epoch + 1; // Les jours commencent à 1

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

    // Calculer l'heure, la minute et la seconde
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

pub struct Tm {
    pub tm_mon: i32,
    pub tm_mday: i32,
    pub tm_hour: i32,
    pub tm_min: i32,
}

const DAYS_IN_MONTH: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

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
        _ => "???",
    }
}