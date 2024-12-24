use crate::galios::constants::*;
use non_std::Vec;
use crate::alloc::vec;

mod constants;

pub struct GeneratorPolynomial {
    coefficients: Vec<i32>
}

impl GeneratorPolynomial {
    pub fn from<V>(coefficients: Vec<V>) -> Self 
        where V: Into<i32>
    {
        let mut vec: Vec<i32> = Vec::new();
        for elem in coefficients {
            vec.push(elem.into());
        }
        Self {
            coefficients: vec
        }
    }

    pub fn get(&self) -> &Vec<i32> {
        &self.coefficients
    }

    pub fn multiply_galios_256(&self, other: &GeneratorPolynomial) -> Self {
        // Initialize a vector of 0s for the multiplication
        let mut output: Vec<i32> = vec![0; self.coefficients.len() + other.coefficients.len() - 1];

        for (i, coefficient) in self.coefficients.iter().enumerate() {
            for (j, other_coeff) in other.coefficients.iter().enumerate() {
                output[i + j] ^= i32::abs(*coefficient * *other_coeff);
            }
        }

        Self {
            coefficients: output,
        }
    }

    pub fn multiply_as_exponents(&self, other: &GeneratorPolynomial) -> Self {
        let mut output: Vec<i32> = vec![0; self.coefficients.len() + other.coefficients.len() - 1];

        for (i, coefficient) in self.coefficients.iter().enumerate() {
            for (j, other_coeff) in other.coefficients.iter().enumerate() {
                output[i + j] ^= LOG_TABLE[((i32::abs(*coefficient + *other_coeff) % 255) as u32) as usize] as i32;
            }
        }
        
        for value in output.iter_mut() {
            *value = ANTI_LOG_TABLE[*value as usize] as i32;
        }

        Self {
            coefficients: output
        }
    }

    pub fn multiply_by_exponent(&self, exponent: i32) -> Self {
        let mut output: Vec<i32> = Vec::new();
        
        for coefficient in self.coefficients.iter() {
            output.push((coefficient + exponent) % 255);
        }

        Self {
            coefficients: output
        }
    }

    pub fn to_integer_notation(&self) -> Self {
        let mut output: Vec<i32> = Vec::with_capacity(self.coefficients.len());

        for coefficient in self.coefficients.iter() {
            output.push(get_log(*coefficient));
        }

        Self {
            coefficients: output
        }
    }

    pub fn to_exponent_notation(&self) -> Self {
        let mut output: Vec<i32> = Vec::with_capacity(self.coefficients.len());

        for coefficient in self.coefficients.iter() {
            output.push(get_antilog(*coefficient));
        }

        Self {
            coefficients: output
        }
    }

    // Must be in integer notation
    // make sure the self is longer than other
    pub fn xor_with_other(&self, other: &GeneratorPolynomial) -> Self {
        let mut output: Vec<i32> = self.coefficients.clone();

        for (i, coefficient) in other.coefficients.iter().enumerate() {
            if i >= output.len() {
                output.push(0);
            }
            output[i] ^= coefficient;
        }

        Self {
            coefficients: output
        }
    }

    pub fn drop_leading_zeros(&self) -> Self {
        let mut output: Vec<i32> = Vec::new();

        let start = {
            let mut i = 0;
            while self.coefficients[i] == 0 { i += 1; }
            i
        };

        for index in start..self.coefficients.len() {
            output.push(self.coefficients[index]);
        }

        Self {
            coefficients: output
        }
    }
}

pub fn get_log(exponent: i32) -> i32 {
    LOG_TABLE[exponent as usize] as i32
}

pub fn get_antilog(value: i32) -> i32 {
    ANTI_LOG_TABLE[value as usize] as i32
}

// This could be useful in the future but for now the 
// log antilog tables are a much better approach
// fn galios_reduction(exponent: u32) -> u8 {
//     let mut value;
//     if exponent > 7 {
//         value = galios_reduction(exponent - 1) as u32 * 2;
//     } else {
//         value = 2_u32.pow(exponent);
//     }
// 
//     if value > 255 {
//         value ^= 285;
//     }
// 
//     value as u8
// }


#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_galios_reduction() {
    //     assert_eq!(29, galios_reduction(8));
    //     assert_eq!(58, galios_reduction(9));
    //     assert_eq!(116, galios_reduction(10));
    //     assert_eq!(232, galios_reduction(11));
    //     assert_eq!(205, galios_reduction(12));
    // }

    #[test]
    fn test_polynomial_multiply() {
        let mut poly: GeneratorPolynomial = GeneratorPolynomial::from(vec![1, -1]);
        let other_poly: GeneratorPolynomial = GeneratorPolynomial::from(vec![1, -2]);
        
        poly = poly.multiply_galios_256(&other_poly);

        assert_eq!(*poly.get(), vec![1, 3, 2]);
        
        let other_poly: GeneratorPolynomial = GeneratorPolynomial::from(vec![1, -4]);
        poly = poly.multiply_galios_256(&other_poly);

        assert_eq!(*poly.get(), vec![1, 7, 14, 8]);
    }

    #[test]
    fn test_exponent_polynomial_multiply() {
        let mut poly: GeneratorPolynomial = GeneratorPolynomial::from(vec![0, 0]);
        let other_poly: GeneratorPolynomial = GeneratorPolynomial::from(vec![0, 1]);

        poly = poly.multiply_as_exponents(&other_poly);

        assert_eq!(*poly.get(), vec![0, 25, 1]);

        let other_poly: GeneratorPolynomial = GeneratorPolynomial::from(vec![0, 2]);
        
        poly = poly.multiply_as_exponents(&other_poly);

        assert_eq!(*poly.get(), vec![0, 198, 199, 3]);
    }

    #[test]
    fn test_multiply_by_exponent() {
        let poly: GeneratorPolynomial = GeneratorPolynomial::from(vec![
            0,
            251,
            67,
            46,
            61,
            118,
            70,
            64,
            94,
            32,
            45,
        ]);

        assert_eq!(vec![
                5,
                1,
                72,
                51,
                66,
                123,
                75,
                69,
                99,
                37,
                50
            ],
            *poly.multiply_by_exponent(5).get()
        );
    }

    #[test]
    fn test_to_integer_notation() {
        let poly: GeneratorPolynomial = GeneratorPolynomial::from(vec![
            5,
            1,
            72,
            51,
            66,
            123,
            75,
            69,
            99,
            37,
            50
        ]);
        
        assert_eq!(vec![
            32,
            2,
            101,
            10,
            97,
            197,
            15,
            47,
            134,
            74,
            5
        ],
        *poly.to_integer_notation().get())
    }

    #[test]
    fn test_xor_with_other() {
        let poly = GeneratorPolynomial::from(vec![
            32,
            2,
            101,
            10,
            97,
            197,
            15,
            47,
            134,
            74,
            5
        ]);

        let message_poly = GeneratorPolynomial::from(vec![
            32,
            91,
            11,
            120,
            209,
            114,
            220,
            77,
            67,
            64,
            236,
            17,
            236,
            17,
            236,
            17
        ]);

        assert_eq!(vec![
            0,
            89,
            110,
            114,
            176,
            183,
            211,
            98,
            197,
            10,
            233,
            17,
            236,
            17,
            236,
            17
        ],
        *message_poly.xor_with_other(&poly).get()
        );
    }

    #[test]
    fn test_drop_leading_zeros() {
        let poly = GeneratorPolynomial::from(vec![
            0,
            89,
            110,
            114,
            176,
            183,
            211,
            98,
            197,
            10,
            233,
            17,
            236,
            17,
            236,
            17
        ]);

        assert_eq!(vec![
            89,
            110,
            114,
            176,
            183,
            211,
            98,
            197,
            10,
            233,
            17,
            236,
            17,
            236,
            17
        ],
        *poly.drop_leading_zeros().get()
        );
    }
}
