// date_and_time
// (c) 2024 by markus dot mueller dot 73 at hotmail dot de
// small crate to get some rudimentary date and time calculations
// the license details are in the main library file.

use crate::date_and_time::time::*;
#[cfg(target_os = "linux")]
use libc::{localtime_r, time, time_t, tm};

#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::SYSTEMTIME;
use windows_sys::Win32::System::SystemInformation::GetLocalTime;
use windows_sys::Win32::System::Time::{GetTimeZoneInformation, TIME_ZONE_INFORMATION};

pub fn get_local_time() -> Time {
    let mut result = Time::new();
    #[cfg(target_os = "linux")]
    unsafe {
        let mut t: time_t = 0;
        let t_ptr: *mut time_t = &mut t;
        t = time(t_ptr);
        let mut ltm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: std::ptr::null(),
        };
        let ltm_ptr: *mut tm = &mut ltm;
        localtime_r(&t, ltm_ptr);
        result.set(ltm.tm_hour as i32, ltm.tm_min as i8, ltm.tm_sec as i8);
    }
    #[cfg(target_os = "windows")]
    unsafe {
        let mut lt = SYSTEMTIME {
            wYear: 0,
            wMonth: 0,
            wDayOfWeek: 0,
            wDay: 0,
            wHour: 0,
            wMinute: 0,
            wSecond: 0,
            wMilliseconds: 0,
        };
        let lt_ptr: *mut SYSTEMTIME = &mut lt;
        GetLocalTime(lt_ptr);
        result.set(lt.wHour as i32, lt.wMinute as i8, lt.wSecond as i8);
    }
    result
}

pub fn is_daylight_saving() -> bool {
    let mut result: bool = false;
    #[cfg(target_os = "linux")]
    unsafe {
        let mut t: time_t = 0;
        let t_ptr: *mut time_t = &mut t;
        t = time(t_ptr);
        let mut ltm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: std::ptr::null(),
        };
        let ltm_ptr: *mut tm = &mut ltm;
        localtime_r(&t, ltm_ptr);
        if ltm.tm_isdst != 0 {
            result = true;
        }
    }
    #[cfg(target_os = "windows")]
    unsafe {
        let mut tzi = TIME_ZONE_INFORMATION {
            Bias: 0,
            StandardName: [0; 32],
            StandardDate: SYSTEMTIME {
                wYear: 0,
                wMonth: 0,
                wDayOfWeek: 0,
                wDay: 0,
                wHour: 0,
                wMinute: 0,
                wSecond: 0,
                wMilliseconds: 0,
            },
            StandardBias: 0,
            DaylightName: [0; 32],
            DaylightDate: SYSTEMTIME {
                wYear: 0,
                wMonth: 0,
                wDayOfWeek: 0,
                wDay: 0,
                wHour: 0,
                wMinute: 0,
                wSecond: 0,
                wMilliseconds: 0,
            },
            DaylightBias: 0,
        };
        let tzi_ptr: *mut TIME_ZONE_INFORMATION = &mut tzi;
        let is_dst = GetTimeZoneInformation(tzi_ptr);
        if is_dst == 2 {
            result = true;
        }
    }
    result
}

pub fn get_gmt_offset() -> i8 {
    let result: i8;
    #[cfg(target_os = "linux")]
    unsafe {
        let mut t: time_t = 0;
        let t_ptr: *mut time_t = &mut t;
        t = time(t_ptr);
        let mut ltm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: std::ptr::null(),
        };
        let ltm_ptr: *mut tm = &mut ltm;
        localtime_r(&t, ltm_ptr);
        result = ltm.tm_gmtoff as i8;
    }
    #[cfg(target_os = "windows")]
    unsafe {
        let mut tzi = TIME_ZONE_INFORMATION {
            Bias: 0,
            StandardName: [0; 32],
            StandardDate: SYSTEMTIME {
                wYear: 0,
                wMonth: 0,
                wDayOfWeek: 0,
                wDay: 0,
                wHour: 0,
                wMinute: 0,
                wSecond: 0,
                wMilliseconds: 0,
            },
            StandardBias: 0,
            DaylightName: [0; 32],
            DaylightDate: SYSTEMTIME {
                wYear: 0,
                wMonth: 0,
                wDayOfWeek: 0,
                wDay: 0,
                wHour: 0,
                wMinute: 0,
                wSecond: 0,
                wMilliseconds: 0,
            },
            DaylightBias: 0,
        };
        let tzi_ptr: *mut TIME_ZONE_INFORMATION = &mut tzi;
        let is_dst = GetTimeZoneInformation(tzi_ptr);
        let mut bias = tzi.Bias;
        if is_dst == 2 {
            bias += tzi.DaylightBias;
        }
        result = (bias * -1) as i8;
    }
    result
}
