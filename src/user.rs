use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub action_points: u32,
    pub position: (u8, u8),
    pub creation_date: u32,
    pub awarded_points: u32,
    pub range: u8,
    pub health: u8,
}
