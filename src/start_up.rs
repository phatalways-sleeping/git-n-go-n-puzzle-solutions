use crate::{Algorithms, BoardBuilder, BoardConfigBuilder, Temperature};

pub fn run(mut algorithms: Box<dyn Algorithms>, n: u8, goal_state: Vec<u8>, informed: bool) {
    let mut builder = BoardConfigBuilder::builder().essential(n, goal_state);

    if informed {
        builder = builder.with_informed();
    }

    let config = builder.build().expect("Fail to build config");

    let board = BoardBuilder::builder()
        .config(config)
        .temperature(Temperature(0.1))
        .build()
        .expect("Fail to build board");
    let answers = algorithms.solve(board).expect("No solution");

    answers.iter().for_each(|answer| println!("{}", answer));
}
