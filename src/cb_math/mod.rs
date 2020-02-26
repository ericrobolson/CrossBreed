/// Returns the number raised to the power
pub fn pow(num: usize, pow: usize) -> usize {
    let mut value = 1;

    for _ in 0..pow {
        value *= num;
    }

    return value;
}

#[cfg(test)]
mod tests {
    use super::*;

    // new tests
    #[test]
    fn pow_num0_pow3_returns0() {
        assert_eq!(0, pow(0, 3));
    }

    #[test]
    fn pow_num1_pow3_returns1() {
        assert_eq!(1, pow(1, 3));
    }

    #[test]
    fn pow_num3_pow0_returns1() {
        assert_eq!(1, pow(3, 0));
    }

    #[test]
    fn pow_num2_pow2_returns4() {
        assert_eq!(4, pow(2, 2));
    }

    #[test]
    fn pow_num3_pow3_returns27() {
        assert_eq!(27, pow(3, 3));
    }
}
