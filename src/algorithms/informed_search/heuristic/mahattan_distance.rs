use crate::{board::board::BoardManager, Board};

use super::HeuristicFn;

pub fn mahattan_distance(board: &Board, _old_idx: Option<usize>) -> usize {
    let n = BoardManager::size_of(board) as usize;
    BoardManager::tiles_of(board)
        .iter()
        .enumerate()
        .map(|(idx, tile)| {
            let value = tile.get_value() as usize;
            if value == 0 {
                return 0;
            }
            let dx = ((value - 1) % n).abs_diff(idx % n);
            let dy = (value / n).abs_diff(idx / n);
            dx + dy
        })
        .reduce(|acc, value| acc + value)
        .unwrap()
}

pub struct MahattanDistance;

impl HeuristicFn for MahattanDistance {
    fn compute(&self, new_state: &Board, old_state: Option<&Board>) -> usize {
        let n = BoardManager::size_of(new_state) as usize;
        let tiles = BoardManager::tiles_of(new_state);
        match old_state {
            None => tiles
                .iter()
                .enumerate()
                .map(|(idx, tile)| {
                    let value = tile.get_value() as usize;
                    if value == 0 {
                        return 0;
                    }
                    let dx = ((value - 1) % n).abs_diff(idx % n);
                    let dy = (value / n).abs_diff(idx / n);
                    dx + dy
                })
                .reduce(|acc, value| acc + value)
                .unwrap(),
            Some(old_state) => {
                let old_idx = BoardManager::empty_tile_idx(old_state) as usize;
                let moved_tile_value =
                    BoardManager::tiles_of(new_state)[old_idx].get_value() as usize;
                let dx = ((moved_tile_value - 1) % n).abs_diff(old_idx % n);
                let dy = (moved_tile_value / n).abs_diff(old_idx / n);
                let change = dx + dy;
                BoardManager::heuristic_value_of(new_state) + change
            }
        }
    }
}
