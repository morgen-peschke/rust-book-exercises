use rand::Rng;

pub const MAX: i32 = 20;
pub const MIN: i32 = 1;

#[derive(Debug)]
pub struct Secret(i32);

impl Secret {
    pub fn random() -> Secret {
        Secret(rand::thread_rng().gen_range(MIN..=MAX))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}
