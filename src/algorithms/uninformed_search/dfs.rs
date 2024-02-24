use std::collections::{BTreeMap, HashSet, LinkedList};

use crate::{board::board::BoardManager, Algorithms, Board};

pub struct DfsAlgorithms {
    visited: HashSet<Board>,
    parents: BTreeMap<Board, Board>,
}

impl DfsAlgorithms {
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
            parents: BTreeMap::new(),
        }
    }
}

impl Algorithms for DfsAlgorithms {
    fn solve(&mut self, initial_state: Board) -> Option<std::collections::LinkedList<Board>> {
        let mut solutions: LinkedList<Board> = LinkedList::new();
        let mut stack: LinkedList<Board> = LinkedList::new();
        stack.push_back(initial_state);
        while let Some(mut current) = stack.pop_back() {
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
                .for_each(|board| {
                    self.parents.insert(board.clone(), current.clone());
                    stack.push_back(board);
                });
        }
        None
    }
}
