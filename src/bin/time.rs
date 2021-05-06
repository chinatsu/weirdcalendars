use std::{thread, time};
use weirdcalendars::{DecimalTime, SwatchInternetTime, NewEarthTime};
use chrono::prelude::*;

pub fn main() {
    let duration = time::Duration::from_millis(1);
    loop {
        let utcnow = Utc::now();
        let localnow = Local::now();

        let net: NewEarthTime = utcnow.into();
        let beats: SwatchInternetTime = utcnow.into();
        let vive_la_revolution: DecimalTime = localnow.time().into();

        print!("{}\t", net);
        print!("{}\t", beats);
        print!("{}\r", vive_la_revolution);
        thread::sleep(duration);
    }
}