use super::TileSpec;

pub struct Tile<'a> {
  spec: Option<&'a TileSpec>,
  neighbours: [Option<&'a Tile<'a>>; 4],
}

impl<'a> Tile<'a> {
  pub fn new() -> Self {
    Self {
      spec: None,
      neighbours: [None, None, None, None],
    }
  }

  pub fn set_up(&mut self, tile: &'a Tile<'a>) {
    self.neighbours[0] = Some(tile);
  }

  pub fn set_right(&mut self, tile: &'a Tile<'a>) {
    self.neighbours[1] = Some(tile);
  }

  pub fn set_down(&mut self, tile: &'a Tile<'a>) {
    self.neighbours[2] = Some(tile);
  }

  pub fn set_left(&mut self, tile: &'a Tile<'a>) {
    self.neighbours[3] = Some(tile);
  }

  pub fn is_collapsed(&self) -> bool {
    match self.spec {
      Some(_) => true,
      _ => false,
    }
  }

  pub fn evaluate(&self, options: &Vec<TileSpec>) -> usize {
    let mut all_options = vec![];

    for x in options {
      all_options.push(x);
    }

    all_options.retain(|option| {
      for i in 0..4 {
        let option_edge = option.edges[i];

        if let Some(neighbour) = self.neighbours[i] {
          if let Some(spec) = neighbour.spec {
            let neighbour_edge = spec.edges[(i + 2) % 4];

            if option_edge == neighbour_edge {
              return true;
            }
          }
        }
      }

      false
    });

    all_options.len()
  }

  pub fn collapse(&mut self, options: &[&'a TileSpec]) {}
}
