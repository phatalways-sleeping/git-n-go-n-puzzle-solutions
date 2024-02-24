use std::collections::{BTreeMap, HashSet, LinkedList, VecDeque};

use crate::{board::board::BoardManager, Algorithms, Board};

pub struct UcsAlgorithms {
    visisted: HashSet<Board>,
    parents: BTreeMap<Board, Board>,
}

impl UcsAlgorithms {
    pub fn new() -> Self {
        Self {
            visisted: HashSet::new(),
            parents: BTreeMap::new(),
        }
    }
}

impl Algorithms for UcsAlgorithms {
    fn solve(&mut self, initial_state: Board) -> Option<std::collections::LinkedList<Board>> {
        let mut solutions: LinkedList<Board> = LinkedList::new();
        let mut queue = VecDeque::new();
        queue.push_back(initial_state);
        while let Some(current) = queue.pop_front() {
            if current.match_goal() {
                let mut current = current;
                solutions.push_front(current.clone());
                while let Some(parent) = self.parents.get(&current) {
                    solutions.push_front(parent.clone());
                    current = parent.clone();
                }
                return Some(solutions);
            }
            self.visisted.insert(current.clone());
            let neighbors_idx = BoardManager::neigbors_of(&current);
            neighbors_idx
                .into_iter()
                .filter(|board| !self.visisted.contains(board))
                .for_each(|board| {
                    self.parents.insert(board.clone(), current.clone());
                    queue.push_back(board);
                });
        }
        None
    }
}
