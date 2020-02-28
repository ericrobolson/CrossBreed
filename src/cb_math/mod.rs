/// Returns the number raised to the power
pub fn pow(num: usize, pow: usize) -> usize {
    let mut value = 1;

    for _ in 0..pow {
        value *= num;
    }

    return value;
}

pub struct Noise {
    values: Vec<Vec<usize>>,
    min_value: u32,
    max_value: u32,
}

impl Noise {
    /// Create a new noise object. Uses u32's so as to be deterministic across machines
    pub fn new(seed: usize, min_value: u32, max_value: u32) -> Self {
        if min_value >= max_value {
            panic!("min should not equal max");
        }

        let diff = max_value - min_value;
        let diff = diff as usize;

        let avg_value = diff / 2;

        let mut values = Vec::with_capacity(diff);

        // populate matrix
        {
            for i in 0..diff {
                let mut v = Vec::with_capacity(diff);
                for j in 0..diff {
                    v.push(avg_value);
                }

                values.push(v);
            }
        }

        // Populate matrix with randomly assigned max / min values
        // step through, interpolating the layers the each time until there is no more interpolation left to do

        let mut i = 0;
        let density = 97;

        let mut flipped = false;

        for x in 0..diff {
            for y in 0..diff {
                //TODO: implement noise
                if i < density {
                    i += 1;
                } else {
                    i = 0;

                    if flipped {
                        values[x][y] = max_value as usize;
                    } else {
                        values[x][y] = min_value as usize;
                    }

                    flipped = !flipped;
                }

                //values[x][y] = 3;
            }
        }

        return Self {
            values: values,
            min_value: min_value,
            max_value: max_value,
        };
    }

    pub fn at(&self, x: usize, y: usize) -> usize {
        return self.values[x][y];
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
