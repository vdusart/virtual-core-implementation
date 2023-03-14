

pub struct Executor;

impl Executor {
    pub fn add(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        println!("je suis le add");
        *dest = ope1 + ope2;
    }

    pub fn sub(&self, ope1: i64, ope2: i64, dest: &mut i64) {
        println!("je suis le sub");
        *dest = ope1 - ope2;
    }

    pub fn mov(&self, value: i64, dest: &mut i64) {
        println!("je suis le mov");
        *dest = value;
    }
}