use std::collections::HashMap;

use js_sys::Array;
use legion::*;
use serde::{Deserialize, Serialize};
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

#[wasm_bindgen]
pub struct Game {
  world: World,
  kek: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Example {
  pub field1: HashMap<u32, String>,
  pub field2: Vec<Vec<f32>>,
  pub field3: [f32; 4],
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
  pub fn example(&self) -> JsValue {
    let mut field1 = HashMap::new();
    field1.insert(0, String::from("ex"));
    let example = Example {
      field1,
      field2: vec![vec![1., 2.], vec![3., 4.]],
      field3: [1., 2., 3., 4.],
    };

    JsValue::from_serde(&example).unwrap()
  }

  #[wasm_bindgen(getter)]
  pub fn stuff(&self) -> Array {
    //let mut query = <&Position>::query();

    let mut query = <(&Velocity, &Position)>::query();

    let baba: Array = query
      .iter(&self.world)
      .map(|p| JsValue::from_serde(&p).unwrap())
      .collect();

    baba
    /*
    let vec_of_positionrefs = query.iter(&self.world).collect::<Vec<&Position>>();

    //vec to js array
    let res: Array = vec_of_positionrefs
      .into_iter()
      .map(|p| JsValue::from_serde(p).unwrap())
      .collect();

    res */
    /*
       let mut a = Array::new();
       for position in query.iter(&self.world) {
         println!("{:?}", position);
         let b = JsValue::from_serde(position).unwrap();
         //a.push(b);
       }

       1.3
    */
    //JsValue::from_serde(&example).unwrap()

    //let b = query.iter(&self.world).collect();
    //let c = clone_vec(b);

    //let b = query.iter(&self.world).collect();

    /*
     let res = query
      .iter(&self.world)
      .into_iter()
      .map(JsValue::from_serde.unwrap())
      .collect();
    res */

    //vec_to_js_array(c)

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

fn slice_to_js_array(slice: &[u32]) -> Array {
  slice.iter().copied().map(JsValue::from).collect()
}

fn vec_to_js_array(vec: Vec<u32>) -> Array {
  vec.into_iter().map(JsValue::from).collect()
}

fn array_to_js_array(array: [u32; 5]) -> Array {
  array.iter().copied().map(JsValue::from).collect()
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
