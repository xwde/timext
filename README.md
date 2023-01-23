### timext

The collection of [time-rs/time](https://github.com/time-rs/time/) extensions
for imprecise time, incomplete formats and other things `time` crate is not
intended for.

- Extends `time::Date`, `time::PrimitiveDateTime`, and `time::OffsetDateTime`
  with several methods to enable arithmetic operations related to months and
  years. Additionally, attaches conversion methods to `i64` and `f64` to improve
  ease of use.

```rust
use time::{Date, Month};
use timext::ext::NumericMonthDuration;

fn main() {
    let d0 = Date::from_calendar_date(2023, Month::January, 31).unwrap();
    let d1 = Date::from_calendar_date(2023, Month::February, 28).unwrap();
    assert_eq!(d0 + 1.months(), d1);

    let d0 = Date::from_calendar_date(2024, Month::February, 29).unwrap();
    let d1 = Date::from_calendar_date(2025, Month::February, 28).unwrap();
    assert_eq!(d0 + 1.years(), d1);
}
```

- Implements its own time types, that are roughly compatible with original types
  and allow incomplete or partial time formats e.g. `xx:24:xx.845` `1998-xx-02`
  or `1998-08 14:xx`.

```rust
fn main() {}
```
