use rand::distributions::{Distribution, Standard};
use rand::{random, Rng};

pub trait Domain {}

pub struct Situation(Vec<u8>);

impl Situation {
    pub fn from_binary(bytes: Vec<u8>) -> Situation {
        Situation(bytes)
    }

    fn read(&self, n: &usize) -> u8 {
        match self {
            Situation(s) =>
                match s.get(n % s.len()) {
                    Some(x) => *x,
                    None => panic!("Core function failure: Vec::get returned None for index obtained using the remainder operator with Vec::len"),
                }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Evaluation(f64);

#[derive(Clone, Debug)]
enum EvaluationExpression {
    Read(usize),
}

impl EvaluationExpression {
    fn evaluate(&self, situation: &Situation) -> Evaluation {
        match self {
            EvaluationExpression::Read(n) => Evaluation(situation.read(n).into()),
        }
    }
}

impl Distribution<EvaluationExpression> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EvaluationExpression {
        EvaluationExpression::Read(rng.gen())
    }
}

#[derive(Debug)]
pub struct Tactician {
    evaluation_rules: EvaluationExpression,
}

impl Tactician {
    pub fn evaluate(&self, _situation: &Situation) -> Evaluation {
        self.evaluation_rules.evaluate(_situation)
    }
}

impl Distribution<Tactician> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tactician {
        Tactician {
            evaluation_rules: rng.gen(),
        }
    }
}

pub fn evolve(_domain: &impl Domain) -> Tactician {
    random()
}
