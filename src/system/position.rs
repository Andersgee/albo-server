use crate::{component, resource};
use legion::*;

#[system(for_each)]
pub fn run(
  pos: &mut component::Position,
  vel: &component::Velocity,
  #[resource] time: &resource::Time,
) {
  pos.x += vel.dx * time.elapsed_seconds;
  pos.y += vel.dy * time.elapsed_seconds;
}
