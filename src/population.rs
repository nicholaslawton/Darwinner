use crate::tactician::Tactician;
use nonempty::NonEmpty;
use rand::random;
use std::num::NonZeroUsize;

pub struct Population(NonEmpty<Tactician>);

impl Population {
    pub fn initialise(size: NonZeroUsize) -> Population {
        Population(NonEmpty {
            head: random(),
            tail: (2..size.get()).map(|_| random()).collect(),
        })
    }

    pub fn most_successful(&self) -> &Tactician {
        let Population(tacticians) = self;
        &tacticians.head
    }
}
