use legion::*;
use std::cell::{RefCell, RefMut};
use wasm_bindgen::prelude::*;
//use js_sys::Uint8Array;

// a component is any type that is 'static, sized, send and sync
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

pub fn add(left: usize, right: usize) -> usize {
  let mut world = World::default();
  // push a component tuple into the world to create an entity
  let entity: Entity = world.push((Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }));

  // or extend via an IntoIterator of tuples to add many at once (this is faster)
  let entities: &[Entity] = world.extend(vec![
    (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }),
    (Position { x: 1.0, y: 1.0 }, Velocity { dx: 0.0, dy: 0.0 }),
    (Position { x: 2.0, y: 2.0 }, Velocity { dx: 0.0, dy: 0.0 }),
  ]);

  let c = world.components();

  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
