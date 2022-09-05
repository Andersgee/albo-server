use std::borrow::Borrow;

use crate::{components, resources};
use gl_matrix::{common::Vec3, vec3};
use legion::*;

pub fn add_inplace(out: &mut Vec3, v: &Vec3) -> Vec3 {
  out[0] = out[0] + v[0];
  out[1] = out[1] + v[1];
  out[2] = out[2] + v[2];

  *out
}

#[system(for_each)]
pub fn transform(transform: &mut components::Transform, #[resource] time: &resources::Time) {
  add_inplace(&mut transform.position, &transform.velocity);
}
