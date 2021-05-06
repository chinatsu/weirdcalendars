use chrono::prelude::*;
use num_integer::Integer;
use std::fmt;

#[derive(Debug)]
pub struct NewEarthTime {
    degree: u32,
    minute: u32,
    second: u32
}

impl From<DateTime<Utc>> for NewEarthTime {
    fn from(dt: DateTime<Utc>) -> Self {
        let time = dt.time();
        let total_seconds = time.hour() * 3600 + time.minute() * 60 + time.second();
        let total_net_seconds = total_seconds * 15;
        let (prelim_minute, net_second) = total_net_seconds.div_rem(&60);
        let (net_degree, net_minute) = prelim_minute.div_rem(&60);
        NewEarthTime {
            degree: net_degree,
            minute: net_minute,
            second: net_second
        }
    }
}

impl From<NewEarthTime> for NaiveTime {
    fn from(net: NewEarthTime) -> Self {
        let total_net_seconds = net.degree * 3600 + net.minute * 60 + net.second;
        let total_seconds = total_net_seconds / 15;
        let (prelim_minute, second) = total_seconds.div_rem(&60);
        let (hour, minute) = prelim_minute.div_rem(&60);
        NaiveTime::from_hms(hour, minute, second)
    }
}

impl fmt::Display for NewEarthTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:03}° {:02}′ {:02}″", self.degree, self.minute, self.second)
    }
}


#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::NewEarthTime;

    #[test]
    fn midnight_is_zero() {
        let midnight: NewEarthTime = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0).into();
        assert_eq!(midnight.degree, 0);
        assert_eq!(midnight.minute, 0);
        assert_eq!(midnight.second, 0);
    }

    #[test]
    fn noon_is_180_degrees() {
        let noon: NewEarthTime = Utc.ymd(1970, 1, 1).and_hms(12, 0, 0).into();
        assert_eq!(noon.degree, 180);
        assert_eq!(noon.minute, 0);
        assert_eq!(noon.second, 0);
    }

    #[test]
    fn cozette_handshake_is_170_15_0() {
        let handshake: NewEarthTime = Utc.ymd(1970, 1, 1).and_hms(11, 21, 0).into();
        assert_eq!(handshake.degree, 170);
        assert_eq!(handshake.minute, 15);
        assert_eq!(handshake.second, 0);
    }

    #[test]
    fn noon_is_180_degrees_and_noon() {
        let noon: NewEarthTime = Utc.ymd(1970, 1, 1).and_hms(12, 0, 0).into();
        let utctime: NaiveTime = noon.into();
        assert_eq!(utctime.hour(), 12);
        assert_eq!(utctime.minute(), 0);
        assert_eq!(utctime.second(), 0);
    }

    #[test]
    fn now_is_now() {
        let utcnow = Utc::now();
        let now: NewEarthTime = utcnow.into();
        let utcconverted: NaiveTime = now.into();
        assert_eq!(utcconverted, utcnow.time().with_nanosecond(0).unwrap());
    }
}
