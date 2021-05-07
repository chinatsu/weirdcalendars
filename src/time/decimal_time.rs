use chrono::prelude::*;
use num_integer::Integer;
use std::fmt;
use super::Time;

#[derive(Debug)]
pub struct DecimalTime {
    hour: u32,
    minute: u32,
    second: u32,
    milli: u32
}

impl Time<DecimalTime> for DecimalTime {
    fn now() -> Self {
        let naive_now = Local::now().time();
        naive_now.into()
    }
}

impl From<NaiveTime> for DecimalTime {
    fn from(time: NaiveTime) -> Self {
        let total_decimal_subseconds = (
            (time.hour() as u64 * 360000000) + 
            (time.minute() as u64 * 6000000) + 
            (time.second() as u64 * 100000) + 
            (time.nanosecond() as u64 / 10000)) as u32 
            / 864 * 10;
        let (prelim_seconds, decimal_millis) = total_decimal_subseconds.div_rem(&1000);
        let (prelim_minutes, decimal_seconds) = prelim_seconds.div_rem(&100);
        let (decimal_hours, decimal_minutes) = prelim_minutes.div_rem(&100);
        DecimalTime {
            hour: decimal_hours,
            minute: decimal_minutes,
            second: decimal_seconds,
            milli: decimal_millis / 10
        }
    }
}

impl From<DecimalTime> for NaiveTime {
    fn from(dt: DecimalTime) -> Self {
        let total_dt_millis = dt.hour * 10000000 + dt.minute * 100000 + dt.second * 1000 + dt.milli;
        let total_millis = (total_dt_millis as f64 * 0.864).ceil() as u32;
        let (seconds, millis) = total_millis.div_rem(&1000);
        NaiveTime::from_num_seconds_from_midnight(seconds, millis*100000)
    }
}

impl fmt::Display for DecimalTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:01}h {:02}m {:02}.{:02}s", self.hour, self.minute, self.second, self.milli)
    }
}


#[cfg(test)]
mod tests {
    use super::DecimalTime;
    use chrono::prelude::*;
    #[test]
    fn midnight_is_zero() {
        let midnight: DecimalTime = NaiveTime::from_hms(0, 0, 0).into();
        assert_eq!(midnight.hour, 0);
        assert_eq!(midnight.minute, 0);
        assert_eq!(midnight.second, 0);
    }

    #[test]
    fn noon_is_five() {
        let noon: DecimalTime = NaiveTime::from_hms(12, 0, 0).into();
        assert_eq!(noon.hour, 5);
        assert_eq!(noon.minute, 0);
        assert_eq!(noon.second, 0);
    }

    #[test]
    fn cozette_handshake_is_4_72_91() {
        let handshake: DecimalTime = NaiveTime::from_hms(11, 21, 0).into();
        assert_eq!(handshake.hour, 4);
        assert_eq!(handshake.minute, 72);
        assert_eq!(handshake.second, 91);
    }

        #[test]
    fn now_is_now() {
        let utcnow = Utc::now().time();
        let now: DecimalTime = utcnow.into();
        let utcconverted: NaiveTime = now.into();
        assert_eq!(utcconverted, utcnow.with_nanosecond(0).unwrap());
    }
}
