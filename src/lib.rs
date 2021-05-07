mod time;
pub use time::{decimal_time::DecimalTime, new_earth_time::NewEarthTime, swatch_internet_time::SwatchInternetTime, Time};

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use crate::DecimalTime;
    use crate::NewEarthTime;
    use crate::SwatchInternetTime;
    #[test]
    fn decimal_time_to_new_earth_time() {
        let utcnow = Utc::now();
        let netnow: NewEarthTime = utcnow.into();
        let convertednow: NaiveTime = netnow.into();
        let decimalnow: DecimalTime = convertednow.into();
        let secondconverted: NaiveTime = decimalnow.into();
        let utcconverted: DateTime<Utc> = Utc.ymd(1970, 1, 1).and_hms(
            secondconverted.hour()-1, // to account for BMT lol
            secondconverted.minute(), 
            secondconverted.second());
        let swatchnow: SwatchInternetTime = utcconverted.into();
        let finalconvertednow: NaiveTime = swatchnow.into();
        
        // fails a lot of the time :(
        assert_eq!(finalconvertednow.round_subsecs(0), utcnow.time().round_subsecs(0)); 
    }
}
