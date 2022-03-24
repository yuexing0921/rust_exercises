// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]


const PRODUCTION_SPEED: f64 = 221.0;


pub fn production_rate_per_hour(speed: u8) -> f64 {
    // unimplemented!("calculate hourly production rate at speed: {}", speed)
//    if speed == 0 {
//        return 0.0
//    }
//    if speed > 0 && speed <= 4 {
//         return speed as f64 * PRODUCTION_SPEED as f64
//    }
//    if speed > 4 && speed <= 8 {
//         return speed as f64 * 0.90 * PRODUCTION_SPEED as f64
//    }
//    return speed as f64 * 0.77 * PRODUCTION_SPEED as f64
    PRODUCTION_SPEED * (speed as f64) * match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        // 9..=u8::MAX => 0.77,
        _ => 0.77,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
    // let total = production_rate_per_hour(speed);
    // (total / 60 as f64).floor() as u32
    let cars = production_rate_per_hour(speed);
    (cars/60.0) as u32
}
