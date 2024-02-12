#[derive(Clone)]
pub struct Number {
    sign: bool,
    num: u128,
    den: u128,
}

impl Number {
    pub fn new(s: bool, n: u128, d: u128) -> Number {
        Number {
            sign: s,
            num: n,
            den: d,
        }
    }
    
    
    fn reduce(&mut self) {
        let mut n = self.num;
        let mut d = self.den;
        
        while n != 0 && d != 0 {
            if n >= d {
                n = n % d;
            } else {
                d = d % n;
            }
        }
        
        n += d;
        
        self.num /= n;
        self.den /= n;
        
        // no negative zero!
        self.sign &= self.num != 0;
    }
    
    fn is_bitmask(&self) -> bool {
        self.sign == false && self.den == 1
    }
    
    fn _add(&mut self, sign: bool, other: &Number) {
        let onum = other.num * self.den;
        self.num *= other.den;
        self.den *= other.den;
        if sign {
            if self.num < onum {
                self.sign = !self.sign;
                self.num = onum - self.num;
            } else {
                self.num -= onum;
            }
        } else {
            self.num += onum;
        }
    }
    
    
    pub fn neg(&mut self) {
        self.sign = !self.sign;
    }
    
    pub fn add(&mut self, other: &Number) {
        if other.sign {
            self._add(!self.sign, other);
        } else {
            self._add(self.sign, other);
        }
        self.reduce();
    }
    
    pub fn sub(&mut self, other: &Number) {
        if other.sign {
            self._add(self.sign, other);
        } else {
            self._add(!self.sign, other);
        }
        self.reduce();
    }
    
    
    pub fn mul(&mut self, other: &Number) {
        if other.sign {
            self.sign = !self.sign;
        }
        self.num *= other.num;
        self.den *= other.den;
        self.reduce();
    }
    
    pub fn div(&mut self, other: &Number) -> Result<(), String> {
        if other.num != 0 {
            if other.sign {
                self.sign = !self.sign;
            }
            self.num *= other.den;
            self.den *= other.num;
            self.reduce();
            Ok(())
        } else {
            Err("division by zero".to_string())
        }
    }
    
    pub fn rem(&mut self, other: &Number) -> Result<(), String> {
        if other.num != 0 {
            if other.sign {
                self.sign = !self.sign;
            }
            if self.den == 1 && other.den == 1 {
                self.num %= other.num;
            } else {
                // what should even be here???
                return Err("cannot get remainder of a fraction".to_string());
            }
            self.reduce();
            Ok(())
        } else {
            Err("modulo by zero".to_string())
        }
    }
    
    
    pub fn not(&mut self) -> Result<(), String> {
        if self.is_bitmask() {
            self.num = !self.num;
            Ok(())
        } else {
            Err("cannot invert a fraction".to_string())
        }
    }
    
    pub fn and(&mut self, other: &Number) -> Result<(), String> {
        if self.is_bitmask() {
            self.num &= other.num;
            Ok(())
        } else {
            Err("cannot bitwise and a fraction".to_string())
        }
    }
    
    pub fn ior(&mut self, other: &Number) -> Result<(), String> {
        if self.is_bitmask() {
            self.num |= other.num;
            Ok(())
        } else {
            Err("cannot inclusive or a fraction".to_string())
        }
    }
    
    pub fn eor(&mut self, other: &Number) -> Result<(), String> {
        if self.is_bitmask() {
            self.num ^= other.num;
            Ok(())
        } else {
            Err("cannot exclusive or a fraction".to_string())
        }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        let s = self.sign;
        let n = self.num;
        let d = self.den;
        
        if d == 1 {
            if s {
                format!("-{n}")
            } else {
                n.to_string()
            }
        } else {
            let approx = n as f64 / d as f64;
            
            if s {
                format!("-{n}/{d} ~ -{approx}")
            } else {
                format!("{n}/{d} ~ {approx}")
            }
        }
    }
}
