use js_sys::Array;
use legion::*;
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
use wasm_bindgen::prelude::*;

//use js_sys::Uint8Array;
//use std::cell::{RefCell, RefMut};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
  x: f32,
  y: f32,
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct Velocity {
  dx: f32,
  dy: f32,
}

struct Time {
  elapsed_seconds: f32,
}

#[wasm_bindgen]
pub struct Game {
  world: World,
  resources: Resources,
  kek: f32,
}

#[wasm_bindgen]
impl Game {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    let mut world = World::default();

    let mut resources = Resources::default();
    //resources.insert(vec!["Jane Doe", "John Smith"]);
    let time = Time {
      elapsed_seconds: 0.0,
    };
    resources.insert(time);

    let _entity: Entity = world.push((Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }));
    let _entity2: Entity =
      world.push((Position { x: 1.2, y: 3.4 }, Velocity { dx: 88.0, dy: 9.2 }));

    let _entity3: Entity = world.push((Position { x: 0.2, y: 5.6 },));

    Self {
      world,
      kek: 1.2,
      resources,
    }
  }

  #[wasm_bindgen(getter)]
  pub fn state(&self) -> f64 {
    //placeholder for now
    3.1
  }

  #[wasm_bindgen(getter)]
  pub fn stuff(&self) -> Array {
    let mut query = <(Option<&Velocity>, &Position)>::query();
    let js_array: Array = query
      .iter(&self.world)
      .map(|p| JsValue::from_serde(&p).unwrap())
      .collect();

    js_array
  }

  pub fn run_systems(&mut self) {
    update_positions(&mut self.world, &mut self.resources);
  }

  pub fn set_player_input(&mut self, player_id: u32, player_input: f32) {
    update_positions(&mut self.world, &mut self.resources);
  }
}

fn update_positions(world: &mut World, resources: &mut Resources) {
  let mut query = <(&Velocity, &mut Position)>::query();

  //let time = resources.get(Time);

  for (velocity, position) in query.iter_mut(world) {
    position.x += velocity.dx;
    position.y += velocity.dy;
  }
}

#[system(for_each)]
fn update_positions_hmm(pos: &mut Position, vel: &Velocity, #[resource] time: &Time) {
  pos.x += vel.dx * time.elapsed_seconds;
  pos.y += vel.dy * time.elapsed_seconds;
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
