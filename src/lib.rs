mod components;
mod resources;
mod systems;

use js_sys::Array;
use legion::*;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;

#[wasm_bindgen]
pub struct Game {
  world: World,
  resources: Resources,
  schedule: Schedule,
  socketmap: HashMap<u32, Entity>,
}

#[wasm_bindgen]
impl Game {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    let mut world = World::default();

    let mut resources = Resources::default();

    resources.insert(resources::Time {
      elapsed_seconds: 2.0,
    });

    let schedule = Schedule::builder()
      .add_system(systems::transform::transform_system())
      .build();

    world.push((components::Renderable {
      vao: components::Vao::Floor,
      model_mat: [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
      ],
    },));

    Self {
      world,
      resources,
      schedule,
      socketmap: HashMap::new(),
    }
  }

  //run all systems (as defined by Schedule)
  pub fn tick(&mut self) {
    self.schedule.execute(&mut self.world, &mut self.resources)
  }

  #[wasm_bindgen(getter)]
  pub fn renderable(&self) -> Array {
    let mut query = <&components::Renderable>::query();
    let js_array: Array = query
      .iter(&self.world)
      .map(|p| JsValue::from_serde(&p).unwrap())
      .collect();

    js_array
  }

  #[wasm_bindgen(getter)]
  pub fn players(&self) -> Array {
    let mut query = <(&components::Player,)>::query();
    let js_array: Array = query
      .iter(&self.world)
      .map(|p| JsValue::from_serde(&p).unwrap())
      .collect();

    js_array
  }

  pub fn add_player(&mut self, socket_id: u32) {
    let entity = self.world.push((components::Player {
      socket_id,
      input: [0, 0, 0, 0],
    },));

    self.socketmap.insert(socket_id, entity);
  }

  pub fn remove_player(&mut self, socket_id: u32) {
    let entity = self.socketmap.get(&socket_id).unwrap().to_owned();
    self.world.remove(entity);
    self.socketmap.remove(&socket_id);
  }

  pub fn set_player_input(
    &mut self,
    socket_id: u32,
    step_forward: u8,
    step_backward: u8,
    step_left: u8,
    step_right: u8,
  ) {
    let entity = self.socketmap.get(&socket_id).unwrap().to_owned();

    if let Some(mut entry) = self.world.entry(entity) {
      // access information about the entity's archetype
      println!(
        "{:?} has {:?}",
        entity,
        entry.archetype().layout().component_types()
      );

      //update player component
      entry.add_component(components::Player {
        socket_id,
        input: [step_forward, step_backward, step_left, step_right],
      });
    }
  }
}
