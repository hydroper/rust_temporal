use super::RangeError;

static DAY_SECONDS: i128 = 86_400;
static DAY_NANOS: i128 = DAY_SECONDS * 1_000_000_000;
static NS_MIN: i128 = -DAY_SECONDS * 100_000_000_000_000_000;
static NS_MAX: i128 = DAY_SECONDS * 100_000_000_000_000_000;
static YEAR_MIN: i128 = -271_821;
static YEAR_MAX: i128 = 275_760;
static BEFORE_FIRST_DST: i128 = -388_152 * 10_000_000_000_000;

pub fn validate_epoch_nanoseconds(epoch_nanoseconds: i128) -> Result<(), RangeError> {
    if epoch_nanoseconds < NS_MIN || epoch_nanoseconds > NS_MAX { Err(RangeError) } else { Ok(()) }
}