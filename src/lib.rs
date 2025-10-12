const BASE: u32 = 1_000_000_000;

pub struct BigInt {
    neg: bool,
    digits: Vec<u32>, // little endian; digits[0] is least significat limb
}

impl BigInt {
    pub fn zero() -> Self {
        Self {
            neg: false,
            digits: Vec::new(),
        }
    }

    pub fn one() -> Self {
        Self {
            neg: false,
            digits: vec![1],
        }
    }
    pub fn from_i64(v: i64) -> Self {
        if v == 0 {
            return Self::zero();
        }

        let mut x = v.unsigned_abs();

        let mut digits = Vec::new();
        while x > 0 {
            digits.push((x % BASE as u64) as u32);
            x /= BASE as u64;
        } 
        Self {
            neg: v < 0, digits
        }.normalize()
    }

    fn normalize(mut self) -> Self {
        while self.digits.last().map_or(false, |&d| d == 0) { 
            self.digits.pop();
        }

        if self.digits.is_empty(){
            self.neg = false; 
        }

        self
    }

}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("vec------: {:?}", vec![10,1]);
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn from_i64() {
       let v = BigInt::from_i64(1002323809800980);
       for (i, &n) in [809800980, 1002323].iter().enumerate() {
        assert_eq!(n, v.digits[i]);
       }
    }
}
