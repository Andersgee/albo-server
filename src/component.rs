use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
  pub x: f32,
  pub y: f32,
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Velocity {
  pub dx: f32,
  pub dy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
  pub socket_id: u32,
  pub input: [u8; 4],
}
