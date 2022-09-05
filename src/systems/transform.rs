use crate::{components, resources};
use gl_matrix::vec3;
use legion::*;

#[system(for_each)]
pub fn transform(transform: &mut components::Transform, #[resource] time: &resources::Time) {
  /*
   vec3::add(
     &mut transform.position.to_owned(),
     &transform.position,
     &transform.velocity,
   );
  */

  transform.position[0] = transform.position[0] + transform.velocity[0];
  transform.position[1] = transform.position[1] + transform.velocity[1];
  transform.position[2] = transform.position[2] + transform.velocity[2];
}
