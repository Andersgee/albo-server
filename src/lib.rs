mod components;
mod resources;
mod systems;

use js_sys::Array;
use legion::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
  world: World,
  resources: Resources,
  schedule: Schedule,
}

#[wasm_bindgen]
impl Game {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    let mut world = World::default();

    let mut resources = Resources::default();
    //resources.insert(vec!["Jane Doe", "John Smith"]);

    resources.insert(resources::Time {
      elapsed_seconds: 2.0,
    });

    let schedule = Schedule::builder()
      .add_system(systems::position::run_system())
      .build();

    let _entity: Entity = world.push((
      components::Position { x: 0.0, y: 0.0 },
      components::Velocity { dx: 0.0, dy: 0.0 },
    ));
    let _entity2: Entity = world.push((
      components::Position { x: 1.2, y: 3.4 },
      components::Velocity { dx: 88.0, dy: 9.2 },
    ));
    let _entity3: Entity = world.push((components::Position { x: 0.2, y: 5.6 },));

    Self {
      world,
      resources,
      schedule,
    }
  }

  //run all systems (as defined by Schedule)
  pub fn tick(&mut self) {
    self.schedule.execute(&mut self.world, &mut self.resources)
  }

  #[wasm_bindgen(getter)]
  pub fn state(&self) -> f64 {
    //placeholder for now
    3.1
  }

  #[wasm_bindgen(getter)]
  pub fn stuff(&self) -> Array {
    let mut query = <(Option<&components::Velocity>, &components::Position)>::query();
    let js_array: Array = query
      .iter(&self.world)
      .map(|p| JsValue::from_serde(&p).unwrap())
      .collect();

    js_array
  }

  pub fn add_player(&mut self, socket_id: u32) {
    self.world.push((components::Player {
      socket_id,
      input: [0, 0, 0, 0],
    },));
  }
}
