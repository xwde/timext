### timext

The collection of [time-rs/time](https://github.com/time-rs/time/) extensions
for calendar arithmetics, incomplete formats handling, imprecise time, and other
things `time` crate is not intended for.

- Introduces `timext:CalendarDuration` and extends `time::Date`,
  `time::PrimitiveDateTime`, and `time::OffsetDateTime` with several methods to
  enable arithmetic operations related to months and years. Additionally,
  attaches conversion methods to `i64` and `f64` to improve ease of use.

```rust
use time::{Date, Month};
use timext::ext::NumericCalendarDuration;

fn main() {
    let d0 = Date::from_calendar_date(2023, Month::January, 31).unwrap();
    let d1 = Date::from_calendar_date(2023, Month::February, 28).unwrap();
    assert_eq!(d0 + 1.months(), d1);

    let d0 = Date::from_calendar_date(2024, Month::February, 29).unwrap();
    let d1 = Date::from_calendar_date(2025, Month::February, 28).unwrap();
    assert_eq!(d0 + 1.years(), d1);
}
```

- Implements its own `time::Time`, `time::Date`, `time::PrimitiveDateTime`, and
  `time::OffsetDateTime` types, that are convertable and roughly compatible with
  original, but allow incomplete time formats e.g. `xx:24:xx.845`, `1998-xx-02`
  or `2016-08 14:xx`. Also extends them with parsing & formatting capabilities.

```rust
use time::{Date, Month};
use timext::{InComplete, InDate};

fn main() {
    let d0 = Date::from_calendar_date(2023, Month::January, 28).unwrap();
    let d1 = InDate::from_calendar_date(None, None, Some(28)).unwrap();
    let d1 = d1.replace_year(Some(2023)).unwrap();
    let d1 = d1.replace_month(Some(Month::January)).unwrap();
    assert_eq!(d0, d1.into_complete().unwrap());
}
```

#### Links

- [time-rs/time](https://github.com/time-rs/time/)
