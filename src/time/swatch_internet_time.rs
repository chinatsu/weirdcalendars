use chrono::prelude::*;
use num_integer::Integer;
use std::fmt;
use super::Time;

#[derive(Debug)]
pub struct SwatchInternetTime {
    beat: u32
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
        let beat = (((time.minute() * 60) as f64 + (time.hour() * 3600) as f64 + time.second() as f64) / 86.4).floor() as u32;
        SwatchInternetTime {
            beat
        }
    }
}

impl From<SwatchInternetTime> for NaiveTime {
    fn from(beat: SwatchInternetTime) -> Self {
        let total_seconds = (beat.beat as f64 * 86.4).ceil() as u32;
        println!("{}", total_seconds);
        let (prelim_minutes, second) = total_seconds.div_rem(&60);
        let (hour, minute) = prelim_minutes.div_rem(&60);
        NaiveTime::from_hms(hour as u32, minute as u32, second as u32)
    }
}

impl fmt::Display for SwatchInternetTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{:03}", self.beat)
    }
}


#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::SwatchInternetTime;

    #[test]
    fn twenty_UTC_is_875() {
        let twenty: SwatchInternetTime = Utc.ymd(1970, 1, 1).and_hms(20, 0, 0).into();
        assert_eq!(twenty.beat, 875);
    }

    #[test]
    fn beat875_is_21BMT() {
        let twenty: SwatchInternetTime = Utc.ymd(1970, 1, 1).and_hms(20, 0, 0).into();
        let utc: NaiveTime = twenty.into();
        assert_eq!(utc.hour(), 21);
        assert_eq!(utc.minute(), 0);
        assert_eq!(utc.second(), 0);
    }

    #[test]
    fn midnight_BMT_is_0() {
        let midnight: SwatchInternetTime = Utc.ymd(1970, 1, 1).and_hms(23, 0, 0).into();
        assert_eq!(midnight.beat, 0);
    }
}
