use std::collections::HashSet;

use crate::graphs::{Graph, GraphOptions};
use rand::{rng, Rng};

impl Graph {
    pub fn generate(options: &GraphOptions) -> Self {

        if options.balanced_arcs {
            Graph::generate_balanced(options)
        } else {
            Graph::generate_normal(options)
        }

    }

    fn generate_normal(options: &GraphOptions) -> Self {
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

    fn generate_balanced(options: &GraphOptions) -> Self {
        let left_size: usize = options.left_size.try_into().unwrap();
        let right_size: usize = options.right_size.try_into().unwrap();

        let mut left_to_right: Vec<Vec<u8>> = vec![vec![0; right_size]; left_size];
        let mut right_to_left: Vec<Vec<u8>> = vec![vec![0; left_size]; right_size];

        let balanced_size: usize = 6;

        for hyperarc_index in 0..right_size {
            let (head, tail) = generate_non_coliding_sets(
                balanced_size,
                left_size
            );

            for el in head {
                left_to_right[el][hyperarc_index] = 1;
            }

            for el in tail {
                right_to_left[hyperarc_index][el] = 1;
            }
        }
        
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

fn generate_non_coliding_sets(capacity: usize, bound: usize) -> (HashSet<usize>, HashSet<usize>) {
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();

    while set1.len() < capacity {
        let random_value = rng().random_range(0..bound);
        set1.insert(random_value);
    }

    while set2.len() < capacity {
        let random_value = rng().random_range(0..bound);
        set2.insert(random_value);
    }

    let mut intersection: HashSet<&usize> = set1.intersection(&set2).collect();

    while !intersection.is_empty() {
        set2 = HashSet::new();
        while set2.len() < capacity {
            let random_value = rng().random_range(0..bound);
            set2.insert(random_value);
        }
        intersection = set1.intersection(&set2).collect();
    }

    (set1, set2)
}