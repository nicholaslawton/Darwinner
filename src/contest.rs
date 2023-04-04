use crate::duel;
use crate::individual::Individual;
use crate::population::Population;
use crate::situation::Situation;
use rand::Rng;

pub struct Contest<'a>([&'a Individual; 4]);

impl<'a> Contest<'a> {
    pub fn select_contestants<R>(rng: &mut R, pop: &'a Population) -> Contest<'a>
    where
        R: Rng + ?Sized,
    {
        let contestants = pop.choose_individuals(rng, 4);
        match contestants[..] {
            [a, b, c, d] => Contest([a, b, c, d]),
            _ => panic!(
                "Population::choose_individuals(rng, 4) returned {} contestants",
                contestants.len()
            ),
        }
    }

    pub fn resolve(&self, initial_duel_situation: &Situation) -> ContestResolution {
        let result_1a = duel::execute(initial_duel_situation, self.0[0], self.0[1]);
        let result_1b = duel::execute(initial_duel_situation, self.0[2], self.0[3]);
        ContestResolution {
            mating_pair: (result_1a.0, result_1b.0),
            //defeated: self.0[3],
        }
    }
}

pub struct ContestResolution<'a> {
    pub mating_pair: (&'a Individual, &'a Individual),
    //defeated: &'a Individual,
}
