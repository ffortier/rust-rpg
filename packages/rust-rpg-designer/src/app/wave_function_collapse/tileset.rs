use std::vec;

#[derive(Clone, Copy, Debug)]
pub enum Tileset {
  Nature,
  Autumn,
  Ice,
}

#[derive(Debug)]
pub struct TileSpec {
  pub col_idx: usize,
  pub row_idx: usize,
  pub tileset: Tileset,
  pub edges: [(Texture, Texture); 4],
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Texture {
  Grass = 1u8,
  Dirt = 2u8,
  Water = 3u8,
  Rock = 4u8,
}

impl Tileset {
  pub fn define_background_tiles(&self) -> Vec<TileSpec> {
    let mut tiles = vec![];

    tiles.push(self.define_plain_tile(0, 2, Texture::Grass));
    tiles.push(self.define_plain_tile(1, 2, Texture::Water));
    tiles.push(self.define_plain_tile(2, 2, Texture::Dirt));

    tiles.extend(self.define_textured_tiles(0, 3, Texture::Dirt, Texture::Grass));
    tiles.extend(self.define_textured_tiles(5, 3, Texture::Water, Texture::Grass));
    tiles.extend(self.define_textured_tiles(10, 3, Texture::Water, Texture::Dirt));
    tiles.extend(self.define_textured_tiles(15, 3, Texture::Rock, Texture::Grass));
    tiles.extend(self.define_textured_tiles(15, 0, Texture::Rock, Texture::Water));
    tiles.extend(self.define_textured_tiles(15, 6, Texture::Rock, Texture::Dirt));

    tiles
  }

  fn define_plain_tile(&self, col_idx: usize, row_idx: usize, texture: Texture) -> TileSpec {
    TileSpec {
      col_idx,
      row_idx,
      tileset: *self,
      edges: self.make_edge(texture, texture, texture, texture),
    }
  }

  fn define_textured_tiles(
    &self,
    col_idx: usize,
    row_idx: usize,
    t1: Texture,
    t2: Texture,
  ) -> Vec<TileSpec> {
    vec![
      TileSpec {
        col_idx: col_idx + 0,
        row_idx: row_idx + 0,
        tileset: *self,
        edges: self.make_edge(t1, t1, t2, t1),
      },
      TileSpec {
        col_idx: col_idx + 1,
        row_idx: row_idx + 0,
        tileset: *self,
        edges: self.make_edge(t1, t1, t2, t2),
      },
      TileSpec {
        col_idx: col_idx + 2,
        row_idx: row_idx + 0,
        tileset: *self,
        edges: self.make_edge(t1, t1, t1, t2),
      },
      TileSpec {
        col_idx: col_idx + 0,
        row_idx: row_idx + 1,
        tileset: *self,
        edges: self.make_edge(t1, t2, t2, t1),
      },
      TileSpec {
        col_idx: col_idx + 2,
        row_idx: row_idx + 1,
        tileset: *self,
        edges: self.make_edge(t2, t1, t1, t2),
      },
      TileSpec {
        col_idx: col_idx + 0,
        row_idx: row_idx + 2,
        tileset: *self,
        edges: self.make_edge(t1, t2, t1, t1),
      },
      TileSpec {
        col_idx: col_idx + 1,
        row_idx: row_idx + 2,
        tileset: *self,
        edges: self.make_edge(t1, t1, t2, t2),
      },
      TileSpec {
        col_idx: col_idx + 2,
        row_idx: row_idx + 2,
        tileset: *self,
        edges: self.make_edge(t2, t1, t1, t1),
      },
      TileSpec {
        col_idx: col_idx + 3,
        row_idx: row_idx + 0,
        tileset: *self,
        edges: self.make_edge(t2, t2, t1, t2),
      },
      TileSpec {
        col_idx: col_idx + 4,
        row_idx: row_idx + 0,
        tileset: *self,
        edges: self.make_edge(t2, t2, t2, t1),
      },
      TileSpec {
        col_idx: col_idx + 3,
        row_idx: row_idx + 1,
        tileset: *self,
        edges: self.make_edge(t2, t1, t2, t2),
      },
      TileSpec {
        col_idx: col_idx + 4,
        row_idx: row_idx + 1,
        tileset: *self,
        edges: self.make_edge(t1, t2, t2, t2),
      },
      TileSpec {
        col_idx: col_idx + 3,
        row_idx: row_idx + 2,
        tileset: *self,
        edges: self.make_edge(t2, t1, t2, t1),
      },
      TileSpec {
        col_idx: col_idx + 4,
        row_idx: row_idx + 2,
        tileset: *self,
        edges: self.make_edge(t1, t2, t1, t2),
      },
    ]
  }

  fn make_edge(
    &self,
    tl: Texture,
    tr: Texture,
    br: Texture,
    bl: Texture,
  ) -> [(Texture, Texture); 4] {
    [(tl, tr), (tr, br), (bl, br), (tl, bl)]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let tiles = Tileset::Nature.define_background_tiles();

    println!("{tiles:?}")
  }
}
