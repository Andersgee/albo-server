use crate::{components, resources};
use gl_matrix::vec3;
use legion::*;

const D: f32 = 0.7071067811865476; //distance to make hypotenuse 1 aka sqrt(2)/2

#[system(for_each)]
pub fn input(
  transform: &mut components::Transform,
  player: &components::Player,
  #[resource] time: &resources::Time,
) {
  //keep it basic for now. player controls velocity in
  //(forward/backward, right/left) directions
  //aka
  //(-z/+z, +x/-x) world axes directions

  //let ms: f32 = 0.1; //move speed
  let move_dist_per_tick: f32 = 1.0; //move speed

  let step_forward = player.input[0] != 0;
  let step_backward = player.input[1] != 0;
  let step_left = player.input[2] != 0;
  let step_right = player.input[3] != 0;

  let mut velocity = vec3::create();

  if step_forward && step_right {
    velocity[2] = -D * move_dist_per_tick;
    velocity[0] = D * move_dist_per_tick;
  } else if step_forward && step_left {
    velocity[2] = -D * move_dist_per_tick;
    velocity[0] = -D * move_dist_per_tick;
  } else if step_backward && step_right {
    velocity[2] = D * move_dist_per_tick;
    velocity[0] = D * move_dist_per_tick;
  } else if step_backward && step_left {
    velocity[2] = D * move_dist_per_tick;
    velocity[0] = -D * move_dist_per_tick;
  } else if step_forward {
    velocity[2] = -move_dist_per_tick;
  } else if step_backward {
    velocity[2] = move_dist_per_tick;
  } else if step_left {
    velocity[0] = -move_dist_per_tick;
  } else if step_right {
    velocity[0] = move_dist_per_tick;
  }

  if step_forward && step_backward {
    velocity[2] = 0.;
  }

  if step_left && step_right {
    velocity[0] = 0.;
  }

  transform.velocity = velocity;
}
