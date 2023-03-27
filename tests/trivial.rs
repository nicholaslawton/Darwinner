use darwinner::{evolve, Domain, Situation};

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
fn most_successful_tactician_evaluation_correlates_directly_with_score() {
    let domain: Trivial = Trivial::new();
    let most_successful = evolve(&domain);
    println!("most_successful: {:?}", most_successful);
    let ordered_scores = (0..10u8).map(|x| State {
        player: x,
        opponent: 0,
    });
    let evaluations = ordered_scores.map(|x| most_successful.evaluate(&x.situation()));
    evaluations.reduce(|n, x| {
        assert!(x > n, "{:?} > {:?}", x, n);
        x
    });
}
