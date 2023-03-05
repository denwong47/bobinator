use conch::{CalendarMonth, Modifier, RegionMarker, StringWrapper};
use termsize;

use crate::{common, flush_stdout, HasDate, UserInput};

pub const CALENDAR_WIDTH: i32 = 22;

/// Trait for shifting the cursor to the line of the relevant date.
pub trait CalendarMonthShiftModifier<T>
where
    T: HasDate,
{
    /// Return the [`Modifier`] that shifts the cursor to the end of line belonging to
    /// `obj`.
    ///
    /// This assumes that the current cursor is at the new line immediately after
    /// printing the calendar.
    ///
    /// If `obj` is not in the [`CalendarMonth`] instance, return [`None`].
    fn shift_modifier_for(&self, obj: &T) -> Option<Modifier>;

    /// Use `f` to produce a user input prompt at the end of line belonging to `obj`,
    /// then return the [`UserInput`].
    ///
    /// If `obj` is not in the [`CalendarMonth`] instance, return [`None`] without
    /// printing the prompt.
    fn shifted_input_for<F>(&self, obj: &T, f: F) -> Option<UserInput>
    where
        F: FnOnce() -> UserInput;

    /// Assuming we are at the new line immediately after printing the calendar, shift
    /// the cursor to the line belonging to `obj` and print out `text`.
    ///
    /// If `obj` is not in the [`CalendarMonth`] instance, return [`None`].
    fn shifted_print_for(&self, obj: &T, text: &str) -> Option<()> {
        self.shift_modifier_for(obj)
            .map(|modifier| print!("{}", modifier.wraps(text)))
    }

    /// Overwrite all characters on the line belonging to `obj`.
    ///
    /// This assumes that the current cursor is at the new line immediately after
    /// printing the calendar.
    fn wipe_line_for(&self, obj: &T) -> Option<()> {
        let term_w = termsize::get().map(|size| size.cols).unwrap_or(60);

        self.shifted_print_for(
            obj,
            &format!(
                "{:len$}",
                "",
                len = (term_w as i32 - CALENDAR_WIDTH - 3) as usize
            ),
        )
        .map(|_| {
            // Don't use `and` here because `and` is eagerly evaluated.
            flush_stdout()
        })
    }
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
            common::flush_stdout();

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