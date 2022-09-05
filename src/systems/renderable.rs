use crate::{components, resources};
use gl_matrix::mat4;
use legion::*;
//use gl_matrix::common::*;

#[system(for_each)]
pub fn renderable(
  renderable: &mut components::Renderable,
  transform: &components::Transform,
  #[resource] time: &resources::Time,
) {
  mat4::from_translation(&mut renderable.model_mat, &transform.position);
}
