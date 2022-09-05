use gl_matrix::common::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
  pub socket_id: u32,
  pub input: [u8; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform {
  pub position: Vec3,
  pub velocity: Vec3,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Vao {
  Floor,
  Bird,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Renderable {
  pub vao: Vao,
  pub model_mat: [f32; 16],
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Controlled {
  owner_socket_id: u32,
}
