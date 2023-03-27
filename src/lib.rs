mod domain;
mod environment;
mod population;
mod situation;
mod tactician;

pub use domain::Domain;
pub use situation::Situation;

pub use environment::Environment;
use population::Population;
use tactician::Tactician;

pub fn evolve(_domain: &impl Domain, env: &Environment) -> Tactician {
    let pop = Population::initialise(env.population_size);
    *pop.most_successful()
}
