use legion::*;
use wasm_bindgen::prelude::*;
//use js_sys::Uint8Array;
//use std::cell::{RefCell, RefMut};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
  x: f32,
  y: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity {
  dx: f32,
  dy: f32,
}

#[wasm_bindgen]
pub struct Game {
  world: World,
  kek: f32,
}

#[wasm_bindgen]
impl Game {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    let mut world = World::default();
    let _entity: Entity = world.push((Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }));
    let _entity2: Entity =
      world.push((Position { x: 1.2, y: 3.4 }, Velocity { dx: 88.0, dy: 9.2 }));

    Self { world, kek: 1.2 }
  }

  #[wasm_bindgen(getter)]
  pub fn state(&self) -> f64 {
    //placeholder for now
    3.1
  }

  #[wasm_bindgen(getter)]
  pub fn stuff(&self) -> f32 {
    let mut query = <&Position>::query();

    // you can then iterate through the components found in the world
    for position in query.iter(&self.world) {
      println!("{:?}", position);
    }

    println!("From inside stuff");

    self.kek
  }
}

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    println!("kjfekjfkjsdfkdjsz {:?}", 654654);
    let w = Game::new();
    w.stuff();
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
