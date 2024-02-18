pub fn narcissistic(num: u64) -> bool {
    num.to_string()
        .chars()
        .map(|digit| u64::from(digit.to_digit(10).unwrap()).pow(num.to_string().len() as u32))
        .sum::<u64>()
        == num
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(input: u64, expected: bool) {
        let actual = narcissistic(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn basic_tests() {
        dotest(7, true);
        dotest(371, true);
        dotest(122, false);
        dotest(4887, false);
    }
}
