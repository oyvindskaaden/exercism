// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (speed as f64) * 221_f64 * match speed {
        0 ..= 4         => 1.0,
        5 ..= 8         => 0.9,
        9 ..= u8::MAX   => 0.77
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60_f64) as u32
}
