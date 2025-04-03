use crate::graphs::{Graph, GraphOptions};
use rand::{rng, Rng};

impl Graph {
    pub fn generate(options: &GraphOptions) -> Self {
        let left_size: usize = options.left_size.try_into().unwrap();
        let right_size: usize = options.right_size.try_into().unwrap();

        let mut left_to_right: Vec<Vec<u8>> = vec![vec![0; right_size]; left_size];
        let mut right_to_left: Vec<Vec<u8>> = vec![vec![0; left_size]; right_size];

        populate_edges(&mut left_to_right, options.edge_chance);
        populate_edges(&mut right_to_left, options.edge_chance);

        cull_loops(
            &mut left_to_right,
            &mut right_to_left
        );

        Self {
            left_to_right,
            right_to_left
        }
    }
}

fn populate_edges(edge_matrix: &mut [Vec<u8>], chance: u8) {
    for row in edge_matrix.iter_mut() {
        for cell in row.iter_mut() {
            *cell = flip_weight_coin(chance);
        }
    }
}

fn cull_loops(edge_matrix_a: &mut [Vec<u8>], edge_matrix_b: &mut [Vec<u8>]) {
    for (i, row) in edge_matrix_a.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if *cell == 1 && edge_matrix_b[j][i] == 1 {
                let flip = flip_weight_coin(50);
                if flip == 1 {
                    *cell = 0;
                } else {
                    edge_matrix_b[j][i] = 0;
                }
            }
        }
    }
}

pub fn flip_weight_coin(chance: u8) -> u8 {
    let flip = rng().random_range(0..100);
    if flip < chance {
        1
    } else {
        0
    }
}