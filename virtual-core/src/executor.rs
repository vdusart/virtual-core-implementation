

pub struct Executor;

impl Executor {

    pub fn and(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        println!("Je suis le and");
        *dest = ope1 & ope2;
    }

    pub fn orr(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        println!("Je suis le or");
        *dest = ope1 | ope2;
    }

    pub fn eor(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        println!("Je suis le xor");
        *dest = ope1 ^ ope2;
    }

    pub fn add(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        println!("je suis le add");
        *dest = ope1 + ope2;
    }

    pub fn adc(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        println!("Je suis le add with carry");
    }

    pub fn cmp(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        println!("Je suis le cmp");
    }

    pub fn sub(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        println!("je suis le sub");
        *dest = ope1 - ope2;
    }

    pub fn sbc(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        println!("Je suis le sub with carry");
    }

    pub fn mov(&self, value: i64, dest: &mut i64) {
        println!("je suis le mov");
        *dest = value;
    }

    pub fn lsh(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        println!("Je suis le left shift");
        *dest = ope1 << ope2;
    }

    pub fn rsh(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        println!("Je suis le right shift");
        *dest = ope1 >> ope2;
    }

}