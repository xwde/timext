### timext

The collection of [time-rs/time](https://github.com/time-rs/time/) extensions:

- Extends `time::Date`. `time::PrimitiveDateTime` and `time::OffsetDateTime` with
  several methods to enable arithmetic operations related to months and years.
  Additionally, attaches conversion methods to `i32`, `u32` and `f32` to improve
  ease of use.

```rust
use time::{Date, Month};

use crate::NumericMonthDuration;

fn main() {
    let d0 = Date::from_calendar_date(2023, Month::January, 31).unwrap();
    let d1 = Date::from_calendar_date(2023, Month::February, 28).unwrap();
    assert_eq!(d0 + 1.months(), d1);

    let d0 = Date::from_calendar_date(2024, Month::February, 29).unwrap();
    let d1 = Date::from_calendar_date(2025, Month::February, 28).unwrap();
    assert_eq!(d0 + 1.years(), d1);
}
```
