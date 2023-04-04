use crate::contest::Contest;
use crate::domain::Domain;
use crate::environment::Environment;
use crate::individual::Individual;
use crate::population::Population;

pub fn evolve(domain: &Domain, env: &Environment) -> Individual {
    let mut rng = rand::thread_rng();
    let pop = Population::initialise(env.population_size);
    let contest = Contest::select_contestants(&mut rng, &pop);
    let result = contest.resolve(&domain.initial_situation);
    *result.mating_pair.0
}
