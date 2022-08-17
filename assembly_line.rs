// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    //unimplemented!("calculate hourly production rate at speed: {}", speed)
    if(speed >= 1 && speed <= 4){
        return speed as f64 * 221.0
    }
    if(speed >= 5 && speed <= 8){
        return speed as f64 * 221.0 * 0.9
    }
    return speed as f64 * 221.0 * 0.77
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    //unimplemented!("calculate the amount of working items at speed: {}", speed)
    return production_rate_per_hour(speed) as u32 / 60 // return (production_rate_per_hour(speed) / 60.0) as u32
}

