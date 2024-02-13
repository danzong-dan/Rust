use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Clone)]
struct BigInt {
    digits: Vec<u8>,
    sign: Sign,
}

impl BigInt {
    fn new() -> BigInt {
        BigInt {
            digits: Vec::new(),
            sign: Sign::Positive,
        }
    }

    fn from(s: &str) -> Result<BigInt, &'static str> {
        if s.is_empty() {
            return Err("Invalid argument");
        }

        let mut sign = Sign::Positive;
        let digits_start = match s.chars().next().unwrap() {
            '-' => {
                sign = Sign::Negative;
                1
            },
            _ => 0,
        };

        let digits: Result<Vec<u8>, _> = s[digits_start..].chars().rev().map(|c| {
            c.to_digit(10).ok_or("Invalid digit").map(|digit| digit as u8)
        }).collect();

        digits.map(|digits| BigInt { digits, sign })
    }

    fn to_string(&self) -> String {
        let sign_str = match self.sign {
            Sign::Positive => "",
            Sign::Negative => "-",
        };
        let digits_str: String = self.digits.iter().rev().map(|digit| digit.to_string()).collect::<Vec<_>>().join("");
        if digits_str.is_empty() {
            return "0".to_string();
        }
        format!("{}{}", sign_str, digits_str)
    }

    fn add(&self, b: &BigInt) -> BigInt {
        match (self.sign, b.sign) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => {
                let mut res = self.clone();
                res.digits.resize(std::cmp::max(self.digits.len(), b.digits.len()) + 1, 0);
                let mut carry = 0;
                for (a, b) in res.digits.iter_mut().zip(b.digits.iter().cloned().chain(std::iter::repeat(0))) {
                    *a += b + carry;
                    if *a >= 10 {
                        *a -= 10;
                        carry = 1;
                    } else {
                        carry = 0;
                    }
                }
                if carry > 0 {
                    res.digits.push(carry);
                }
                res.normalize();
                res
            },
            (Sign::Positive, Sign::Negative) => self.sub(&BigInt { sign: Sign::Positive, digits: b.digits.clone() }),
            (Sign::Negative, Sign::Positive) => b.sub(&BigInt { sign: Sign::Positive, digits: self.digits.clone() }),
        }
    }

    fn sub(&self, b: &BigInt) -> BigInt {
        match (self.sign, b.sign) {
            (Sign::Positive, Sign::Positive) => match self.cmp_abs(b) {
                Ordering::Greater => self.sub_abs(b),
                Ordering::Equal => BigInt::new(),
                Ordering::Less => {
                    let mut res = b.sub_abs(self);
                    res.sign = Sign::Negative;
                    res
                },
            },
            (Sign::Positive, Sign::Negative) => self.add(&BigInt { sign: Sign::Positive, digits: b.digits.clone() }),
            (Sign::Negative, Sign::Positive) => {
                let mut res = self.add(&BigInt { sign: Sign::Negative, digits: b.digits.clone() });
                res.sign = Sign::Negative;
                res
            },
            (Sign::Negative, Sign::Negative) => match self.cmp_abs(b) {
                Ordering::Greater => {
                    let mut res = self.sub_abs(b);
                    res.sign = Sign::Negative;
                    res
                },
                Ordering::Equal => BigInt::new(),
                Ordering::Less => b.sub_abs(self),
            },
        }
    }

    // Helper methods
    fn cmp_abs(&self, other: &BigInt) -> Ordering {
        match self.digits.len().cmp(&other.digits.len()) {
            Ordering::Equal => self.digits.iter().rev().zip(other.digits.iter().rev()).find_map(|(a, b)| {
                match a.cmp(b) {
                    Ordering::Equal => None,
                    non_eq => Some(non_eq),
                }
            }).unwrap_or(Ordering::Equal),
            non_eq => non_eq,
        }
    }

    fn sub_abs(&self, other: &BigInt) -> BigInt {
        let mut res = self.clone();
        let mut borrow = 0;
        for (a, b) in res.digits.iter_mut().zip(other.digits.iter().cloned().chain(std::iter::repeat(0))) {
            let b = b + borrow;
            borrow = if *a < b {
                *a += 10;
                1
            } else {
                0
            };
            *a -= b;
        }
        while let Some(&last) = res.digits.last() {
            if last == 0 {
                res.digits.pop();
            } else {
                break;
            }
        }
        res
    }

    fn normalize(&mut self) {
        while let Some(&last) = self.digits.last() {
            if last == 0 {
                self.digits.pop();
            } else {
                break;
            }
        }
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(BigInt::from("123").unwrap().add(&BigInt::from("456").unwrap()), BigInt::from("579").unwrap());
        assert_eq!(BigInt::from("123").unwrap().add(&BigInt::from("-456").unwrap()), BigInt::from("-333").unwrap());
        assert_eq!(BigInt::from("-123").unwrap().add(&BigInt::from("456").unwrap()), BigInt::from("333").unwrap());
        assert_eq!(BigInt::from("-123").unwrap().add(&BigInt::from("-456").unwrap()), BigInt::from("-579").unwrap());
        assert_eq!(BigInt::from("1145141919810").unwrap().add(&BigInt::from("1145141919810").unwrap()), BigInt::from("2290283839620").unwrap());
    }

    #[test]
    #[should_panic]
    fn test_invalid_add() {
        BigInt::from("").unwrap().add(&BigInt::from("456").unwrap());
    }
}