// date_and_time
// (c) 2024 by markus dot mueller dot 73 at hotmail dot de
// small crate to get some rudimentary date and time calculations
// the license details are in the main library file.
use std::time::SystemTime;

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
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Time {
    pub h: i32,
    pub m: i8,
    pub s: i8,
}

#[allow(dead_code)]
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
    /// ```as_string()``` gets the Time structure as a string in the format HH:MM:SS.
    pub fn as_string(&self) -> String {
        let s = String::from(format!("{:02}:{:02}:{:02}", self.h, self.m, self.s));
        s
    }
    /// ```as_formated_string(time_format)``` gets the ```Time``` structure as a string in
    /// the ```time_format``` parameter.
    ///
    /// The format is similar to the C function strftime(), but not all placeholders are
    /// available. Here ist a list of the available ones:
    ///
    /// | Placeholder | Explanation |
    /// :-: | ----------- |
    /// | % | writes the literal %, The full conversion specification must be %%. |
    /// | n | writes newline character. |
    /// | t | writes horizontal tab character
    /// | H | writes hour as a decimal number, 24 hour clock (range 00-23) |
    /// | I | writes hour as a decimal number, 12 hour clock (range 01-12) |
    /// | M | writes minute as a decimal number (range 00-59) |
    /// | p | writes localized a.m. or p.m. |
    /// | r | writes localized 12-hour clock time |
    /// | R | equivalent to "%H:%M" |
    /// | S | writes second as a decimal number (range 00-59) |
    /// | T | equivalent to "%H:%M:%S" (the ISO 8601 time format) |
    ///
    pub fn as_formated_string(&self, time_format: &str) -> String {
        let mut chars = time_format.chars();
        let mut result = String::default();
        while let Some(c) = chars.next() {
            if c == '%' {
                let Some(cn) = chars.next() else {
                    continue;
                };
                match cn {
                    '%' => result.push('%'),
                    'H' => {
                        let s = String::from(format!("{:02}", self.h));
                        result.push_str(&s)
                    }
                    'I' => {
                        let t = if self.h == 0 {
                            12
                        } else if self.h > 12 {
                            self.h - 12
                        } else {
                            self.h
                        };
                        let s = String::from(format!("{:02}", t));
                        result.push_str(&s)
                    }
                    'M' => {
                        let s = String::from(format!("{:02}", self.m));
                        result.push_str(&s)
                    }
                    'p' => {
                        if self.h >= 12 {
                            result.push_str("p.m.");
                        } else {
                            result.push_str("a.m.");
                        }
                    }
                    'r' => {
                        let mut hour = self.h;
                        if hour == 0 {
                            hour = 24
                        };
                        if hour > 12 {
                            hour -= 12
                        };
                        let h_12 = if self.h >= 12 { "PM" } else { "AM" };
                        let s = String::from(format!(
                            "{:2}:{:02}:{:02} {}",
                            hour, self.m, self.s, h_12
                        ));
                        result.push_str(&s)
                    }
                    'R' => {
                        let s = String::from(format!("{:02}:{:02}", self.h, self.m));
                        result.push_str(&s)
                    }
                    'S' => {
                        let s = String::from(format!("{:02}", self.s));
                        result.push_str(&s)
                    }
                    'T' => {
                        let s = String::from(format!("{:02}:{:02}:{:02}", self.h, self.m, self.s));
                        result.push_str(&s)
                    }
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    _ => result.push(cn),
                };
            } else {
                result.push(c)
            }
        }
        result
    }
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

// Returns true if the time is valid, else false
fn is_time_valid(t: &Time) -> bool {
    if t.m >= 0 && t.m < 60 && t.s >= 0 && t.s < 60 {
        return true;
    }
    false
}
