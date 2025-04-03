mod generate;
mod save;

pub struct Graph {
    left_to_right: Vec<Vec<u8>>,
    right_to_left: Vec<Vec<u8>>
}

pub struct GraphOptions {
    pub left_size: u32,
    pub right_size: u32,
    pub edge_chance: u8
}