

pub struct Executor;

impl Executor {

    pub fn and(&self, ope1: u64, ope2: u64, dest: &mut u64) {
        println!("Je suis le and");
        *dest = ope1 & ope2;
    }

    pub fn orr(&self, ope1: u64, ope2: u64, dest: &mut u64) {
        println!("Je suis le or");
        *dest = ope1 | ope2;
    }

    pub fn eor(&self, ope1: u64, ope2: u64, dest: &mut u64) {
        println!("Je suis le xor");
        *dest = ope1 ^ ope2;
    }

    fn safe_add(&self, ope1: u64, ope2: u64, dest: &mut u64, carry: &mut bool) {
        match ope1.checked_add(ope2) {
            Some(x) => {
                *dest = x;
                *carry = false;
            }
            None => {
                *dest = ope1.wrapping_add(ope2);
                *carry = true;
            }
        }
    }

    pub fn add(&self, ope1: u64, ope2: u64, dest: &mut u64, carry: &mut bool) {
        println!("je suis le add");
        self.safe_add(ope1, ope2, dest, carry);
    }
    
    pub fn adc(&self, ope1: u64, ope2: u64, dest: &mut u64, carry: &mut bool) {
        println!("Je suis le add with carry");
        self.safe_add(ope1, *carry as u64, dest, carry);
        self.safe_add(*dest, ope2, dest, carry);
    }

    pub fn cmp(&self, ope1: u64, ope2: u64, dest: &mut u64) {
        println!("Je suis le cmp");
    }

    fn safe_sub(&self, ope1: u64, ope2: u64, dest: &mut u64, carry: &mut bool) {
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

    pub fn sub(&self, ope1: u64, ope2: u64, dest: &mut u64, carry: &mut bool) {
        println!("je suis le sub");
        self.safe_sub(ope1, ope2, dest, carry);
    }

    pub fn sbc(&self, ope1: u64, ope2: u64, dest: &mut u64, carry: &mut bool) {
        println!("Je suis le sub with carry");
        self.safe_sub(ope1, 1 - *carry as u64, dest, carry);
        self.safe_sub(*dest, ope2, dest, carry);
    }

    pub fn mov(&self, value: u64, dest: &mut u64) {
        println!("je suis le mov");
        *dest = value;
    }

    pub fn lsh(&self, ope1: u64, ope2: u64, dest: &mut u64, carry: &mut bool) {
        println!("Je suis le left shift");
        if ope2 <= 64 {
            *carry = (ope1 >> 64 - ope2) & 0b1 == 1;
            *dest = ope1 << ope2;
        } else {
            *carry = false;
            *dest = 0;
        }
    }

    pub fn rsh(&self, ope1: u64, ope2: u64, dest: &mut u64, carry: &mut bool) {
        println!("Je suis le right shift");
        if ope2 <= 64 {
            *carry = (ope1 >> (ope2 - 1)) & 0b1 == 1;
            *dest = ope1 >> ope2;
        } else {
            *carry = false;
            *dest = 0;
        }
    }

}