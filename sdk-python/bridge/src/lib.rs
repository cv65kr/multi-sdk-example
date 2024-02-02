use pyo3::prelude::*;

mod calculator;
mod person;

#[pymodule]
fn sdk_python(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculator::sum_numbers, m)?)?;
    m.add_class::<person::Person>()?;
    Ok(())
}