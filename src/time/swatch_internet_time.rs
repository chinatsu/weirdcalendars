use chrono::prelude::*;
use num_integer::Integer;
use std::fmt;
use super::Time;

#[derive(Debug)]
pub struct SwatchInternetTime {
    beat: u32,
    millibeat: u32
}

impl Time<SwatchInternetTime> for SwatchInternetTime {
    fn now() -> Self {
        let naive_now = Utc::now();
        naive_now.into()
    }
}

impl From<DateTime<Utc>> for SwatchInternetTime {
    fn from(dt: DateTime<Utc>) -> Self {
        let time = dt.with_timezone(&FixedOffset::east(3600)).time();
        let total_subbeats = (
            time.hour() * 36000000 + 
            time.minute() * 600000 + 
            time.second() * 10000 +
            time.nanosecond() / 100000  
        ) / 864;
        let (beat, millibeat) = total_subbeats.div_rem(&1000);
        SwatchInternetTime {
            beat,
            millibeat
        }
    }
}

impl From<SwatchInternetTime> for NaiveTime {
    fn from(beat: SwatchInternetTime) -> Self {
        let total_subseconds = beat.beat * 864000 + beat.millibeat * 864;
        let (second, millisecond) = total_subseconds.div_rem(&10000); 
        NaiveTime::from_num_seconds_from_midnight(second, millisecond*100000)
    }
}

impl fmt::Display for SwatchInternetTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{:03}.{:03}", self.beat, self.millibeat)
    }
}


#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::SwatchInternetTime;

    #[test]
    fn twenty_utc_is_beat_875() {
        let twenty: SwatchInternetTime = Utc.ymd(1970, 1, 1).and_hms(20, 0, 0).into();
        assert_eq!(twenty.beat, 875);
    }

    #[test]
    fn beat_875_is_21_bmt() {
        let twenty: SwatchInternetTime = Utc.ymd(1970, 1, 1).and_hms(20, 0, 0).into();
        let utc: NaiveTime = twenty.into();
        assert_eq!(utc.hour(), 21);
        assert_eq!(utc.minute(), 0);
        assert_eq!(utc.second(), 0);
    }

    #[test]
    fn midnight_bmt_is_0() {
        let midnight: SwatchInternetTime = Utc.ymd(1970, 1, 1).and_hms(23, 0, 0).into();
        assert_eq!(midnight.beat, 0);
    }
}
