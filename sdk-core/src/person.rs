use email_address::*;

#[derive(Clone, Default)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl Person {
    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn validate_email(&self) -> bool {
        EmailAddress::is_valid(self.get_email())
    }
}

#[cfg(test)]
mod tests {
    use crate::person::Person;

    #[test]
    fn getters() {
        let person = Person {
            id: 5,
            name: "Janet".to_string(),
            email: "test@example.com".to_string(),
        };

        assert_eq!(*person.get_id(), 5);
        assert_eq!(*person.get_name(), "Janet");
        assert_eq!(*person.get_email(), "test@example.com");
    }

    #[test]
    fn valid_email() {
        let person = Person {
            id: 5,
            name: "Janet".to_string(),
            email: "test@example.com".to_string(),
        };

        assert_eq!(person.validate_email(), true)
    }

    #[test]
    fn invalid_email() {
        let person = Person {
            id: 5,
            name: "Janet".to_string(),
            email: "testexample.com".to_string(),
        };

        assert_eq!(person.validate_email(), false)
    }
}