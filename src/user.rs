use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub action_points: u32,
    pub position: (u8, u8)
}
