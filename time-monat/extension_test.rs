#[cfg(test)]
mod tests {
    use time::Date;
    use time::Month::*;

    use crate::{MonthDuration, MonthExtension};

    #[test]
    fn sub_one() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let d1 = Date::from_calendar_date(2022, December, 1).unwrap();
        let md = MonthDuration::months(-12 + -1);
        let rs = d0.saturating_calendar_add(md);
        assert_eq!(rs, d1);
    }

    #[test]
    fn sub_many() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let d1 = Date::from_calendar_date(2019, December, 1).unwrap();
        let md = MonthDuration::months(-48 + -1);
        let rs = d0.saturating_calendar_add(md);
        assert_eq!(rs, d1);
    }

    #[test]
    fn sub_max() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let d1 = Date::from_calendar_date(2023, February, 1).unwrap();
        let md = MonthDuration::months(-11);
        let rs = d0.saturating_calendar_add(md);
        assert_eq!(rs, d1);
    }

    #[test]
    fn add_one() {
        let d0 = Date::from_calendar_date(2024, December, 1).unwrap();
        let d1 = Date::from_calendar_date(2026, January, 1).unwrap();
        let md = MonthDuration::months(12 + 1);
        let rs = d0.saturating_calendar_add(md);
        assert_eq!(rs, d1);
    }

    #[test]
    fn add_many() {
        let d0 = Date::from_calendar_date(2024, December, 1).unwrap();
        let d1 = Date::from_calendar_date(2029, January, 1).unwrap();
        let md = MonthDuration::months(48 + 1);
        let rs = d0.saturating_calendar_add(md);
        assert_eq!(rs, d1);
    }

    #[test]
    fn add_max() {
        let d0 = Date::from_calendar_date(2024, February, 1).unwrap();
        let d1 = Date::from_calendar_date(2025, January, 1).unwrap();
        let md = MonthDuration::months(11);
        let rs = d0.saturating_calendar_add(md);
        assert_eq!(rs, d1);
    }

    #[test]
    fn add_leap() {
        let d0 = Date::from_calendar_date(2024, February, 29).unwrap();
        let d1 = Date::from_calendar_date(2025, February, 28).unwrap();
        let md = MonthDuration::months(12);
        let rs = d0.saturating_calendar_add(md);
        assert_eq!(rs, d1);
    }

    #[test]
    fn sub_leap() {
        let d0 = Date::from_calendar_date(2024, February, 29).unwrap();
        let d1 = Date::from_calendar_date(2023, February, 28).unwrap();
        let md = MonthDuration::months(-12);
        let rs = d0.saturating_calendar_add(md);
        assert_eq!(rs, d1);
    }

    #[test]
    fn add_underflow() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let md = MonthDuration::months(i32::MIN);
        let rs = d0.saturating_calendar_add(md);
        assert_eq!(rs, Date::MIN);
    }

    #[test]
    fn sub_underflow() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let md = MonthDuration::months(i32::MAX);
        let rs = d0.saturating_calendar_sub(md);
        assert_eq!(rs, Date::MIN);
    }

    #[test]
    fn add_overflow() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let md = MonthDuration::months(i32::MAX);
        let rs = d0.saturating_calendar_add(md);
        assert_eq!(rs, Date::MAX);
    }

    #[test]
    fn sub_overflow() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let md = MonthDuration::months(i32::MIN);
        let rs = d0.saturating_calendar_sub(md);
        assert_eq!(rs, Date::MAX);
    }
}
