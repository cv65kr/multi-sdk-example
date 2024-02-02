use ext_php_rs::prelude::*;
use sdk_core::calculator::sum_numbers as base_sum_numbers;

#[php_function]
pub fn sum_numbers(a: i8, b: i8) -> i8 {
    base_sum_numbers(a, b)
}