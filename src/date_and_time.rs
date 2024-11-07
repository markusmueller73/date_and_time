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
//! To use the library only import it: ```use crate::date_and_time::*;```. You can also import only
//! date (```use crate::date_and_time::date::*;```) or time (```use crate::date_and_time::time::*;```)
//! calculations.
//!

pub mod date;
pub mod local;
pub mod time;

// TEST area
#[cfg(test)]
mod tests {
    use crate::date_and_time::time::*;

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

    use crate::date_and_time::date::*;
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
