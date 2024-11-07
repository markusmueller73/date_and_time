# Date_And_Time
(c) 2024 by markus dot mueller dot 73 at hotmail dot de

Is a small library for rudimentary date and time calculations.

The date calculation works bevore the Linux epoch time (01. Jan 1970). The time calculations are not reduced to 24 hours, you can calc with positive or negative values. A documentation is per ```cargo doc``` available. For Date and Time are two seperate easy structures you can use in your code. You can import the Date, the Time and a LocalTime crate separatly. The LocalTime depends on the Time crate.

The new LocalTime functions are unsafe functions from the OS depended API. At this time only Linux and Windows are available. While MacOS is using the Standart C library, the Linux code should work for MacOS too.

Suggestions are welcome.

## Usage:
Only import the library: ```use date_and_time::*;``` to get success to the whole library. Or import ```use date_and_time::date::*;``` to get only the Date features.

## Examples:
Using the Date structure for the actual date:
```
let today = Date::from_system_date();
println!("Date: {}", today.as_formated_string("%F"));
```
Using the Date structure for calculations:
```
let christmas = Date::from(25,12,2024);
let today = Date::get_system_date();
if today == christmas {
    println!("It's christmas eve!");
} else {
    println!("In {} days it's christmas!", today.diff_in_days(&christmas));
}
```
Using Time calculations:
```
let start_time = Time::from_seconds(0);
// do somthing ... and store a counter in: let counted_seconds
let end_time = Time::from(counted_seconds);
println!("The code part needs {} second(s).", start_time.diff_in_seconds(&end_time);
```
This code can be produced with the standart library (```duration```), but it's only an example.
```
let current_time = Time::from_system_clock();
println!("I have 55 minutes to finish, then it's {} o'clock.", current_time.add_minutes(55));
```

