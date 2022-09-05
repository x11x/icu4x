// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use alloc::sync::Arc;
    use core::convert::TryInto;
    use core::fmt::Write;

    use diplomat_runtime::DiplomatResult;
    use icu_calendar::types::Time;
    use icu_calendar::AnyCalendar;
    use icu_calendar::{DateTime, Iso};

    use crate::calendar::ffi::ICU4XCalendar;
    use crate::date::ffi::{ICU4XDate, ICU4XIsoDate, ICU4XIsoWeekday};
    use crate::errors::ffi::ICU4XError;
    use crate::time::ffi::ICU4XTime;
    use crate::week::ffi::{ICU4XWeekCalculator, ICU4XWeekOf};

    #[diplomat::opaque]
    /// An ICU4X DateTime object capable of containing a ISO-8601 date and time.
    #[diplomat::rust_link(icu::calendar::DateTime, Struct)]
    pub struct ICU4XIsoDateTime(pub DateTime<Iso>);

    impl ICU4XIsoDateTime {
        /// Creates a new [`ICU4XIsoDateTime`] from the specified date and time.
        #[diplomat::rust_link(icu::calendar::DateTime::new_iso_datetime, FnInStruct)]
        pub fn try_new(
            year: i32,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            nanosecond: u32,
        ) -> DiplomatResult<Box<ICU4XIsoDateTime>, ICU4XError> {
            let nanosecond = try_icu4x!(nanosecond.try_into());
            DateTime::new_iso_datetime(year, month, day, hour, minute, second)
                .map(|mut dt| {
                    dt.time.nanosecond = nanosecond;
                    Box::new(ICU4XIsoDateTime(dt))
                })
                .map_err(Into::into)
                .into()
        }

        /// Creates a new [`ICU4XIsoDateTime`] from an [`ICU4XIsoDate`] and [`ICU4XTime`] object
        #[diplomat::rust_link(icu::calendar::DateTime::new, FnInStruct)]
        pub fn new_from_date_and_time(
            date: &ICU4XIsoDate,
            time: &ICU4XTime,
        ) -> Box<ICU4XIsoDateTime> {
            let dt = DateTime::new(date.0, time.0);
            Box::new(ICU4XIsoDateTime(dt))
        }

        /// Construct from the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)
        #[diplomat::rust_link(
            icu::calendar::DateTime::from_minutes_since_local_unix_epoch,
            FnInStruct
        )]
        pub fn from_minutes_since_local_unix_epoch(
            minutes: i32,
        ) -> DiplomatResult<Box<ICU4XIsoDateTime>, ICU4XError> {
            DateTime::from_minutes_since_local_unix_epoch(minutes)
                .map(|dt| Box::new(ICU4XIsoDateTime(dt)))
                .map_err(Into::into)
                .into()
        }

        /// Gets the date contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::date, StructField)]
        pub fn date(&self) -> Box<ICU4XIsoDate> {
            Box::new(ICU4XIsoDate(self.0.date))
        }

        /// Gets the time contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::time, StructField)]
        pub fn time(&self) -> Box<ICU4XTime> {
            Box::new(ICU4XTime(self.0.time))
        }

        /// Converts this to an [`ICU4XDateTime`] capable of being mixed with dates of
        /// other calendars
        #[diplomat::rust_link(icu::calendar::DateTime::to_any, FnInStruct)]
        #[diplomat::rust_link(icu::calendar::DateTime::new_from_iso, FnInStruct, hidden)]
        pub fn to_any(&self) -> Box<ICU4XDateTime> {
            Box::new(ICU4XDateTime(self.0.to_any().wrap_calendar_in_arc()))
        }

        /// Gets the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)
        #[diplomat::rust_link(icu::calendar::DateTime::minutes_since_local_unix_epoch, FnInStruct)]
        pub fn minutes_since_local_unix_epoch(&self) -> i32 {
            self.0.minutes_since_local_unix_epoch()
        }

        /// Convert this datetime to one in a different calendar
        #[diplomat::rust_link(icu::calendar::DateTime::to_calendar, FnInStruct)]
        pub fn to_calendar(&self, calendar: &ICU4XCalendar) -> Box<ICU4XDateTime> {
            Box::new(ICU4XDateTime(self.0.to_calendar(calendar.0.clone())))
        }

        /// Returns the hour in this time
        #[diplomat::rust_link(icu::calendar::types::Time::hour, StructField)]
        pub fn hour(&self) -> u8 {
            self.0.time.hour.into()
        }
        /// Returns the minute in this time
        #[diplomat::rust_link(icu::calendar::types::Time::minute, StructField)]
        pub fn minute(&self) -> u8 {
            self.0.time.minute.into()
        }
        /// Returns the second in this time
        #[diplomat::rust_link(icu::calendar::types::Time::second, StructField)]
        pub fn second(&self) -> u8 {
            self.0.time.second.into()
        }
        /// Returns the nanosecond in this time
        #[diplomat::rust_link(icu::calendar::types::Time::nanosecond, StructField)]
        pub fn nanosecond(&self) -> u32 {
            self.0.time.nanosecond.into()
        }

        /// Returns the 1-indexed day in the month for this date
        #[diplomat::rust_link(icu::calendar::Date::day_of_month, FnInStruct)]
        pub fn day_of_month(&self) -> u32 {
            self.0.date.day_of_month().0
        }

        /// Returns the day in the week for this day
        #[diplomat::rust_link(icu::calendar::Date::day_of_week, FnInStruct)]
        pub fn day_of_week(&self) -> ICU4XIsoWeekday {
            self.0.date.day_of_week().into()
        }

        /// Returns the week number in this month, 1-indexed, based on what
        /// is considered the first day of the week (often a locale preference).
        ///
        /// `first_weekday` can be obtained via `first_weekday()` on [`ICU4XWeekCalculator`]
        #[diplomat::rust_link(icu::calendar::Date::week_of_month, FnInStruct)]
        #[diplomat::rust_link(
            icu::calendar::week::WeekCalculator::week_of_month,
            FnInStruct,
            hidden
        )]
        pub fn week_of_month(&self, first_weekday: ICU4XIsoWeekday) -> u32 {
            self.0.date.week_of_month(first_weekday.into()).0
        }

        /// Returns the week number in this year, using week data
        #[diplomat::rust_link(icu::calendar::Date::week_of_year, FnInStruct)]
        #[diplomat::rust_link(
            icu::calendar::week::WeekCalculator::week_of_year,
            FnInStruct,
            hidden
        )]
        pub fn week_of_year(
            &self,
            calculator: &ICU4XWeekCalculator,
        ) -> DiplomatResult<ICU4XWeekOf, ICU4XError> {
            self.0
                .date
                .week_of_year(&calculator.0)
                .map(Into::into)
                .map_err(Into::into)
                .into()
        }

        /// Returns 1-indexed number of the month of this date in its year
        #[diplomat::rust_link(icu::calendar::Date::month, FnInStruct)]
        pub fn month(&self) -> u32 {
            self.0.date.month().ordinal
        }

        /// Returns the year number for this date
        #[diplomat::rust_link(icu::calendar::Date::year, FnInStruct)]
        pub fn year(&self) -> i32 {
            self.0.date.year().number
        }

        /// Returns the number of months in the year represented by this date
        #[diplomat::rust_link(icu::calendar::Date::months_in_year, FnInStruct)]
        pub fn months_in_year(&self) -> u8 {
            self.0.date.months_in_year()
        }

        /// Returns the number of days in the month represented by this date
        #[diplomat::rust_link(icu::calendar::Date::days_in_month, FnInStruct)]
        pub fn days_in_month(&self) -> u8 {
            self.0.date.days_in_month()
        }

        /// Returns the number of days in the year represented by this date
        #[diplomat::rust_link(icu::calendar::Date::days_in_year, FnInStruct)]
        pub fn days_in_year(&self) -> u32 {
            self.0.date.days_in_year()
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateTime object capable of containing a date and time for any calendar.
    #[diplomat::rust_link(icu::calendar::DateTime, Struct)]
    pub struct ICU4XDateTime(pub DateTime<Arc<AnyCalendar>>);

    impl ICU4XDateTime {
        /// Creates a new [`ICU4XDateTime`] representing the ISO date and time
        /// given but in a given calendar
        #[diplomat::rust_link(icu::DateTime::new_from_iso, FnInStruct)]
        #[allow(clippy::too_many_arguments)]
        pub fn try_new_from_iso_in_calendar(
            year: i32,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            nanosecond: u32,
            calendar: &ICU4XCalendar,
        ) -> DiplomatResult<Box<ICU4XDateTime>, ICU4XError> {
            let cal = calendar.0.clone();
            let nanosecond = try_icu4x!(nanosecond.try_into());
            DateTime::new_iso_datetime(year, month, day, hour, minute, second)
                .map(|mut dt| {
                    dt.time.nanosecond = nanosecond;
                    Box::new(ICU4XDateTime(dt.to_calendar(cal)))
                })
                .map_err(Into::into)
                .into()
        }
        /// Creates a new [`ICU4XDateTime`] from the given codes, which are interpreted in the given calendar system
        #[diplomat::rust_link(icu::calendar::DateTime::new_from_codes, FnInStruct)]
        #[allow(clippy::too_many_arguments)]
        pub fn try_new_from_codes_in_calendar(
            era_code: &str,
            year: i32,
            month_code: &str,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            nanosecond: u32,
            calendar: &ICU4XCalendar,
        ) -> DiplomatResult<Box<ICU4XDateTime>, ICU4XError> {
            let era = try_icu4x!(era_code.parse());
            let month = try_icu4x!(month_code.parse());
            let cal = calendar.0.clone();
            let hour = try_icu4x!(hour.try_into());
            let minute = try_icu4x!(minute.try_into());
            let second = try_icu4x!(second.try_into());
            let nanosecond = try_icu4x!(nanosecond.try_into());
            let time = Time {
                hour,
                minute,
                second,
                nanosecond,
            };
            DateTime::new_from_codes(era, year, month, day, time, cal)
                .map(|dt| Box::new(ICU4XDateTime(dt)))
                .map_err(Into::into)
                .into()
        }
        /// Creates a new [`ICU4XDateTime`] from an [`ICU4XDate`] and [`ICU4XTime`] object
        #[diplomat::rust_link(icu::calendar::DateTime::new, FnInStruct)]
        pub fn new_from_date_and_time(date: &ICU4XDate, time: &ICU4XTime) -> Box<ICU4XDateTime> {
            let dt = DateTime::new(date.0.clone(), time.0);
            Box::new(ICU4XDateTime(dt))
        }

        /// Gets a copy of the date contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::date, StructField)]
        pub fn date(&self) -> Box<ICU4XDate> {
            Box::new(ICU4XDate(self.0.date.clone()))
        }

        /// Gets the time contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::time, StructField)]
        pub fn time(&self) -> Box<ICU4XTime> {
            Box::new(ICU4XTime(self.0.time))
        }

        /// Converts this date to ISO
        #[diplomat::rust_link(icu::calendar::DateTime::to_iso, FnInStruct)]
        pub fn to_iso(&self) -> Box<ICU4XIsoDateTime> {
            Box::new(ICU4XIsoDateTime(self.0.to_iso()))
        }

        /// Convert this datetime to one in a different calendar
        #[diplomat::rust_link(icu::calendar::DateTime::to_calendar, FnInStruct)]
        pub fn to_calendar(&self, calendar: &ICU4XCalendar) -> Box<ICU4XDateTime> {
            Box::new(ICU4XDateTime(self.0.to_calendar(calendar.0.clone())))
        }

        /// Returns the hour in this time
        #[diplomat::rust_link(icu::calendar::types::Time::hour, StructField)]
        pub fn hour(&self) -> u8 {
            self.0.time.hour.into()
        }
        /// Returns the minute in this time
        #[diplomat::rust_link(icu::calendar::types::Time::minute, StructField)]
        pub fn minute(&self) -> u8 {
            self.0.time.minute.into()
        }
        /// Returns the second in this time
        #[diplomat::rust_link(icu::calendar::types::Time::second, StructField)]
        pub fn second(&self) -> u8 {
            self.0.time.second.into()
        }
        /// Returns the nanosecond in this time
        #[diplomat::rust_link(icu::calendar::types::Time::nanosecond, StructField)]
        pub fn nanosecond(&self) -> u32 {
            self.0.time.nanosecond.into()
        }

        /// Returns the 1-indexed day in the month for this date
        #[diplomat::rust_link(icu::calendar::Date::day_of_month, FnInStruct)]
        pub fn day_of_month(&self) -> u32 {
            self.0.date.day_of_month().0
        }

        /// Returns the day in the week for this day
        #[diplomat::rust_link(icu::calendar::Date::day_of_week, FnInStruct)]
        pub fn day_of_week(&self) -> ICU4XIsoWeekday {
            self.0.date.day_of_week().into()
        }

        /// Returns the week number in this month, 1-indexed, based on what
        /// is considered the first day of the week (often a locale preference).
        ///
        /// `first_weekday` can be obtained via `first_weekday()` on [`ICU4XWeekCalculator`]
        #[diplomat::rust_link(icu::calendar::Date::week_of_month, FnInStruct)]
        #[diplomat::rust_link(
            icu::calendar::week::WeekCalculator::week_of_month,
            FnInStruct,
            hidden
        )]
        pub fn week_of_month(&self, first_weekday: ICU4XIsoWeekday) -> u32 {
            self.0.date.week_of_month(first_weekday.into()).0
        }

        /// Returns the week number in this year, using week data
        #[diplomat::rust_link(icu::calendar::Date::week_of_year, FnInStruct)]
        #[diplomat::rust_link(
            icu::calendar::week::WeekCalculator::week_of_year,
            FnInStruct,
            hidden
        )]
        pub fn week_of_year(
            &self,
            calculator: &ICU4XWeekCalculator,
        ) -> DiplomatResult<ICU4XWeekOf, ICU4XError> {
            self.0
                .date
                .week_of_year(&calculator.0)
                .map(Into::into)
                .map_err(Into::into)
                .into()
        }

        /// Returns 1-indexed number of the month of this date in its year
        ///
        /// Note that for lunar calendars this may not lead to the same month
        /// having the same ordinal month across years; use month_code if you care
        /// about month identity.
        #[diplomat::rust_link(icu::calendar::Date::month, FnInStruct)]
        pub fn ordinal_month(&self) -> u32 {
            self.0.date.month().ordinal
        }

        /// Returns the month code for this date. Typically something
        /// like "M01", "M02", but can be more complicated for lunar calendars.
        #[diplomat::rust_link(icu::calendar::Date::month, FnInStruct)]
        pub fn month_code(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let code = self.0.date.month().code;
            let result = write.write_str(&code.0).map_err(Into::into).into();
            write.flush();
            result
        }

        /// Returns the year number in the current era for this date
        #[diplomat::rust_link(icu::calendar::Date::year, FnInStruct)]
        pub fn year_in_era(&self) -> i32 {
            self.0.date.year().number
        }

        /// Returns the era for this date,
        #[diplomat::rust_link(icu::calendar::Date::year, FnInStruct)]
        pub fn era(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let era = self.0.date.year().era;
            let result = write.write_str(&era.0).map_err(Into::into).into();
            write.flush();
            result
        }

        /// Returns the number of months in the year represented by this date
        #[diplomat::rust_link(icu::calendar::Date::months_in_year, FnInStruct)]
        pub fn months_in_year(&self) -> u8 {
            self.0.date.months_in_year()
        }

        /// Returns the number of days in the month represented by this date
        #[diplomat::rust_link(icu::calendar::Date::days_in_month, FnInStruct)]
        pub fn days_in_month(&self) -> u8 {
            self.0.date.days_in_month()
        }

        /// Returns the number of days in the year represented by this date
        #[diplomat::rust_link(icu::calendar::Date::days_in_year, FnInStruct)]
        pub fn days_in_year(&self) -> u32 {
            self.0.date.days_in_year()
        }

        /// Returns the [`ICU4XCalendar`] object backing this date
        #[diplomat::rust_link(icu::calendar::Date::calendar, FnInStruct)]
        #[diplomat::rust_link(icu::calendar::Date::calendar_wrapper, FnInStruct, hidden)]
        pub fn calendar(&self) -> Box<ICU4XCalendar> {
            Box::new(ICU4XCalendar(self.0.date.calendar_wrapper().clone()))
        }
    }
}