pub struct Situation(Vec<u8>);

impl Situation {
    pub fn from_binary(bytes: Vec<u8>) -> Situation {
        Situation(bytes)
    }

    pub fn read(&self, n: &usize) -> u8 {
        let Situation(s) = self;
        match s.get(n % s.len()) {
            Some(x) => *x,
            None => panic!("Core function failure: Vec::get returned None for index obtained using the remainder operator with Vec::len"),
        }
    }
}
