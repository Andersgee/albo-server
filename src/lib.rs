use js_sys::Array;
use legion::*;
use wasm_bindgen::prelude::*;
//use js_sys::Uint8Array;
//use std::cell::{RefCell, RefMut};

#[derive(Clone, Copy, Debug, PartialEq)]
#[wasm_bindgen]
pub struct Position {
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
  pub fn stuff(&self) -> Array {
    let mut query = <&Position>::query();

    //let vec_of_positionrefs = query.iter(&self.world).collect();

    for position in query.iter(&self.world) {
      println!("{:?}", position);
    }

    let b: Vec<&Position> = query.iter(&self.world).collect();
    let c = clone_vec(b);

    let res = c.into_iter().map(JsValue::from).collect();
    res
    /*
        let kaka = vec![JsValue::NULL, JsValue::UNDEFINED];
        let res = c.into_iter().map(JsValue::from).collect();
    */
    //let res = c.into_iter().map(JsValue::from).collect();

    //1.2

    //let vec_of_position = clone_vec(vec_of_positionrefs);

    //println!("vec_of_position");

    //vec_of_positionrefs..map(JsValue::from).collect()

    //let res = vec_of_position.into_iter().map(JsValue::from).collect();
    //res

    // you can then iterate through the components found in the world
    /*
    let mut vec_of_position: Vec<Position> = vec![];
    for position in query.iter(&self.world) {
      println!("{:?}", position);
      vec_of_position.push(Position {
        x: position.x.clone(),
        y: position.y.clone(),
      })
    }
    let res = vec_of_position.into_iter().map(JsValue::from).collect();
    res
    */

    /*
    println!("From inside stuff");

    let vec_of_positionrefs = query.iter(&self.world).collect::<Vec<&Position>>();
    println!("vec_of_positionrefs {:?}", vec_of_positionrefs);

    //let vec_of_position = query.iter(&self.world).cloned().collect::<Vec<Position>>();
    let iterthing = query.iter(&self.world);

    let vec_of_position = query
      .iter(&self.world)
      .cloned()
      .cloned()
      .collect::<Vec<Position>>();

    let res = vec_of_position.into_iter().map(JsValue::from).collect();
    res
    */
  }
}

pub fn clone_vec<T: Clone>(vec: Vec<&T>) -> Vec<T> {
  vec.into_iter().cloned().collect()
}

pub fn clone_slice<T: Clone>(slice: &[&T]) -> Vec<T> {
  slice.iter().cloned().cloned().collect()
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
