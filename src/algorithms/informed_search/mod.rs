use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList};

use crate::board::{
    board::{Board, BoardManager},
    cell::Tile,
};

use self::heuristic::HeuristicFn;

use super::Algorithms;

pub mod heuristic;

pub struct AStarAlgorithms {
    heuristic: Box<dyn HeuristicFn>,
    caching: HashMap<Vec<Tile>, usize>,
    visited: HashSet<Vec<Tile>>,
    parents: HashMap<Vec<Tile>, Board>,
}

impl AStarAlgorithms {
    pub fn with(heuristic: Box<dyn HeuristicFn>) -> Self {
        Self {
            heuristic,
            caching: HashMap::new(),
            visited: HashSet::new(),
            parents: HashMap::new(),
        }
    }
}

impl Algorithms for AStarAlgorithms {
    fn solve(&mut self, initial_state: Board) -> Option<std::collections::LinkedList<Board>> {
        let mut solutions = LinkedList::new();
        let mut pq = BinaryHeap::new();
        // initialize weight for initial_state
        let weight = self.heuristic.compute(&initial_state, None);
        let initial_state = BoardManager::assign_weight(initial_state, weight);
        pq.push(initial_state);
        while let Some(current) = pq.pop() {
            let tiles = BoardManager::tiles_of(&current).to_vec();
            if self.visited.contains(&tiles) {
                continue;
            }
            if current.match_goal() {
                solutions.push_front(current.clone());
                let mut tiles = tiles;
                while let Some(parent) = self.parents.get(&tiles) {
                    solutions.push_front(parent.clone());
                    tiles = BoardManager::tiles_of(parent).to_vec();
                }
                return Some(solutions);
            }
            self.visited.insert(tiles);
            BoardManager::neigbors_of(&current)
                .into_iter()
                .filter(|board| !self.visited.contains(BoardManager::tiles_of(board)))
                .map(|board| {
                    let weight = self.heuristic.compute(&board, Some(&current));
                    BoardManager::assign_weight(board, weight)
                })
                .for_each(|board| {
                    let weight = BoardManager::total_weight(&board);
                    let tiles = BoardManager::tiles_of(&board).to_vec();
                    if let Some(value) = self.caching.get_mut(&tiles) {
                        if *value > weight {
                            *value = weight;
                            self.parents.insert(tiles, current.clone());
                            pq.push(board);
                        }
                    } else {
                        self.caching.insert(tiles.clone(), weight);
                        self.parents.insert(tiles, current.clone());
                        pq.push(board);
                    }
                });
        }
        None
    }
}
