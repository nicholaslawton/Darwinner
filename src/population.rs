use crate::individual::Individual;
use nonempty::NonEmpty;
use rand::seq::IteratorRandom;
use rand::{random, Rng};
use std::num::NonZeroUsize;

pub struct Population(NonEmpty<Individual>);

impl Population {
    pub fn initialise(size: NonZeroUsize) -> Population {
        Population(NonEmpty {
            head: random(),
            tail: (2..size.get()).map(|_| random()).collect(),
        })
    }

    pub fn choose_individuals<R>(&self, rng: &mut R, n: usize) -> Vec<&Individual>
    where
        R: Rng + ?Sized,
    {
        self.0.iter().choose_multiple(rng, n)
    }
}
