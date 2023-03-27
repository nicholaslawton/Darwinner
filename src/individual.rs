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
pub struct Individual {
    evaluation_rules: EvaluationExpression,
}

impl Individual {
    pub fn evaluate(&self, situation: &Situation) -> Evaluation {
        self.evaluation_rules.evaluate(situation)
    }
}

impl Distribution<Individual> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Individual {
        Individual {
            evaluation_rules: rng.gen(),
        }
    }
}
