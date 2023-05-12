use crate::keywords::Flags;
use std::cmp::min;

pub struct Executor;

// Implementation of all possibe operations
impl Executor {
    pub fn and(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        *dest = ope1 & ope2;
    }

    pub fn orr(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        *dest = ope1 | ope2;
    }

    pub fn eor(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        *dest = ope1 ^ ope2;
    }

    // addition with carry set in case of overflow
    fn safe_add(&self, ope1: i64, ope2: i64, dest: &mut i64, carry: &mut bool) {
        *dest = ope1.wrapping_add(ope2);
        *carry = (ope1.is_negative() && ope2.is_negative())
            || min(ope1.leading_zeros(), ope2.leading_zeros()) < dest.leading_zeros();
    }

    pub fn add(&self, ope1: i64, ope2: i64, dest: &mut i64, carry: &mut bool) {
        self.safe_add(ope1, ope2, dest, carry);
    }

    pub fn adc(&self, ope1: i64, ope2: i64, dest: &mut i64, carry: &mut bool) {
        self.safe_add(ope1, *carry as i64, dest, carry);
        self.safe_add(*dest, ope2, dest, carry);
    }

    pub fn cmp(&self, ope1: i64, ope2: i64, flags: &mut Flags) {
        flags.beq = ope1 == ope2;
        flags.bne = ope1 != ope2;
        flags.ble = ope1 <= ope2;
        flags.bge = ope1 >= ope2;
        flags.bl = ope1 < ope2;
        flags.bg = ope1 > ope2;
    }

    // substraction with carry set in case of underflow
    fn safe_sub(&self, ope1: i64, ope2: i64, dest: &mut i64, carry: &mut bool) {
        match ope1.checked_sub(ope2) {
            Some(x) => {
                *dest = x;
                *carry = false;
            }
            None => {
                *dest = ope1.wrapping_sub(ope2);
                *carry = true;
            }
        }
    }

    pub fn sub(&self, ope1: i64, ope2: i64, dest: &mut i64, carry: &mut bool) {
        self.safe_sub(ope1, ope2, dest, carry);
    }

    pub fn sbc(&self, ope1: i64, ope2: i64, dest: &mut i64, carry: &mut bool) {
        self.safe_sub(ope1, 1 - *carry as i64, dest, carry);
        self.safe_sub(*dest, ope2, dest, carry);
    }

    pub fn mov(&self, value: i64, dest: &mut i64) {
        *dest = value;
    }

    pub fn lsh(&self, ope1: i64, ope2: i64, dest: &mut i64, carry: &mut bool) {
        if ope2 <= 64 {
            *carry = (ope1 >> 64 - ope2) & 0b1 == 1;
            *dest = ope1 << ope2;
        } else {
            *carry = false;
            *dest = 0;
        }
    }

    pub fn rsh(&self, ope1: i64, ope2: i64, dest: &mut i64, carry: &mut bool) {
        if ope2 <= 64 {
            *carry = (ope1 >> (ope2 - 1)) & 0b1 == 1;
            *dest = ope1 >> ope2;
        } else {
            *carry = false;
            *dest = 0;
        }
    }
}
