// date_and_time
// (c) 2024 by markus dot mueller dot 73 at hotmail dot de
// small crate to get some rudimentary date and time calculations
// the license details are in the main library file.
use std::time::SystemTime;

// These constant arrays are private and only used for calculatons.
const LAST_DAY_OF_MONTH_LEAP: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LAST_DAY_OF_MONTH_COMMON: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

// These constants are placeholders, Rust has no direct methods to get the local
// date and time format of the running system.
const WEEKDAY_FULL: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];
const WEEKDAY_ABBREVIATE: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

const MONTH_NAME_FULL: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const MONTH_NAME_ABBREVIATE: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// The Date structure can build/filled with with the functions ```new()```, ```set()```,
/// ```from()``` and ```from_system_date()```. An ```as_strinng()``` function is
/// available to print the date.
///
/// Take a look further into the methods.
///
/// The structure owns the traits ```Copy```, ```Clone``` and ```PartialEq```. so you can
/// compare two dates if they are equal or not.
///
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Date {
    pub y: i32,
    pub m: u8,
    pub d: u8,
}

#[allow(dead_code)]
impl Date {
    /// ```new()``` creates a ```Date``` structure with the date 1st January 0 (year Null).
    pub fn new() -> Date {
        Date { d: 1, m: 1, y: 0 }
    }
    /// ```from(day, month, year)``` creates a ```Date``` structure with the given date.
    /// The new ```Date``` will be checked for validity, if it was invalid, the returned date
    /// will be ```Date{d: 0, m: 0, y: 0}```. You can check against the day or month if you
    /// got a valid date.
    pub fn from(day: u8, month: u8, year: i32) -> Date {
        let new_date = Date {
            d: day,
            m: month,
            y: year,
        };
        if is_date_valid(&new_date) == false {
            return Date { d: 0, m: 0, y: 0 };
        }
        new_date
    }
    /// ```from_system_date()``` creates a ```Date``` structure with the current system date
    /// derived from UTC time.
    pub fn from_system_date() -> Date {
        let sys_secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        get_date_from_days((sys_secs / 86_400) as i64)
    }
    // TODO pub fn from_local_date() -> Date
    /// ```set(day, month, year)``` modified the ```Date``` structure to the given date.
    /// The new ```Date``` will be checked for validity, if it was invalid, the returned date
    /// will be ```Date{d: 0, m: 0, y: 0}```. You can check against the day or month if you
    /// got a valid date.
    pub fn set(&mut self, day: u8, month: u8, year: i32) {
        self.d = day;
        self.m = month;
        self.y = year;
        if is_date_valid(self) == false {
            self.d = 0;
            self.m = 0;
            self.y = 0;
        }
    }
    /// ```get_day_of_year()``` gets the day in year from the Date structure as a number.
    pub fn get_day_of_year(&self) -> u32 {
        let mut d: u32 = self.d as u32;
        let m: usize = self.m as usize;
        if is_leap_year(self.y) {
            for n in 0..m - 1 {
                d += LAST_DAY_OF_MONTH_LEAP[n as usize] as u32;
            }
        } else {
            for n in 0..m - 1 {
                d += LAST_DAY_OF_MONTH_COMMON[n as usize] as u32;
            }
        }
        d
    }
    /// ```get_iso_week_of_year()``` gets the number of the week in the year of the
    /// Date structure as a number. This is the ISO 8601 weeknumber. The ISO weeks
    /// starts with Monday.
    pub fn get_iso_week_of_year(&self) -> u8 {
        let jan1 = Date {
            d: 1,
            m: 1,
            y: self.y,
        };
        let current: Date = *self;
        let days_jan1 = get_days_from_date(&jan1);
        let days_date = get_days_from_date(&current);
        let weeks = (days_date - days_jan1) / 7 + 1;
        weeks as u8
    }
    /// ```get_week_of_year()``` gets the number of the week in the year of the
    /// Date structure. The parameter ```firstweekday``` has to be ```== 0``` if the
    /// week starts with Sundays or has to be ```!= 0``` if the week starts with
    /// Mondays. The result can differs to the ISO week.
    pub fn get_week_of_year(&self, firstweekday: u8) -> u8 {
        let mut wday: u32 = self.get_weekday() as u32;
        if firstweekday != 0 {
            if wday == 0 {
                wday = 6;
            } else {
                wday -= 1;
            }
        }
        let yday: u32 = self.get_day_of_year();
        let result: u32 = (yday + 7 - wday) / 7;
        result as u8
    }
    /// ```get_weekday()``` gets a number for the day in the week of the Date structure.
    /// From 0 = Sunday to 6 = Saturday
    pub fn get_weekday(&self) -> u8 {
        let days: i64 = get_days_from_date(&self);
        let weekday = if days >= -4 {
            (days + 4) % 7
        } else {
            (days + 5) % 7 + 6
        };
        weekday as u8
    }
    /// ```diff_in_days(&other_date)``` gets the difference between the to dates in days.
    pub fn diff_in_days(&self, date: &Date) -> i64 {
        let d1_days = get_days_from_date(&self);
        let d2_days = get_days_from_date(&date);
        d2_days - d1_days
    }
    /// ```add_date(&other_ate)``` adds the ```&other_date``` to the date and returns a new
    /// ```Date``` structure.
    ///
    /// The new ```Date``` will be checked for validity, if it was invalid, the returned date
    /// will be ```Date{d: 0, m: 0, y: 0}```. You can check against the day or month if you
    /// got a valid date.
    ///
    pub fn add_date(&self, date: &Date) -> Date {
        let d1_days = get_days_from_date(&self);
        let d2_days = get_days_from_date(&date);
        let new_date: Date = get_date_from_days(d1_days + d2_days);
        if is_date_valid(&new_date) == false {
            return Date { d: 0, m: 0, y: 0 };
        }
        new_date
    }
    /// ```sub_date(&other_ate)``` subs the ```&other_date``` from the date and returns a new
    /// ```Date``` structure.
    ///
    /// The new ```Date``` will be checked for validity, if it was invalid, the returned date
    /// will be ```Date{d: 0, m: 0, y: 0}```. You can check against the day or month if you
    /// got a valid date.
    ///
    pub fn sub_date(&self, date: &Date) -> Date {
        let d1_days = get_days_from_date(&self);
        let d2_days = get_days_from_date(&date);
        let new_date: Date = get_date_from_days(d1_days - d2_days);
        if is_date_valid(&new_date) == false {
            return Date { d: 0, m: 0, y: 0 };
        }
        new_date
    }
    /// ```add_years(years as u32)``` adds the years to the date and returns a new
    /// ```Date``` structure.
    pub fn add_years(&self, years: u32) -> Date {
        let new_date = Date {
            d: self.d,
            m: self.m,
            y: self.y + years as i32,
        };
        new_date
    }
    /// ```sub_years(years as u32)``` substract the years from the date and returns a new
    /// ```Date``` structure.
    pub fn sub_years(&self, years: u32) -> Date {
        let new_date = Date {
            d: self.d,
            m: self.m,
            y: self.y - years as i32,
        };
        new_date
    }
    /// ```add_months(months as u32)``` adds the months to the date and returns a new
    /// ```Date``` structure.
    pub fn add_months(&self, months: u32) -> Date {
        let mon: u32 = self.m as u32 + months;
        let mut new_date = Date {
            d: self.d,
            m: self.m,
            y: self.y,
        };
        if mon > 12 {
            new_date.y += mon as i32 / 12;
            new_date.m = (mon % 12) as u8;
        } else {
            new_date.m = mon as u8;
        }
        new_date
    }
    /// ```sub_months(years as u32)``` substract the months from the date and returns a new
    /// ```Date``` structure.
    pub fn sub_months(&self, months: u32) -> Date {
        let mut new_date = Date {
            d: self.d,
            m: self.m,
            y: self.y,
        };
        let mon: i32;
        if months > 12 {
            new_date.y -= months as i32 / 12;
            mon = months as i32 % 12;
        } else {
            mon = months as i32;
        }
        if mon >= new_date.m as i32 {
            new_date.y -= 1;
            new_date.m = 12 - (mon - new_date.m as i32) as u8;
        } else {
            new_date.m -= mon as u8;
        }
        new_date
    }
    /// ```add_days(days as u64)``` adds the days to the date and returns a new
    /// ```Date``` structure.
    ///
    /// The new ```Date``` will be checked for validity, if it was invalid, the returned date
    /// will be ```Date{d: 0, m: 0, y: 0}```. You can check against the day or month if you
    /// got a valid date.
    ///
    pub fn add_days(&self, days: u64) -> Date {
        let d_days = get_days_from_date(&self);
        let new_date: Date = get_date_from_days(d_days + days as i64);
        if is_date_valid(&new_date) == false {
            return Date { d: 0, m: 0, y: 0 };
        }
        new_date
    }
    /// ```sub_days(years as u32)``` substract the days from the date and returns a new
    /// ```Date``` structure.
    ///
    /// The new ```Date``` will be checked for validity, if it was invalid, the returned date
    /// will be ```Date{d: 0, m: 0, y: 0}```. You can check against the day or month if you
    /// got a valid date.
    ///
    pub fn sub_days(&self, days: u64) -> Date {
        let d_days = get_days_from_date(&self);
        let new_date: Date = get_date_from_days(d_days - days as i64);
        if is_date_valid(&new_date) == false {
            return Date { d: 0, m: 0, y: 0 };
        }
        new_date
    }
    /// ```as_string()``` gets the ```Date``` structure as a string in the format: YYYY-MM-DD
    /// (ISO 8601 date format)
    pub fn as_string(&self) -> String {
        String::from(format!("{:04}-{:02}-{:02}", self.y, self.m, self.d))
    }
    /// ```as_formated_string(date_format)``` gets the ```Date``` structure as a string in
    /// the ```date_format``` parameter.
    ///
    /// The format is similar to the C function strftime(), but not all placeholders area
    /// available. Here ist a list of the available ones:
    ///
    /// | Placeholder | Explanation |
    /// :-: | ----------- |
    /// | % | writes the literal %, The full conversion specification must be %%. |
    /// | n | writes newline character. |
    /// | t | writes horizontal tab character
    /// | Y | writes year as a decimal number, e.g. 2017. |
    /// | y | writes last 2 digits of year as a decimal number (range 00 - 99). |
    /// | C | writes first 2 digits of year as a decimal number (range 00 - 99) |
    /// | G | writes ISO 8601 week-based year. |
    /// | g | writes last 2 digits of ISO 8601 week-based year, i.e. the year that contains the specified week (range 00-99). |
    /// | b | writes abbreviated month name, e.g. Oct. |
    /// | B | writes full month name, e.g. October. |
    /// | m | writes month as a decimal number (range 01 - 12). |
    /// | U | writes week of the year as a decimal number (Sunday is the first day of the week) (range 00-53). |
    /// | V | writes ISO 8601 week of the year (range 01 - 53). In ISO 8601 weeks begin with Monday. |
    /// | W | writes week of the year as a decimal number (Monday is the first day of the week) (range 00-53). |
    /// | j | writes day of the year as a decimal number (range 001-366)
    /// | d | writes day of the month as a decimal number (range 01-31)
    /// | e | writes day of the month as a decimal number (range 1-31). Single digit is preceded by a space. |
    /// | a | writes abbreviated weekday name, e.g. Fri. |
    /// | A | writes full weekday name, e.g. Friday.
    /// | w | writes weekday as a decimal number, where Sunday is 0. |
    /// | u | writes weekday as a decimal number, where Monday is 1 (ISO 8601 format). |
    /// | D | equivalent to "%m/%d/%y" |
    /// | F | equivalent to "%Y-%m-%d" (the ISO 8601 date format) |
    ///
    /// The result of the week and month names are only in english atm.
    ///
    pub fn as_formated_string(&self, date_format: &str) -> String {
        let mut chars = date_format.chars();
        let mut result = String::default();
        while let Some(c) = chars.next() {
            if c == '%' {
                let Some(cn) = chars.next() else {
                    continue;
                };
                match cn {
                    '%' => result.push(c),
                    'a' => {
                        let wd: usize = self.get_weekday() as usize;
                        let s = String::from(format!("{}", WEEKDAY_ABBREVIATE[wd]));
                        result.push_str(&s)
                    }
                    'A' => {
                        let wd: usize = self.get_weekday() as usize;
                        let s = String::from(format!("{}", WEEKDAY_FULL[wd]));
                        result.push_str(&s)
                    }
                    'b' => {
                        let mn: usize = self.m as usize;
                        let s = String::from(format!("{}", MONTH_NAME_ABBREVIATE[mn]));
                        result.push_str(&s)
                    }
                    'B' => {
                        let mn: usize = self.m as usize;
                        let s = String::from(format!("{}", MONTH_NAME_FULL[mn]));
                        result.push_str(&s)
                    }
                    'C' => {
                        let s = String::from(format!("{:02}", self.y / 100));
                        result.push_str(&s)
                    }
                    'd' => {
                        let s = String::from(format!("{:02}", self.d));
                        result.push_str(&s)
                    }
                    'D' => {
                        let s = String::from(format!("{:02}/{:02}/{:02}", self.m, self.d, self.y));
                        result.push_str(&s)
                    }
                    'e' => {
                        let s = String::from(format!("{:2}", self.d));
                        result.push_str(&s)
                    }
                    'F' => {
                        let s = String::from(format!("{:04}-{:02}-{:02}", self.y, self.m, self.d));
                        result.push_str(&s)
                    }
                    'g' => {
                        let s = String::from(format!("{:02}", self.y));
                        result.push_str(&s)
                    }
                    'G' => {
                        let s = String::from(format!("{:04}", self.y));
                        result.push_str(&s)
                    }
                    'j' => {
                        let s = String::from(format!("{:03}", self.get_day_of_year()));
                        result.push_str(&s)
                    }
                    'm' => {
                        let s = String::from(format!("{:02}", self.m));
                        result.push_str(&s)
                    }
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'u' => {
                        let mut wd = self.get_weekday();
                        if wd == 0 {
                            wd = 7;
                        }
                        let s = String::from(format!("{}", wd));
                        result.push_str(&s)
                    }
                    'U' => {
                        let s = String::from(format!("{}", self.get_week_of_year(0)));
                        result.push_str(&s)
                    }
                    'V' => {
                        let s = String::from(format!("{}", self.get_iso_week_of_year()));
                        result.push_str(&s)
                    }
                    'W' => {
                        let s = String::from(format!("{}", self.get_week_of_year(1)));
                        result.push_str(&s)
                    }
                    'w' => {
                        let s = String::from(format!("{}", self.get_weekday()));
                        result.push_str(&s)
                    }
                    'y' => {
                        let s = String::from(format!("{:02}", self.y));
                        result.push_str(&s)
                    }
                    'Y' => {
                        let s = String::from(format!("{:04}", self.y));
                        result.push_str(&s)
                    }
                    _ => result.push(cn),
                }
            } else {
                result.push(c)
            }
        }
        result
    }
}

// Returns: true if year: i32 is a leap year, else false
fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        return true;
    }
    false
}
// Returns a Date in the civil calendar from the days: u64
fn get_date_from_days(days: i64) -> Date {
    let z: i64 = days + 719_468;
    let era = if z >= 0 {
        z / 146_097
    } else {
        (z - 146_096) / 146_097
    };
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let mon = if mp < 10 { mp + 3 } else { mp - 9 };
    let mut result = Date {
        d: day as u8,
        m: mon as u8,
        y: year as i32,
    };
    if mon <= 2 {
        result.y = (year + mon) as i32;
    }
    result
}

// // Returns the number of days from the seconds: u64
// fn get_days_from_seconds(seconds: u64) -> u64 {
//     let days = seconds / 86_400;
//     days
// }

// Returns the number of days since civil 1970-01-01.
// Negative values indicate days prior to 1970-01-01.
fn get_days_from_date(date: &Date) -> i64 {
    let mut y: i64 = date.y as i64;
    let m = date.m as i64;
    let d = date.d as i64;
    if m <= 2 {
        y -= 1;
    }
    let era: i64 = if y >= 0 { y / 400 } else { (y - 399) / 400 };
    let yoe: i64 = y - era * 400;
    let doy: i64 = if m > 2 {
        (153 * (m - 3) + 2) / 5 + d - 1
    } else {
        (153 * (m + 9) + 2) / 5 + d - 1
    };
    let doe: i64 = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let result: i64 = era * 146_097 + doe - 719_468;
    result
}

// Returns the maximal number days of the given month: u8 in the given year: i32
fn get_max_days_of_month(month: u8, year: i32) -> u8 {
    let m: usize = (month - 1) as usize;
    if is_leap_year(year) {
        LAST_DAY_OF_MONTH_LEAP[m]
    } else {
        LAST_DAY_OF_MONTH_COMMON[m]
    }
}

// Returns true if date: &Date is a valid date, else false
fn is_date_valid(date: &Date) -> bool {
    if date.m < 1 || date.m > 12 {
        return false;
    }
    let max_days = get_max_days_of_month(date.m, date.y);
    if date.d < 1 || date.d > max_days {
        return false;
    }
    true
}
