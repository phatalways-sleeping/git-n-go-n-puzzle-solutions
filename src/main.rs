use n_puzzle_trial::{inversion_distance::InversionDistance, start_up::run, AStarAlgorithms};

fn main() {
    let n = 4;

    let mut goal_state: Vec<u8> = (1..(n * n)).collect();

    goal_state.push(0);

    run(
        Box::new(AStarAlgorithms::with(Box::new(InversionDistance::new()))),
        n,
        goal_state.to_vec(),
        true,
    );
}
