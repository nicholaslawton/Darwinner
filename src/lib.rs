pub trait Domain {}

pub struct Situation(pub Vec<u8>);

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Evaluation(u32);

pub struct Tactician {}

impl Tactician {
    pub fn evaluate(&self, _situation: &Situation) -> Evaluation {
        Evaluation(0)
    }
}

pub fn evolve(_domain: &impl Domain) -> Tactician {
    Tactician {}
}
