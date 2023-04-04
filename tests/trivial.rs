use darwinner::{evolve, Domain, Environment, Situation};
use std::num::NonZeroUsize;

struct State {
    player: u8,
    opponent: u8,
}

impl State {
    const INITIAL: State = State {
        player: 0,
        opponent: 0,
    };

    fn situation(&self) -> Situation {
        Situation::from_binary(vec![self.player, self.opponent])
    }
}

fn domain() -> Domain {
    Domain {
        initial_situation: State::INITIAL.situation(),
    }
}

#[test]
fn most_successful_tactician_evaluation_correlates_directly_with_score_every_time() {
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
        .map(|_| evolve(&domain(), &env))
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
