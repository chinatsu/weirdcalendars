mod decimal_time;
mod new_earth_time;

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use crate::decimal_time::DecimalTime;
    use crate::new_earth_time::NewEarthTime;
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
