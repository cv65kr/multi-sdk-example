use sdk_core::person::Person as BasePerson;

#[derive(Default)]
pub struct Person {
    base_person: BasePerson
}

impl Person {
    pub fn __construct(id: u32, name: String, email: String) -> Self {
        Person {
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

    pub fn validate_email(&self) -> bool {
        self.base_person.validate_email()
    }
}