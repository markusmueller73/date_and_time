// date_and_time
// (c) 2024 by markus dot mueller dot 73 at hotmail dot de
// small crate to get some rudimentary date and time calculations
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the “Software”), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Most of the date calculations was taken from here: https://howardhinnant.github.io/date_algorithms.html
// Thank you Mr.Hinnant
//

//! # Date_And_Time
//! is a small library for rudimentary date and time calculations.
//! (c) 2024 by markus dot mueller dot 73 at hotmail dot de
//!
//! The date calculation works bevore the Linux epoch time (01. Jan 1970). The time calculations
//! are not reduced to the (clock) time, you can calc in addition to the default 24h clock.
//!
//! For the Date and the Time are two seperate easy structures (see below).
//! Suggestions are welcome.
//!
//! To use the library only import it: ```use date_and_time::*;```.
//!
use std::time::SystemTime;

// These constant arrays are private and only used for calculatons.
const LAST_DAY_OF_MONTH_LEAP: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LAST_DAY_OF_MONTH_COMMON: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

//const DATE_FORMAT = String.from("{:02}.{:02}.{:04}");     // FIXME didn't work
//const TIME_FORMAT: 'static &str = "{:02}:{:02}:{:02}";    // FIXME didn't work either

/// The Date structure can build/filled with with the functions ```new()```, ```set()```,
/// ```from()``` and ```from_system_date()```. An ```as_strinng()``` function is
/// available to print the date.
///
/// Take a look further into the methods.
///
/// The structure owns the traits ```Copy```, ```Clone``` and ```PartialEq```. so you can
/// compare two dates if they are equal or not.
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Date {
    pub d: u8,
    pub m: u8,
    pub y: i32,
}

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
    /// ```get_week_number_of_year()``` gets the number of the week in the year of the
    /// Date structure as a number. This is the ISO weeknumber. The ISO weeks starts
    /// with Monday.
    pub fn get_week_number_of_year(&self) -> u8 {
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
    /// ```as_string()``` gets the Date structure as a string in the format DD.MM.YYYY.
    pub fn as_string(&self) -> String {
        let s = String::from(format!("{:02}.{:02}.{:04}", self.d, self.m, self.y));
        // let s = String::from(format!(DATE_FORMAT, self.d, self.m, self.y)); <- FIXME need a 'string literal'
        s
    }
}

/// The Time structure can build/filled with with the functions ```new()```, ```set()```,
/// ```from()```,  ```from_seconds()``` and ```from_system_date()```. An ```as_strinng()``` function is
/// available to print the time.
///
/// Take a look further into the methods.
///
/// This time structure is not only for the clock. You can use it also for time counting.
/// In validity checks the hours didn't checked at all. Only minutes and seconds get checked
/// and only in a few methods.
///
/// The structure owns the traits ```Copy```, ```Clone``` and ```PartialEq```. so you can
/// compare two times if they are equal or not.
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Time {
    pub h: i32,
    pub m: i8,
    pub s: i8,
}

impl Time {
    /// ```new()``` creates a ```Time``` structure with this time 0:00:00.
    pub fn new() -> Time {
        Time { h: 0, m: 0, s: 0 }
    }
    /// ```from(hour, minute, second)``` creates a ```Time``` structure with
    /// the time from the parameters .
    ///
    /// The new ```Time``` will be checked for validity, if it was invalid, the returned time
    /// will be ```Time{h: 0, m: -1, s: -1}```. You can check against the minutes or seconds if
    ///  you got a valid time.
    ///
    pub fn from(hour: i32, minute: i8, second: i8) -> Time {
        let t = Time {
            h: hour,
            m: minute,
            s: second,
        };
        if is_time_valid(&t) == false {
            return Time { h: 0, m: -1, s: -1 };
        }
        t
    }
    /// ```from_seconds(seconds)``` creates a new ```Time``` structure from the ```seconds```
    pub fn from_seconds(seconds: i64) -> Time {
        secs_to_time(seconds)
    }
    /// ```from_system_clock()``` creates a new ```Time``` structure from the systems clock.
    /// The result is in UTC time and will probably be different from your local time.
    pub fn from_system_clock() -> Time {
        let sys_secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let secs: i64 = sys_secs as i64 % 86_400;
        secs_to_time(secs)
    }
    // TODO pub fn from_local_clock() -> Time
    /// ```set(hour, minute, second)``` modifies your ```Time``` structure to the hour, minute
    /// and second from the parameters.
    ///
    /// The new ```Time``` will be checked for validity, if it was invalid, the returned time
    /// will be ```Time{h: 0, m: -1, s: -1}```. You can check against the minutes or seconds if
    ///  you got a valid time.
    ///
    pub fn set(&mut self, hour: i32, minute: i8, second: i8) {
        self.h = hour;
        self.m = minute;
        self.s = second;
        if is_time_valid(self) == false {
            self.h = 0;
            self.m = -1;
            self.s = -1;
        }
    }
    /// ```as_seconds()``` returns the seconds from your ```Time``` structure.
    pub fn as_seconds(&self) -> u32 {
        self.h as u32 * 3_600 + self.m as u32 * 60 + self.s as u32
    }
    /// ```as_float()``` returns the your ```Time``` structure as a float.
    /// For the calculation, the seconds and minutes are each extrapolated to 100 values.
    pub fn as_float(&self) -> f32 {
        let h: f32 = self.h as f32;
        let m: f32 = self.m as f32 / 60.0 * 100.0;
        dbg!(m);
        let s: f32 = self.s as f32 / 60.0 * 100.0;
        dbg!(s);
        h + m / 100.0 + s / 10_000.0
    }
    /// ```as_string()``` gets the Time structure as a string in the format HH:MM:SS.
    pub fn as_string(&self) -> String {
        let s = String::from(format!("{:02}:{:02}:{:02}", self.h, self.m, self.s));
        s
    }
    /// ```diff_in_seconds(&other_time)``` gets the difference between the two times in seconds.
    pub fn diff_in_seconds(&self, t: &Time) -> i64 {
        let diff_secs: i64 = time_to_secs(t) as i64 - time_to_secs(self) as i64;
        diff_secs
    }
    /// ```add_time(&other_time)``` adds the ```&other_time``` to the time and returns a new
    /// ```Time``` structure.
    pub fn add_time(&self, time: &Time) -> Time {
        let s: i64 = time_to_secs(self) as i64 + time_to_secs(time) as i64;
        secs_to_time(s)
    }
    /// ```sub_time(&other_time)``` substract the ```&other_time``` from the time and returns
    /// a new ```Time``` structure.
    pub fn sub_time(&self, time: &Time) -> Time {
        let s: i64 = time_to_secs(self) as i64 - time_to_secs(time) as i64;
        secs_to_time(s)
    }
    /// ```add_hours(hours)``` adds the ```hours``` to the time and returns a new
    /// ```Time``` structure.
    pub fn add_hours(&self, hours: i64) -> Time {
        let s: i64 = time_to_secs(self) as i64 + hours * 3_600;
        secs_to_time(s)
    }
    /// ```sub_hours(hours)``` substract the ```hours``` from the time and returns a new
    /// ```Time``` structure.
    /// It is possible to get a negative result.
    pub fn sub_hours(&self, hours: i64) -> Time {
        let s: i64 = time_to_secs(self) as i64 - hours * 3_600;
        secs_to_time(s)
    }
    /// ```add_minutes(minutes)``` adds the ```minutes``` to the time and returns a new
    /// ```Time``` structure.
    pub fn add_minutes(&self, minutes: i64) -> Time {
        let s: i64 = time_to_secs(self) as i64 + minutes * 60;
        secs_to_time(s)
    }
    /// ```sub_minutes(minutes)``` substract the ```minutes``` from the time and returns a new
    /// ```Time``` structure.
    /// It is possible to get a negative result.
    pub fn sub_minutes(&mut self, minutes: i64) -> Time {
        let s: i64 = time_to_secs(self) as i64 - minutes * 60;
        secs_to_time(s)
    }
    /// ```add_seconds(seconds)``` adds the ```seconds``` to the time and returns a new
    /// ```Time``` structure.
    pub fn add_seconds(&mut self, seconds: i64) -> Time {
        let s: i64 = time_to_secs(self) as i64 + seconds;
        secs_to_time(s)
    }
    /// ```sub_seconds(seconds)``` substract the ```seconds``` from the time and returns a new
    /// ```Time``` structure.
    /// It is possible to get a negative result.
    pub fn sub_seconds(&mut self, seconds: i64) -> Time {
        let s: i64 = time_to_secs(self) as i64 - seconds;
        secs_to_time(s)
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

// Returns the time in the Time structure in seconds
fn time_to_secs(t: &Time) -> u32 {
    t.h as u32 * 3_600 + t.m as u32 * 60 + t.s as u32
}

// Returns a Time structure of the give secs: i64, the time in the Time structure is
// always positive
fn secs_to_time(secs: i64) -> Time {
    let mut sec = secs;
    let hrs = sec / 3_600;
    sec -= hrs * 3_600;
    let min = sec / 60;
    sec -= min * 60;
    Time {
        h: hrs as i32,
        m: min as i8,
        s: sec as i8,
    }
}

fn is_time_valid(t: &Time) -> bool {
    if t.m >= 0 && t.m < 60 && t.s >= 0 && t.s < 60 {
        return true;
    }
    false
}

// TEST area
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_from_seconds() {
        let result = Time::from_seconds(9000);
        assert_eq!(result.h, 2);
        assert_eq!(result.m, 30);
    }

    #[test]
    fn test_time_difference() {
        let t1 = Time::from(21, 30, 45);
        let t2 = Time::from(21, 29, 15);
        assert_eq!(t1.diff_in_seconds(&t2), -90);
    }

    #[test]
    fn test_add_time() {
        let t1 = Time::from(18, 00, 00);
        let t2 = Time::from(0, 30, 0);
        assert_eq!(t1.add_time(&t2), Time::from(18, 30, 0));
    }

    #[test]
    fn test_date_set_and_from() {
        let mut d1 = Date::new();
        d1.set(22, 6, 2024);
        let d2 = Date::from(22, 6, 2024);
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_invalid_date() {
        let d1 = Date::from(29, 2, 1985);
        assert_eq!(d1, Date { d: 0, m: 0, y: 0 });
    }
}
