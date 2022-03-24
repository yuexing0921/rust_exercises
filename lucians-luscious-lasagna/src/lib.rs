// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const EXPECTED_MINUTES: i32 = 40;
 
pub fn expected_minutes_in_oven() -> i32 {
    // unimplemented!("return expected minutes in the oven")
    EXPECTED_MINUTES
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    // unimplemented!(
    //     "calculate remaining minutes in oven given actual minutes in oven: {}",
    //     actual_minutes_in_oven
    // )
    return expected_minutes_in_oven() - actual_minutes_in_oven
}

const LAYERS_NUMBER: i32 = 2;
pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    // unimplemented!(
    //     "calculate preparation time in minutes for number of layers: {}",
    //     number_of_layers
    // )
    LAYERS_NUMBER * number_of_layers
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    // unimplemented!(
    //     "calculate elapsed time in minutes for number of layers {} and actual minutes in oven {}",
    //     number_of_layers,
    //     actual_minutes_in_oven
    // );
    return preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
