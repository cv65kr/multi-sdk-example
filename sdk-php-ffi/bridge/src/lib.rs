use sdk_core::calculator::sum_numbers as base_sum_numbers;
use sdk_core::person::Person as BasePerson;

// Sum numbers
#[no_mangle]
pub unsafe extern "C" fn sum_numbers(a: i8, b: i8) -> i8 {
    base_sum_numbers(a, b)
}

// Person
#[repr(C)]
pub struct Person {
    pub base_person: BasePerson
}