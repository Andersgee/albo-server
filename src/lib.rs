use std::cell::{RefCell, RefMut};

use wasm_bindgen::prelude::*;
//use js_sys::Uint8Array;

//tutorial writing a tiny ECS in rust
//https://ianjk.com/ecs-in-rust/

struct Health(i32);
struct Name(&'static str);

trait ComponentVec {
  fn as_any(&self) -> &dyn std::any::Any;
  fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
  fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
  // Same as before
  fn as_any(&self) -> &dyn std::any::Any {
    self as &dyn std::any::Any
  }

  // Same as before
  fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
    self as &mut dyn std::any::Any
  }

  fn push_none(&mut self) {
    // `&mut self` already guarantees we have
    // exclusive access to self so can use `get_mut` here
    // which avoids any runtime checks.
    self.get_mut().push(None)
  }
}

#[wasm_bindgen]
pub struct World {
  // We'll use `entities_count` to assign each Entity a unique ID.
  entities_count: usize,
  component_vecs: Vec<Box<dyn ComponentVec>>,
}

#[wasm_bindgen]
impl World {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self {
      entities_count: 0,
      component_vecs: Vec::new(),
    }
  }

  #[wasm_bindgen(getter)]
  pub fn state(&self) -> f64 {
    //placeholder for now
    3.1
  }

  fn new_entity(&mut self) -> usize {
    let entity_id = self.entities_count;
    for component_vec in self.component_vecs.iter_mut() {
      component_vec.push_none();
    }
    self.entities_count += 1;
    entity_id
  }

  fn add_component_to_entity<ComponentType: 'static>(
    &mut self,
    entity: usize,
    component: ComponentType,
  ) {
    for component_vec in self.component_vecs.iter_mut() {
      // The `downcast_mut` type here is changed to `RefCell<Vec<Option<ComponentType>>`
      if let Some(component_vec) = component_vec
        .as_any_mut()
        .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
      {
        // add a `get_mut` here. Once again `get_mut` bypasses
        // `RefCell`'s runtime checks if accessing through a `&mut` reference.
        component_vec.get_mut()[entity] = Some(component);
        return;
      }
    }

    let mut new_component_vec: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);

    for _ in 0..self.entities_count {
      new_component_vec.push(None);
    }

    new_component_vec[entity] = Some(component);

    // Here we create a `RefCell` before inserting into `component_vecs`
    self
      .component_vecs
      .push(Box::new(RefCell::new(new_component_vec)));
  }

  // We've changed the return type to be a `RefMut`.
  // That's what `RefCell` returns when `borow_mut` is used to borrow from the `RefCell`
  // When `RefMut` is dropped the `RefCell` is alerted that it can be borrowed from again.
  fn borrow_component_vec<ComponentType: 'static>(
    &self,
  ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
    for component_vec in self.component_vecs.iter() {
      if let Some(component_vec) = component_vec
        .as_any()
        .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
      {
        // Here we use `borrow_mut`.
        // If this `RefCell` is already borrowed from this will panic.
        return Some(component_vec.borrow_mut());
      }
    }
    None
  }
}

fn hej() {
  let mut world = World::new();

  world.add_component_to_entity(0, Health(100));
  world.add_component_to_entity(0, Name("Somebody"));

  let mut healths = world.borrow_component_vec::<Health>().unwrap();
  let mut names = world.borrow_component_vec::<Name>().unwrap();
  let zip = healths.iter_mut().zip(names.iter_mut());
  let iter = zip.filter_map(|(health, name)| Some((health.as_mut()?, name.as_mut()?)));

  for (health, name) in iter {
    if name.0 == "Perseus" && health.0 <= 0 {
      *health = Health(100);
    }
  }
}

pub fn add(left: usize, right: usize) -> usize {
  hej();
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
