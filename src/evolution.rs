use crate::domain::Domain;
use crate::environment::Environment;
use crate::individual::Individual;
use crate::population::Population;

pub fn evolve(_domain: &impl Domain, env: &Environment) -> Individual {
    let pop = Population::initialise(env.population_size);
    *pop.most_successful()
}
