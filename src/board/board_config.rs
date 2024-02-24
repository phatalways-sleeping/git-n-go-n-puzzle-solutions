use super::cell::Tile;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct UninformedConfig {
    pub n: u8,
    pub empty_tile_idx: u8,
    pub goal_state: Vec<Tile>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct InformedConfig {
    pub n: u8,
    pub empty_tile_idx: u8,
    pub goal_state: Vec<Tile>,
    pub weight: usize,
    pub depth: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub enum BoardConfig {
    Uninformed(UninformedConfig),
    Informed(InformedConfig),
}

impl BoardConfig {
    pub fn n(&self) -> u8 {
        match self {
            Self::Uninformed(config) => config.n,
            Self::Informed(config) => config.n,
        }
    }

    pub fn goal_state(&self) -> &[Tile] {
        match self {
            Self::Uninformed(config) => &config.goal_state,
            Self::Informed(config) => &config.goal_state,
        }
    }

    pub fn empty_tile_idx(&self) -> u8 {
        match self {
            Self::Uninformed(config) => config.empty_tile_idx,
            Self::Informed(config) => config.empty_tile_idx,
        }
    }

    pub fn set_empty_tile_idx(&mut self, empty_tile_idx: u8) {
        match self {
            Self::Uninformed(config) => config.empty_tile_idx = empty_tile_idx,
            Self::Informed(config) => config.empty_tile_idx = empty_tile_idx,
        }
    }

    pub fn get_mut_depth(&mut self) -> Option<&mut usize> {
        match self {
            Self::Informed(config) => Some(&mut config.depth),
            Self::Uninformed(_) => None,
        }
    }

    pub fn get_mut_weight(&mut self) -> Option<&mut usize> {
        match self {
            Self::Informed(config) => Some(&mut config.weight),
            Self::Uninformed(_) => None,
        }
    }

    pub fn get_depth(&self) -> Option<&usize> {
        match self {
            Self::Informed(config) => Some(&config.depth),
            Self::Uninformed(_) => None,
        }
    }

    pub fn get_weight(&self) -> Option<&usize> {
        match self {
            Self::Informed(config) => Some(&config.weight),
            Self::Uninformed(_) => None,
        }
    }
}

pub struct BoardConfigBuilder {
    n: Option<u8>,
    goal_state: Option<Vec<Tile>>,
    weight: Option<usize>,
    depth: Option<usize>,
    informed: Option<bool>,
}

impl BoardConfigBuilder {
    pub fn builder() -> Self {
        Self {
            n: None,
            goal_state: None,
            weight: None,
            depth: None,
            informed: None,
        }
    }

    pub fn essential(mut self, n: u8, goal_state: Vec<u8>) -> Self {
        self.goal_state = Some(
            goal_state
                .into_iter()
                .map(|value| Tile::with_value(value))
                .collect(),
        );
        self.n = Some(n);

        self
    }

    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = Some(depth);
        self
    }

    pub fn with_weight(mut self, weight: usize) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn with_informed(mut self) -> Self {
        self.informed = Some(true);
        self
    }

    pub fn build(mut self) -> Result<BoardConfig, &'static str> {
        if self.n.is_none() || self.goal_state.is_none() {
            return Err("BoardConfigBuilder: must call self.essential()");
        }

        let n = self.n.take().unwrap();
        let goal_state = self.goal_state.take().unwrap();

        if self.informed.is_none() {
            Ok(BoardConfig::Uninformed(UninformedConfig {
                n,
                empty_tile_idx: 0,
                goal_state,
            }))
        } else {
            Ok(BoardConfig::Informed(InformedConfig {
                n,
                empty_tile_idx: 0,
                goal_state,
                weight: self.weight.take().unwrap_or(0),
                depth: self.depth.take().unwrap_or(0),
            }))
        }
    }
}
