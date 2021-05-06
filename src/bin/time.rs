use std::{thread, time};
use weirdcalendars::{DecimalTime, SwatchInternetTime, NewEarthTime, Time};
use chrono::prelude::*;

pub fn main() {
    let duration = time::Duration::from_millis(1);
    loop {
        let net = NewEarthTime::now();
        let beats = SwatchInternetTime::now();
        let vive_la_revolution = DecimalTime::now();

        print!("{}\t", net);
        print!("{}\t", beats);
        print!("{}\r", vive_la_revolution);
        thread::sleep(duration);
    }
}