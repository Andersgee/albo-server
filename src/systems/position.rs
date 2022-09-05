use crate::{components, resources};
use legion::*;

#[system(for_each)]
pub fn run(
  pos: &mut components::Position,
  vel: &components::Velocity,
  #[resource] time: &resources::Time,
) {
  pos.x += vel.dx * time.elapsed_seconds;
  pos.y += vel.dy * time.elapsed_seconds;
}
