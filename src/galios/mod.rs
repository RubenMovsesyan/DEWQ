use crate::galios::constants::*;
use non_std::Vec;
use crate::alloc::vec;
use crate::test_utils::{test_println, test_print};

mod constants;

#[cfg(any(
        test,
        feature = "test_feature"
))]
extern crate std;

#[cfg(any(
        test,
        feature = "test_feature"
))]
#[macro_use]
use std::println;


#[cfg(any(
        test,
        feature = "test_feature"
))]
#[macro_use]
use std::print;

#[cfg(any(
        test,
        feature = "test_feature"
))]
use std::fmt::Display;

#[derive(Clone)]
struct PolynomialData(Vec<i32>);

impl PolynomialData {
    pub fn new<V>(data: Vec<V>) -> Self
        where V: Into<i32>
    {
        let vec: Vec<i32> = data.into_iter().map(Into::into).collect();
        Self(vec)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self) -> &Vec<i32> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Vec<i32> {
        &mut self.0
    }
}


fn get_log(exponent: i32) -> i32 {
    LOG_TABLE[exponent as usize] as i32
}

fn get_antilog(value: i32) -> i32 {
    ANTI_LOG_TABLE[value as usize] as i32
}


#[derive(Clone)]
enum Notation {
    Integer,
    Exponent,
}

#[derive(Clone)]
pub struct Polynomial {
    data: PolynomialData,
    notation: Notation, 
}



impl Polynomial {
    pub fn from_integer_notation<V>(data: Vec<V>) -> Self
        where V: Into<i32>
    {
        Self {
            data: PolynomialData::new(data),
            notation: Notation::Integer,
        }
    }

    pub fn from_exponent_notation<V>(data: Vec<V>) -> Self
        where V: Into<i32>
    {
        Self {
            data: PolynomialData::new(data),
            notation: Notation::Exponent,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn convert_to_exponent_notation(&mut self) {
        match self.notation {
            Notation::Integer => {
                for elem in self.data.get_mut() {
                    *elem = get_antilog(*elem);
                }

                self.notation = Notation::Exponent;
            },
            _ => {},
        }
    }

    pub fn convert_to_integer_notation(&mut self) {
        match self.notation {
            Notation::Exponent => {
                for elem in self.data.get_mut() {
                    *elem = get_log(*elem);
                }

                self.notation = Notation::Integer;
            },
            _ => {},
        }
    }

    pub fn as_exponent_notation(&self) -> Self {
        match self.notation {
            Notation::Integer => {
                let mut output = PolynomialData::new(Vec::<i32>::with_capacity(self.len()));
                
                for elem in self.data.get() {
                    output.get_mut().push(get_antilog(*elem));
                }

                Self {
                    data: output,
                    notation: Notation::Exponent,
                }
            },
            _ => { self.clone() }
        }
    }

    pub fn as_integer_notation(&self) -> Self {
        match self.notation {
            Notation::Exponent => {
                let mut output = PolynomialData::new(Vec::<i32>::with_capacity(self.len()));
                
                for elem in self.data.get() {
                    output.get_mut().push(get_log(*elem));
                }

                Self {
                    data: output,
                    notation: Notation::Integer,
                }
            },
            _ => { self.clone() }
        }
    }

    pub fn multiply(&mut self, other: &mut Polynomial) -> Self {
        self.convert_to_exponent_notation();
        other.convert_to_exponent_notation();


        let mut output = PolynomialData::new(vec![0; self.len() + other.len() - 1]);

        for (i, self_coeff) in self.data.get().iter().enumerate() {
            for (j, other_coeff) in other.data.get().iter().enumerate() {
                output.get_mut()[i + j] ^= get_log(
                        i32::abs(*self_coeff + *other_coeff) % 255
                );
            }
        }

        // HACK: Fix this later
        let mut ret = Self {
            data: output,
            notation: Notation::Integer,
        };

        ret.convert_to_exponent_notation();
        ret
    }

    pub fn xor(&mut self, other: &mut Polynomial) -> Self {
        self.convert_to_integer_notation();
        other.convert_to_integer_notation();

        let mut output = PolynomialData::new(self.get_as_integer_vec().clone());

        for (i, coeff) in other.data.get().iter().enumerate() {
            if i >= self.len() {
                output.get_mut().push(0);
            }

            output.get_mut()[i] ^= coeff;
        }

        Self {
            data: output,
            notation: Notation::Integer,
        }
    }

    pub fn multiply_by_exponent(&mut self, exponent: i32) -> Self {
        self.convert_to_exponent_notation();

        let mut output = PolynomialData::new(Vec::<i32>::with_capacity(self.len()));

        for elem in self.data.get() {
            output.get_mut().push((*elem + exponent) % 255);
        }

        Self {
            data: output,
            notation: Notation::Exponent,
        }
    }

    pub fn drop_leading_zeros(&mut self) {
        self.convert_to_integer_notation();
        let mut new = PolynomialData::new(Vec::<i32>::with_capacity(self.len()));
        
        let start = {
            let mut i = 0;
            while self.data.get()[i] == 0 { i += 1; }
            i
        };

        for index in start..self.len() {
            new.get_mut().push(self.data.get()[index]);
        }

        self.data = new;
    }

    pub fn get_as_integer_vec(&mut self) -> &Vec<i32> {
        self.convert_to_integer_notation();
        self.data.get()
    }

    pub fn get_as_exponent_vec(&mut self) -> &Vec<i32> {
        self.convert_to_exponent_notation();
        self.data.get()
    }
}


#[cfg(any(
        test,
        feature = "test_feature"
))]
impl Display for Polynomial {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{:?}", self.as_integer_notation().data.get())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_leading_zeros() {
        let mut poly = Polynomial::from_integer_notation(vec![0, 0, 34, 45]);
        
        poly.drop_leading_zeros();

        assert_eq!(poly.get_as_integer_vec(), &vec![34, 45]);
    }
}
