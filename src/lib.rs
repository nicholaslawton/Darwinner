mod domain;
mod environment;
mod individual;
mod population;
mod situation;

pub use domain::Domain;
pub use environment::Environment;
pub use situation::Situation;

use individual::Individual;
use population::Population;

pub fn evolve(_domain: &impl Domain, env: &Environment) -> Individual {
    let pop = Population::initialise(env.population_size);
    *pop.most_successful()
}
