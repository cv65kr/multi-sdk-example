use sdk_core::calculator::sum_numbers as base_sum_numbers;
use pyo3::prelude::*;

#[pyfunction]
pub fn sum_numbers(a: i8, b: i8) -> PyResult<i8> {
    Ok(base_sum_numbers(a, b))
}