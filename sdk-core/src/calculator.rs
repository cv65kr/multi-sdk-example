pub fn sum_numbers(a: i8, b: i8) -> i8 {
    a+b
}

#[cfg(test)]
mod tests {
    use super::sum_numbers;
    #[test]
    fn sum_positive_numbers() {
        assert_eq!(sum_numbers(2,2), 4);
    }

    #[test]
    fn sum_negative_numbers() {
        assert_eq!(sum_numbers(2, -2), 0);
        assert_eq!(sum_numbers(-2, -2), -4);
    }
}