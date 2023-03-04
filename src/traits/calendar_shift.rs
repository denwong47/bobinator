use conch::{CalendarMonth, Modifier, RegionMarker};

use crate::{HasDate, UserInput};

pub const CALENDAR_WIDTH: i32 = 22;

/// Trait for shifting the cursor to the line of the relevant date.
pub trait CalendarMonthShiftModifier<T>
where
    T: HasDate,
{
    fn shift_modifier_for(&self, obj: &T) -> Option<Modifier>;

    fn shifted_input_for<F>(&self, obj: &T, f: F) -> Option<UserInput>
    where
        F: FnOnce() -> UserInput;
}

impl<T, Region> CalendarMonthShiftModifier<T> for CalendarMonth<Region>
where
    T: HasDate,
    Region: RegionMarker,
{
    fn shift_modifier_for(&self, obj: &T) -> Option<Modifier> {
        let date = obj.date();

        self.week_number_of(date).map(|week_num| {
            Modifier::up((self.weeks_count() - week_num) as i32)
                + Modifier::right(CALENDAR_WIDTH + 3)
        })
    }

    /// Trigger a [`UserInput`] prompt at the shifted location for this date.
    ///
    /// Be careful not to include any `\n` in your prompt, as that resets
    /// the horizontal positioning of the cursor.
    fn shifted_input_for<F>(&self, obj: &T, f: F) -> Option<UserInput>
    where
        F: FnOnce() -> UserInput,
    {
        let date = obj.date();

        self.week_number_of(date).map(|week_num| {
            print!(
                "{}",
                Modifier::up((self.weeks_count() - week_num) as i32)
                    + Modifier::right(CALENDAR_WIDTH + 3)
            );
            // Assuming that `f()` will enter a new line using `println!()`, ...
            let result = f();

            // ...we will have
            // - one less line to move downwards, and
            // - no need to carriage return.
            print!(
                "{}",
                Modifier::down((self.weeks_count() - week_num - 1) as i32)
            );

            result
        })
    }
}
