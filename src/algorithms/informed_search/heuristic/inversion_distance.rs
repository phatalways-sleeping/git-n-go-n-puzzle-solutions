use std::{
    cell::RefCell,
    cmp::{max, min},
    collections::HashMap,
};

use crate::{
    board::{board::BoardManager, cell::Tile},
    Board,
};

use super::HeuristicFn;

pub struct InversionDistance {
    cache: RefCell<HashMap<Vec<Tile>, (usize, usize)>>,
}

impl InversionDistance {
    pub fn new() -> Self {
        Self {
            cache: RefCell::new(HashMap::new()),
        }
    }
    fn transpose(&self, tiles: &[Tile], n: usize) -> Vec<Tile> {
        let mut result = Vec::new();

        let mut idx = 0;

        while result.len() != tiles.len() {
            result.push(tiles[idx].clone());
            idx += n;
            if idx > tiles.len() - 1 {
                idx %= tiles.len() - 1;
            }
        }

        result
    }

    fn inversion_count(&self, board: &Board, vertical: bool) -> usize {
        let order = self.compute_order(board, vertical);
        let mut tiles = BoardManager::tiles_of(board).to_vec();
        if vertical {
            tiles = self.transpose(&tiles, BoardManager::size_of(board) as usize);
        }
        let mut count = 0;
        for i in 0..tiles.len() {
            if tiles[i].get_value() == 0 {
                continue;
            }
            let order_i = order.get(&tiles[i].get_value()).unwrap();
            for j in (i + 1)..tiles.len() {
                if tiles[j].get_value() == 0 {
                    continue;
                }
                let order_j = order.get(&tiles[j].get_value()).unwrap();
                if order_i > order_j {
                    count += 1;
                }
            }
        }
        count
    }

    fn compute_order(&self, board: &Board, vertical: bool) -> HashMap<u8, u8> {
        let n = BoardManager::size_of(board);
        let mut goal = BoardManager::goal_of(board).to_vec();
        let mut result = HashMap::new();
        if vertical {
            goal = self.transpose(&goal, n as usize);
        }
        for i in 0..(n * n) {
            result.insert(goal[i as usize].get_value(), i);
        }
        result
    }
}

impl HeuristicFn for InversionDistance {
    fn compute(&self, new_state: &Board, old_state: Option<&Board>) -> usize {
        // Still bugs
        match old_state {
            None => {
                let horizontal = self.inversion_count(new_state, false);
                let vertical = self.inversion_count(new_state, true);

                let n = BoardManager::size_of(new_state) as usize;

                let tiles = BoardManager::tiles_of(new_state).to_vec();

                self.cache
                    .borrow_mut()
                    .insert(tiles, (horizontal, vertical));

                let heuristic = (horizontal / (n - 1) + horizontal % (n - 1))
                    + (vertical / (n - 1) + vertical % (n - 1));

                heuristic
            }
            Some(old_state) => {
                let mut old_idx = BoardManager::empty_tile_idx(old_state) as usize;
                let mut new_idx = BoardManager::empty_tile_idx(new_state) as usize;

                let old_tiles = BoardManager::tiles_of(old_state).to_vec();
                let mut new_tiles = BoardManager::tiles_of(new_state).to_vec();

                let (horizontal, vertical) = *self.cache.borrow().get(&old_tiles).unwrap();

                // horizontal swaps affect the vertical inversion counts
                // and vice versa
                let is_vertical = new_idx.abs_diff(old_idx) == 1;

                let mut change: isize = 0;
                let order = self.compute_order(new_state, is_vertical);

                if is_vertical {
                    new_tiles =
                        self.transpose(&new_tiles, BoardManager::size_of(new_state) as usize);
                    let n = BoardManager::size_of(new_state) as usize;
                    old_idx = (old_idx % n) * n + old_idx / n;
                    new_idx = (new_idx % n) * n + new_idx / n;
                }

                let start = min(old_idx, new_idx);
                let end = max(old_idx, new_idx);

                let swapped_tile_order = order.get(&new_tiles[old_idx].get_value()).unwrap();
                for i in (start + 1)..end {
                    if order.get(&new_tiles[i].get_value()).unwrap() < swapped_tile_order {
                        change += 1;
                    }
                }

                if end == old_idx {
                    change *= -1;
                }

                let horizontal = if !is_vertical {
                    horizontal as isize + change
                } else {
                    horizontal as isize
                } as usize;

                let vertical = if is_vertical {
                    vertical as isize + change
                } else {
                    vertical as isize
                } as usize;

                let n = BoardManager::size_of(new_state) as usize;

                let heuristic = (horizontal / (n - 1) + horizontal % (n - 1))
                    + (vertical / (n - 1) + vertical % (n - 1));

                let new_tiles = BoardManager::tiles_of(new_state).to_vec();

                self.cache
                    .borrow_mut()
                    .insert(new_tiles, (horizontal, vertical));

                heuristic
            }
        }
    }
}
