use rand::Rng;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Tile {
    tile_type: TileType
}

#[derive(Clone, Serialize, Deserialize)]
pub enum TileType {
    EMPTY,
    RESOURCEFUL(u32)
}

pub fn create_board() -> Vec<Vec<Tile>> {
    let mut board: Vec<Vec<Tile>> = Vec::new();
    for _row_id in 0..100 {
        let mut row: Vec<Tile> = Vec::new();
        for _tile_id in 0..100 {
            let is_resourceful = rand::thread_rng().gen_bool(1.0 / 25.0);
            row.push(match is_resourceful {
                true => {
                    let resources = rand::thread_rng().gen_range(1..=5);

                    Tile {
                        tile_type: TileType::RESOURCEFUL(resources)
                    }
                },
                false => Tile { tile_type: TileType::EMPTY}
            });
        };
        board.push(row);
    };
    board
}
