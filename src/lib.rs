mod time;
pub use time::{decimal_time::DecimalTime, new_earth_time::NewEarthTime, swatch_internet_time::SwatchInternetTime};

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use crate::DecimalTime;
    use crate::NewEarthTime;
    #[test]
    fn decimal_time_to_new_earth_time() {
        let utcnow = Utc::now();
        let netnow: NewEarthTime = utcnow.into();
        let convertednow: NaiveTime = netnow.into();
        let decimalnow: DecimalTime = convertednow.into();
        let finalconvertednow: NaiveTime = decimalnow.into();
        assert_eq!(finalconvertednow, utcnow.time().with_nanosecond(0).unwrap());
    }
}
