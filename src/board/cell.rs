#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
pub enum Tile {
    Empty,
    Value(u8), // only supports solving n-puzzle where n is less than 7
}

impl Tile {
    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn with_value(value: u8) -> Self {
        match value {
            0 => Self::Empty,
            _ => Self::Value(value),
        }
    }

    pub fn get_value(&self) -> u8 {
        match &self {
            Self::Empty => 0,
            Self::Value(value) => value.to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Tile;

    #[test]
    fn it_should_create_an_empty_tile() {
        let empty_tile = Tile::empty();
        assert_eq!(empty_tile.get_value(), 0);
        assert_eq!(empty_tile, Tile::Empty);
    }

    #[test]
    fn it_should_create_a_tile_with_value() {
        let value_tile = Tile::with_value(12);
        assert_eq!(value_tile, Tile::Value(12));
    }

    #[test]
    fn it_should_create_an_empty_tile_although_using_with_value() {
        let empty_tile = Tile::with_value(0);
        assert_eq!(empty_tile.get_value(), 0);
        assert_eq!(empty_tile, Tile::Empty);
    }
}
