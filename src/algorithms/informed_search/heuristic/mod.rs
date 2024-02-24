use crate::Board;

pub mod inversion_distance;
pub mod mahattan_distance;
pub mod pattern_database;

pub trait HeuristicFn {
    fn compute(&self, new_state: &Board, old_idx: Option<&Board>) -> usize;
}
