use crate::{components, resources};
use gl_matrix::vec3;
use legion::*;

const D: f32 = 0.7071067811865476; //distance to make hypotenuse 1 aka sqrt(2)/2
const MOVE_SPEED: f32 = 0.5; //move distance per tick

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

  let step_forward = player.input[0] != 0;
  let step_backward = player.input[1] != 0;
  let step_left = player.input[2] != 0;
  let step_right = player.input[3] != 0;

  let mut velocity = vec3::create();

  if step_forward && step_right {
    velocity[2] = -D * MOVE_SPEED;
    velocity[0] = D * MOVE_SPEED;
  } else if step_forward && step_left {
    velocity[2] = -D * MOVE_SPEED;
    velocity[0] = -D * MOVE_SPEED;
  } else if step_backward && step_right {
    velocity[2] = D * MOVE_SPEED;
    velocity[0] = D * MOVE_SPEED;
  } else if step_backward && step_left {
    velocity[2] = D * MOVE_SPEED;
    velocity[0] = -D * MOVE_SPEED;
  } else if step_forward {
    velocity[2] = -MOVE_SPEED;
  } else if step_backward {
    velocity[2] = MOVE_SPEED;
  } else if step_left {
    velocity[0] = -MOVE_SPEED;
  } else if step_right {
    velocity[0] = MOVE_SPEED;
  }

  if step_forward && step_backward {
    velocity[2] = 0.;
  }

  if step_left && step_right {
    velocity[0] = 0.;
  }

  transform.velocity = velocity;
}
