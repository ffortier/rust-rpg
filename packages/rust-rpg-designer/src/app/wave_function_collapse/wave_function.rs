use super::{Tile, TileSpec, Tileset};

struct Data<'a> {
  tileSpecs: Vec<TileSpec>,
  tiles: Vec<Tile<'a>>,
}

impl<'a> Data<'a> {
  pub fn new(cols: usize, rows: usize, tileset: &Tileset) -> Self {
    let tileSpecs = tileset.define_background_tiles();

    Self {
      tileSpecs,
      tiles: (0..rows * cols).into_iter().map(|_| Tile::new()).collect(),
    }
  }
}
