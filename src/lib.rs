mod domain;
mod situation;
mod tactician;

pub use domain::Domain;
pub use situation::Situation;

use tactician::Tactician;

pub fn evolve(_domain: &impl Domain) -> Tactician {
    rand::random()
}
