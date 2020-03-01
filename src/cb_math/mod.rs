/// Returns the number raised to the power
pub fn pow(num: usize, pow: usize) -> usize {
    let mut value = 1;

    for _ in 0..pow {
        value *= num;
    }

    return value;
}

pub fn index_1d_to_3d(index: usize, x_max: usize, y_max: usize) -> (usize, usize, usize) {
    let mut i = index;

    let z = i / (x_max * y_max);
    i -= (z * x_max * y_max);
    let y = i / x_max;
    let x = i % x_max;

    return (x, y, z);
}

pub fn index_2d_to_1d(x: usize, y: usize, array_size: usize) -> usize {
    return x + array_size * y;
}

pub struct Noise {
    values: Vec<u32>,
    max_value: u32,
}

impl Noise {
    /// Create a new noise object. Uses u32's so as to be deterministic across machines
    pub fn new(seed: usize, max_value: u32) -> Self {
        const MIN_VALUE: u32 = 0;

        // populate matrix

        // Populate matrix with randomly assigned max / min values
        // step through, interpolating the layers the each time until there is no more interpolation left to do

        let mut i = 0;
        let density = 97;

        let mut flipped = false;

        return Self {
            values: vec![],
            max_value: max_value,
        };
    }

    pub fn at(&self, x: usize, y: usize) -> u32 {
        return self.values[index_2d_to_1d(x, y, self.values.len())];
    }
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
