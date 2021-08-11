use std::ops::*;
use num_traits::{Num, NumOps, NumAssignOps, NumRef, NumAssignRef};

#[derive(Clone, Copy, Debug)]
pub struct Ratio {
    pub num: u128,
    pub den: u128,
}

pub fn gcd(mut a: u128, mut b: u128) -> u128 {
    loop {
        if a == 0 { return b; }
        let tmp = b % a;
        b = a;
        a = tmp;
    }
}

fn glcm(a: u128, b: u128) -> u128 {
    a/gcd(a,b)*b
}

impl Ratio {
    pub fn simplify(&mut self) {
        let gc = gcd(self.num, self.den);
        self.num /= gc;
        self.den /= gc;
    }

    pub fn inv(&self) -> Ratio {
        Ratio {
            num: self.den,
            den: self.num,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn addtwo() {
        let t = crate::ratio::Ratio { num: 1, den: 1 } + crate::ratio::Ratio { num: 1, den: 2 };
        assert!(t.num == 3 && t.den == 2);

        let t = crate::ratio::Ratio { num: 668, den: 15 } + crate::ratio::Ratio { num: 736, den: 5 };
        assert!(t.num == 2876 && t.den == 15);
    }

    #[test]
    fn mul() {
        let t = crate::ratio::Ratio { num: 160, den: 3 } * crate::ratio::Ratio { num: 936, den: 7 };
        assert!(t.num == 49920 && t.den == 7);
        let p = t.inv();
        assert!(p.den == 49920 && p.num == 7);
    }
}

impl Add for Ratio {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let lc = glcm(other.den, self.den);
        let f1 = lc / self.den;
        let f2 = lc / other.den;

        let mut res = Ratio {
            num: self.num * f1 + other.num * f2,
            den: lc,
        };
        res.simplify();

        res
    }
}

impl Sub for Ratio {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let lc = glcm(other.den, self.den);
        let f1 = lc / self.den;
        let f2 = lc / other.den;

        let mut res = Ratio {
            num: self.num * f1 - other.num * f2,
            den: lc,
        };
        res.simplify();

        res
    }
}

impl Mul for Ratio {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut res = Ratio {
            num: self.num * rhs.num,
            den: self.den * rhs.den,
        };
        res.simplify();

        res
    }
}

impl Div for Ratio {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut res = Ratio {
            num: self.num * rhs.den,
            den: self.den * rhs.num,
        };
        res.simplify();

        res
    }
}
