use std::collections::LinkedList;

use crate::Board;

pub mod informed_search;
pub mod uninformed_search;

pub trait Algorithms {
    fn solve(&mut self, initial_state: Board) -> Option<LinkedList<Board>>;
}
