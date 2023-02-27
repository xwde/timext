## xwde: timext

[![Build Status][action-badge]][action-url]
[![Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]

[action-badge]: https://img.shields.io/github/actions/workflow/status/xwde/timext/build.yaml?branch=main&label=build&logo=github&style=for-the-badge
[action-url]: https://github.com/xwde/timext/actions/workflows/build.yaml
[crates-badge]: https://img.shields.io/crates/v/timext.svg?logo=rust&style=for-the-badge
[crates-url]: https://crates.io/crates/timext
[docs-badge]: https://img.shields.io/docsrs/timext?logo=Docs.rs&style=for-the-badge
[docs-url]: http://docs.rs/timext

> **Warning** : The library is in active development. Expect breaking changes.

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

#### Links

- [time-rs/time](https://github.com/time-rs/time/)
