mod algorithms;
mod board;
pub mod start_up;

pub use algorithms::{
    informed_search::{
        heuristic::inversion_distance, heuristic::mahattan_distance, AStarAlgorithms,
    },
    uninformed_search::{dfs::DfsAlgorithms, ucs::UcsAlgorithms},
    Algorithms,
};
pub use board::{
    board::{Board, BoardBuilder, Temperature},
    BoardConfig, BoardConfigBuilder,
};
