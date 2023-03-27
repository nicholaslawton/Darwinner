use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::situation::Situation;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Evaluation(f64);

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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
