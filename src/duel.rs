use crate::individual::Individual;
use crate::situation::Situation;
use either::Either;

#[derive(PartialEq)]
enum Player {
    First,
    //Second,
}

enum DuelResolution {
    //Conclusive(Player),
    Inconclusive,
}

fn next_turn(situation: &Situation, active_player: Player) -> Either<DuelResolution, &Situation> {
    if active_player == Player::First {
        Either::Left(DuelResolution::Inconclusive)
    } else {
        Either::Right(situation)
    }
}

pub fn execute<'a>(
    initial_situation: &Situation,
    a: &'a Individual,
    b: &'a Individual,
) -> (&'a Individual, &'a Individual) {
    let situation: &Situation = initial_situation;
    let active_player: Player = Player::First;
    next_turn(situation, active_player);

    (a, b)
}
