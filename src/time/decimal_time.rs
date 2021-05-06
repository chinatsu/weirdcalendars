use chrono::prelude::*;
use num_integer::Integer;
use std::fmt;

#[derive(Debug)]
pub struct DecimalTime {
    hour: u32,
    minute: u32,
    second: u32
}

impl From<NaiveTime> for DecimalTime {
    fn from(time: NaiveTime) -> Self {
        let total_seconds = time.hour() * 3600 + time.minute() * 60 + time.second();
        let total_decimal_seconds: u32 = (total_seconds as f64 / 0.864) as u32;
        let (prelim_minutes, decimal_seconds) = total_decimal_seconds.div_rem(&100);
        let (decimal_hours, decimal_minutes) = prelim_minutes.div_rem(&100);
        DecimalTime {
            hour: decimal_hours,
            minute: decimal_minutes,
            second: decimal_seconds
        }
    }
}

impl From<DecimalTime> for NaiveTime {
    fn from(dt: DecimalTime) -> Self {
        let total_dt_seconds = dt.hour * 10000 + dt.minute * 100 + dt.second;
        let total_seconds = (total_dt_seconds as f64 * 0.864).ceil() as u32;
        let (prelim_minute, second) = total_seconds.div_rem(&60);
        let (hour, minute) = prelim_minute.div_rem(&60);
        NaiveTime::from_hms(hour, minute, second)
    }
}

impl fmt::Display for DecimalTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}h {:02}m {:02}s", self.hour, self.minute, self.second)
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
