use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  #[wasm_bindgen(js_namespace = Math)]
  fn random() -> f64;
}

enum Vao {
  floor,
  bird,
}

pub struct Renderable {
  vao: Vao,
  model_mat: [f32; 16],
}

pub struct Movable {
  x: f32,
  y: f32,
}

enum Entity {
  Renderable(Option<Renderable>),
  Movable(Option<Movable>),
}

#[wasm_bindgen]
pub struct World {
  entities: Vec<Entity>,
}

#[wasm_bindgen]
impl World {
  #[wasm_bindgen(constructor)]
  pub fn new() -> World {
    log("instantiating new Example");

    //webgl::start().unwrap();

    World { entities: vec![] }
  }

  #[wasm_bindgen(getter)]
  pub fn state(&self) -> f64 {
    3.1
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
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
