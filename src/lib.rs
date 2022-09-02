use wasm_bindgen::prelude::*;
//use js_sys::Uint8Array;

//tutorial writing a tiny ECS in rust
//https://ianjk.com/ecs-in-rust/

struct Health(i32);
struct Name(&'static str);

#[wasm_bindgen]
struct World {
  health_components: Vec<Option<Health>>,
  name_components: Vec<Option<Name>>,
}

#[wasm_bindgen]
impl World {
  #[wasm_bindgen(constructor)]
  pub fn new() -> World {
    World {
      health_components: Vec::new(),
      name_components: Vec::new(),
    }
  }

  #[wasm_bindgen(getter)]
  pub fn state(&self) -> f64 {
    //placeholder for now
    3.1
  }

  fn new_entity(&mut self, health: Option<Health>, name: Option<Name>) {
    self.health_components.push(health);
    self.name_components.push(name);
  }
}

fn hej() {
  let mut world = World::new();
  // Icarus's health is *not* looking good.
  world.new_entity(Some(Health(-10)), Some(Name("Icarus")));
  // Prometheus is very healthy.
  world.new_entity(Some(Health(100)), Some(Name("Prometheus")));
  // Note that Zeus does not have a `Health` component.
  world.new_entity(None, Some(Name("Zeus")));

  let zip = world
    .health_components
    .iter()
    .zip(world.name_components.iter());

  let with_health_and_name = zip.filter_map(|(health, name): (&Option<Health>, &Option<Name>)| {
    Some((health.as_ref()?, name.as_ref()?))
  });

  for (health, name) in with_health_and_name {
    if health.0 < 0 {
      println!("{} has perished!", name.0);
    } else {
      println!("{} is still healthy", name.0);
    }
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
