#![allow(unused)]

const PRODUCTION_PER_SPEED: u8 = 221;
const MINUTES_PER_HOUR: u8     = 60;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed = speed.min(10);

    let success_rate = if speed > 8 {
        0.77
    } else if speed > 4 {
        0.9
    } else if speed > 0 {
        1.0
    } else {
        0.0_f64
    };

    PRODUCTION_PER_SPEED as f64 * speed as f64 * success_rate
}

// another implementation
pub fn production_rate_
per_hour_a(speed: u8) -> f64 {
    let success_rate = match speed {
        0 => 0.0_f64,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9 | 10 => 0.77,
        _ => panic!("speed value must be between 0 and 10: but got {}.", speed)
    };

    PRODUCTION_PER_SPEED as f64 * speed as f64 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let cars_per_hour = production_rate_per_hour(speed);

    (cars_per_hour / MINUTES_PER_HOUR as f64) as u32
}

fn main() {
    println!( "{}", production_rate_per_hour(6));
    println!( "{}", working_items_per_minute(6));
}
