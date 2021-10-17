// quiz3.rs
// This is a quiz for the following sections:
// - Tests

// This quiz isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests that we get the result
// we expect to get when we call `times_two` with a negative number.
// No hints, you can do this :)

pub fn times_two(num: i32) -> i64 {
    num as i64 * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_twice_of_positive_numbers() {
        assert_eq!(times_two(4), 8);
        assert_eq!(times_two(1), 2);
        assert_eq!(times_two(0), 0);
    }

    #[test]
    fn returns_twice_of_negative_numbers() {
        assert_eq!(times_two(-4), -8);
        for i in [i32::MIN, -1, 0, 1, i32::MAX] {
            assert_eq!(times_two(i), 2 * i as i64);
        }
    }
}
