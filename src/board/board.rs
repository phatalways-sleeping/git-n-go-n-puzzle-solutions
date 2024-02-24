use std::{cmp::Ordering, fmt::Display, hash::Hash};

use super::{board_config::BoardConfig, cell::Tile};
use rand::{thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct Board {
    tiles: Vec<Tile>,
    config: BoardConfig,
}

impl Hash for Board {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tiles.hash(state);
    }
}

impl Eq for Board {}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Board {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match &self.config {
            BoardConfig::Uninformed(_) => self.tiles.cmp(&other.tiles),
            BoardConfig::Informed(config) => match &other.config {
                BoardConfig::Uninformed(_) => self.tiles.cmp(&other.tiles),
                BoardConfig::Informed(other_config) => {
                    if config.weight + config.depth < other_config.weight + other_config.depth {
                        Ordering::Greater
                    } else if config.weight + config.depth
                        == other_config.weight + other_config.depth
                    {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                }
            },
        }
    }
}

impl Board {
    fn from(mut config: BoardConfig, temperature: f32) -> Self {
        let (tiles, empty_tile_idx) =
            BoardManager::generate_random_board(config.n(), &config.goal_state(), temperature);

        config.set_empty_tile_idx(empty_tile_idx);

        Self { tiles, config }
    }

    pub fn match_goal(&self) -> bool {
        self.config.goal_state() == self.tiles
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: Vec<Vec<u8>> = self
            .tiles
            .chunks(self.config.n() as usize)
            .map(|tiles| tiles.iter().map(|tile| tile.get_value()).collect())
            .collect();
        for row in &s {
            for tile in row {
                write!(f, "{:3}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct Temperature(pub f32);

impl Temperature {
    pub fn from(temperature: f32) -> Result<Self, &'static str> {
        if temperature <= 0.0 || temperature > 1.0 {
            Err("Temperature: temperature must be within 0.0 and 1.0 (inclusive)")
        } else {
            Ok(Self(temperature))
        }
    }
}

pub struct BoardBuilder {
    config: Option<BoardConfig>,
    temperature: Option<Temperature>,
}

impl BoardBuilder {
    pub fn builder() -> Self {
        Self {
            config: None,
            temperature: None,
        }
    }

    pub fn config(mut self, config: BoardConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn temperature(mut self, temperature: Temperature) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn build(mut self) -> Result<Board, &'static str> {
        if self.config.is_none() {
            return Err("BoardBuilder: config must be provided");
        }
        Ok(Board::from(
            self.config.take().unwrap(),
            self.temperature.take().unwrap_or(Temperature(0.3)).0,
        ))
    }
}

pub struct BoardManager;

impl BoardManager {
    pub fn tiles_of(board: &Board) -> &[Tile] {
        &board.tiles
    }
    
    pub fn goal_of(board: &Board) -> &[Tile] {
        board.config.goal_state()
    }

    pub fn size_of(board: &Board) -> u8 {
        board.config.n()
    }

    pub fn empty_tile_idx(board: &Board) -> u8 {
        board.config.empty_tile_idx()
    }

    pub fn assign_weight(mut board: Board, weight: usize) -> Board {
        if let Some(value) = board.config.get_mut_weight() {
            *value = weight;
        }
        board
    }

    pub fn total_weight(board: &Board) -> usize {
        if board.config.get_depth().is_none() || board.config.get_weight().is_none() {
            0
        } else {
            *board.config.get_depth().unwrap() + *board.config.get_weight().unwrap()
        }
    }

    pub fn heuristic_value_of(board: &Board) -> usize {
        if board.config.get_depth().is_none() || board.config.get_weight().is_none() {
            0
        } else {
            *board.config.get_weight().unwrap()
        }
    }

    pub fn neigbors_of(board: &Board) -> Vec<Board> {
        let mut neighbors = Vec::<usize>::with_capacity(4);
        let idx = board.config.empty_tile_idx() as usize;
        let n = board.config.n() as usize;
        if let Some(left) = Self::move_left(idx, n) {
            neighbors.push(left)
        }
        if let Some(right) = Self::move_right(idx, n) {
            neighbors.push(right)
        }
        if let Some(up) = Self::move_up(idx, n) {
            neighbors.push(up)
        }
        if let Some(down) = Self::move_down(idx, n) {
            neighbors.push(down)
        }
        neighbors.shrink_to_fit();
        neighbors
            .into_iter()
            .map(|idx| Self::swap_empty_tile_with(idx, board))
            .collect()
    }

    fn swap_empty_tile_with(idx: usize, board: &Board) -> Board {
        let mut tiles = board.tiles.to_vec();
        tiles.swap(board.config.empty_tile_idx() as usize, idx);
        let mut config = board.config.clone();
        config.set_empty_tile_idx(idx as u8);
        if let Some(depth) = config.get_mut_depth() {
            *depth += 1;
        }
        Board { tiles, config }
    }

    fn generate_random_board(n: u8, goal_state: &[Tile], temperature: f32) -> (Vec<Tile>, u8) {
        let mut tiles: Vec<Tile> = goal_state.to_vec();

        let mut times = (temperature * 1000.0) as u16;

        let mut idx = tiles.len() - 1;

        let mut rng = thread_rng();

        let map_to_function = |number: u8| {
            if number == 0 {
                Self::move_left
            } else if number == 1 {
                Self::move_right
            } else if number == 2 {
                Self::move_up
            } else {
                Self::move_down
            }
        };

        // Start from the goal states, performs sequence of swapping operations from the empty tile
        // We will achieve solvable states of puzzle
        while times > 0 {
            loop {
                let number = rng.gen_range(0..4);
                let func = map_to_function(number);
                if let Some(next) = func(idx, n as usize) {
                    tiles.swap(idx, next);
                    break idx = next;
                }
            }
            times -= 1;
        }

        (tiles, idx as u8)
    }

    fn move_left(idx: usize, n: usize) -> Option<usize> {
        if idx % n == 0 {
            return None;
        }
        Some(idx - 1)
    }

    fn move_right(idx: usize, n: usize) -> Option<usize> {
        if idx % (n - 1) == 0 {
            return None;
        }
        Some(idx + 1)
    }

    fn move_up(idx: usize, n: usize) -> Option<usize> {
        if idx < n {
            return None;
        }
        Some(idx - n)
    }

    fn move_down(idx: usize, n: usize) -> Option<usize> {
        if idx >= n * (n - 1) {
            return None;
        }
        Some(idx + n)
    }
}

// #[cfg(test)]
// mod test {
//     use crate::board::{board::BoardManager, cell::Tile};

//     use super::Board;

//     #[test]
//     fn it_should_create_a_board_of_size_3() {
//         let board = Board::new(3, None);
//         assert_eq!(board.get_tiles().len(), 9);
//         println!("{}", board);
//     }

//     #[test]
//     fn it_should_persist_default_goal_state() {
//         let board = Board::new(3, None);
//         assert_eq!(board.get_goal().len(), 9);
//         let default_goal_state: Vec<Tile> = (0..9).map(|value| Tile::with_value(value)).collect();
//         assert_eq!(&default_goal_state, board.get_goal());
//     }

//     #[test]
//     fn it_should_persist_the_customed_goal_state() {
//         let default_goal_state: Vec<Tile> = (1..=9).map(|value| Tile::with_value(value)).collect();
//         let (custom_goal_state, _) =
//             BoardManager::generate_random_board(3, &default_goal_state, 0.2);
//         let board = Board::new(3, Some(custom_goal_state.clone()));
//         assert_eq!(board.get_goal().len(), 9);
//         assert_eq!(&custom_goal_state, board.get_goal());
//     }

//     #[test]
//     fn it_creates_board_using_from() {
//         let default_goal_state: Vec<Tile> = (1..=9).map(|value| Tile::with_value(value)).collect();
//         let (custom_goal_state, idx) =
//             BoardManager::generate_random_board(3, &default_goal_state, 0.2);
//         let board = Board::new(3, Some(custom_goal_state.clone()));
//         let new_board = Board::from(board.get_tiles().to_vec(), board.n, &board.goal_state, idx);

//         assert_eq!(new_board.get_tiles(), board.get_tiles());

//         assert_eq!(new_board.get_goal(), board.get_goal());

//         assert_eq!(new_board.n, board.n);
//     }
// }
