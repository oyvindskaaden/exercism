// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let production: f64 = (speed as f64) * 221_f64;

    match speed {
        1 ..= 4     => production,
        5 ..= 8     => production * 0.9,
        9 ..= 10    => production * 0.77,
        _           => 0_f64
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60_f64) as u32
}
