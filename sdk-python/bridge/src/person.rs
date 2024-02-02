use sdk_core::person::Person as BasePerson;
use pyo3::prelude::*;

#[derive(Default)]
#[pyclass]
pub struct Person {
    base_person: BasePerson
}

#[pymethods]
impl Person {
    #[new]
    pub fn new(id: u32, name: String, email: String) -> Self {
        Self {
            base_person: BasePerson {
                id, name, email
            }
        }
    }

    #[getter]
    pub fn get_id(&self) -> u32 {
        *self.base_person.get_id()
    }

    #[getter]
    pub fn get_name(&self) -> String {
        self.base_person.get_name().to_string()
    }

    #[getter]
    pub fn get_email(&self) -> String {
        self.base_person.get_email().to_string()
    }

    pub fn validate_email(&self) -> PyResult<bool> {
        Ok(self.base_person.validate_email())
    }
}