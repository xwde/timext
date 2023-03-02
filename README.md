## xwde: timext

[![Build Status][action-badge]][action-url]
[![Crate Docs][docs-badge]][docs-url]
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

- Introduces `CalendarDuration` and extends `time::Date`,
  `time::OffsetDateTime`, `time::PrimitiveDateTime`, and with several methods to
  enable arithmetic operations related to months and years. Additionally,
  attaches conversion methods to `i64` and `f64` to improve ease of use.

```rust
use time::{Date, Month::*};
use timext::ext::NumericCalendarDuration;

fn main() {
    let d0 = Date::from_calendar_date(2023, January, 31);
    let d1 = Date::from_calendar_date(2023, February, 28);
    assert_eq!(d0.unwrap() + 1.months(), d1.unwrap());

    let d0 = Date::from_calendar_date(2024, February, 29);
    let d1 = Date::from_calendar_date(2025, February, 28);
    assert_eq!(d0.unwrap() + 1.years(), d1.unwrap());
}
```

- Implements its own `time::Time`, `time::Date`, `time::PrimitiveDateTime`, and
  `time::OffsetDateTime` types, that are convertable from/to original, but allow
  incomplete time formats e.g. `xx:24:xx.845`, `1998-xx-02` or `2016-08 14:xx`.
  Also extends them with parsing & formatting capabilities.

> **Warning** : `parsing` & `formatting` are not yet implemented.

```rust
use time::{Date, Month::*};
use timext::{InComplete, InDate};

fn main() {
    let d0 = Date::from_calendar_date(2023, January, 28);
    let d1 = InDate::from_calendar_date(None, None, Some(28));
    let d1 = d1.unwrap();

    let d1 = d1.replace_year(Some(2023)).unwrap();
    let d1 = d1.replace_month(Some(January)).unwrap();
    assert_eq!(d0.unwrap(), d1.into_complete().unwrap());
}
```

#### Links

- [time-rs/time](https://github.com/time-rs/time/)
