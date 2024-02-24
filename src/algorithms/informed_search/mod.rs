use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList};

use crate::board::board::{Board, BoardManager};

use self::heuristic::HeuristicFn;

use super::Algorithms;

pub mod heuristic;

pub struct AStarAlgorithms {
    heuristic: Box<dyn HeuristicFn>,
    caching: HashMap<Board, usize>,
    visited: HashSet<Board>,
    parents: HashMap<Board, Board>,
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
        while let Some(mut current) = pq.pop() {
            if self.visited.contains(&current) {
                continue;
            }
            if current.match_goal() {
                solutions.push_front(current.clone());
                while let Some(parent) = self.parents.get(&current) {
                    solutions.push_front(parent.clone());
                    current = parent.clone();
                }
                return Some(solutions);
            }
            self.visited.insert(current.clone());
            BoardManager::neigbors_of(&current)
                .into_iter()
                .filter(|board| !self.visited.contains(board))
                .map(|board| {
                    let weight = self.heuristic.compute(
                        &board,
                        Some(&current),
                    );
                    BoardManager::assign_weight(board, weight)
                })
                .for_each(|board| {
                    let weight = BoardManager::total_weight(&board);
                    if let Some(value) = self.caching.get_mut(&board) {
                        if *value > weight {
                            *value = weight;
                            self.parents.insert(board.clone(), current.clone());
                            pq.push(board);
                        }
                    } else {
                        self.caching.insert(board.clone(), weight);
                        self.parents.insert(board.clone(), current.clone());
                        pq.push(board);
                    }
                });
        }
        None
    }
}
