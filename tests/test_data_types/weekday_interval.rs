use chrono::offset::Local;
use chrono::{Duration, Weekday};

use conch::IterRangeByDuration;

use bobinator::*;

mod test_friday_off {
    use chrono::Datelike;

    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $meth:ident(
                $($args:expr),*
            ),
            $expected:expr
            $(,)?
        ) => {
            #[test]
            fn $name() {
                assert_eq!(
                    FridayOff::$meth($($args,)*),
                    $expected
                );
            }
        };
    }

    // TODO Re write these tests to make it usable.

    test_factory!(this_week, this_week(), {
        let today = Local::now().date_naive();

        today
            .week(Weekday::Mon)
            .days()
            .into_iter_by_duration(Duration::days(1))
            .find(|date| date.weekday() == Weekday::Fri)
            .unwrap()
    },);

    test_factory!(next_week, next_week(), {
        let today = Local::now().date_naive();

        (today + Duration::days(7))
            .week(Weekday::Mon)
            .days()
            .into_iter_by_duration(Duration::days(1))
            .find(|date| date.weekday() == Weekday::Fri)
            .unwrap()
    },);

    // test_factory!(
    //     next_in_group_from_next_week,
    //     next_in_group(0, Some(Local::now().date_naive() + Duration::days(7))),
    //     {
    //         let today = Local::now().date_naive();

    //         today
    //             .week(Weekday::Mon)
    //             .days()
    //             .into_iter_by_duration(Duration::days(1))
    //             .find(|date| date.weekday() == Weekday::Fri)
    //             .unwrap()
    //             + Duration::days(14)
    //     },
    // );

    // test_factory!(
    //     next_in_group_from_two_weeks_later,
    //     next_in_group(0, Some(Local::now().date_naive() + Duration::days(14))),
    //     {
    //         let today = Local::now().date_naive();

    //         today
    //             .week(Weekday::Mon)
    //             .days()
    //             .into_iter_by_duration(Duration::days(1))
    //             .find(|date| date.weekday() == Weekday::Fri)
    //             .unwrap()
    //             + Duration::days(14 * {
    //                 if Local::now().date_naive().weekday().num_days_from_monday() <= Weekday::Fri.num_days_from_monday() {
    //                     1
    //                 } else {
    //                     2
    //                 }
    //             })
    //     },
    // );
}
