use darwinner::{evolve, Domain, Environment, Situation};
use std::num::NonZeroUsize;

struct Trivial {}

impl Trivial {
    fn new() -> Trivial {
        Trivial {}
    }
}

impl Domain for Trivial {}

struct State {
    player: u8,
    opponent: u8,
}

impl State {
    fn situation(&self) -> Situation {
        Situation::from_binary(vec![self.player, self.opponent])
    }
}

#[test]
fn most_successful_tactician_evaluation_correlates_directly_with_score_every_time() {
    let domain: Trivial = Trivial::new();
    let env: Environment = Environment {
        population_size: NonZeroUsize::new(100).unwrap(),
    };
    let ordered_scores: Vec<State> = (0..10u8)
        .map(|x| State {
            player: x,
            opponent: 0,
        })
        .collect();
    (1..10)
        .map(|_| evolve(&domain, &env))
        .for_each(|most_successful| {
            let evaluations = ordered_scores
                .iter()
                .map(|x| most_successful.evaluate(&x.situation()));
            evaluations.reduce(|n, x| {
                assert!(x > n, "{:?} > {:?}", x, n);
                x
            });
        });
}
