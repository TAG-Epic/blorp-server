use rand::Rng;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub position: (u8, u8),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum TileType {
    EMPTY,
    RESOURCEFUL(u32),
}

pub fn create_board() -> Vec<Vec<Tile>> {
    let mut board: Vec<Vec<Tile>> = Vec::new();
    for row_id in 0..30 {
        let mut row: Vec<Tile> = Vec::new();
        for tile_id in 0..30 {
            let is_resourceful = rand::thread_rng().gen_bool(1.0 / 25.0);
            row.push(match is_resourceful {
                true => {
                    let resources = rand::thread_rng().gen_range(1..=5);

                    Tile {
                        tile_type: TileType::RESOURCEFUL(resources),
                        position: (tile_id, row_id)
                    }
                }
                false => Tile {
                    tile_type: TileType::EMPTY,
                    position: (tile_id, row_id)
                },
            });
        }
        board.push(row);
    }
    board
}
