/*!
The Rialight temporal API for working with date/time.

This API follows a convention of using types whose names start
with "Plain" (like `PlainDate`, `PlainTime` and `PlainDateTime`)
for objects which do not have an associated time zone. Converting between
such types and exact time types (like `Instant` and `ZonedDateTime`)
can be ambiguous because of time zones and daylight saving time,
and the temporal API lets developers configure how this ambiguity
is resolved.

Several important concepts are explained elsewhere:
[exact time, wall-clock time, time zones, DST, handling ambiguity, and more][docs::ambiguity].
*/

mod platform;

pub mod docs;

mod instant;
pub use instant::Instant;

mod duration;
pub use duration::Duration;

mod error;
pub use error::RangeError;

mod zoned_date_time;
pub use zoned_date_time::{ZonedDateTime, ZonedDateTimeOptions};

mod timezone;
pub use timezone::TimeZone;

mod calendar;
pub use calendar::Calendar;