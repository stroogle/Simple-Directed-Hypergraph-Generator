use crate::graphs::{Graph, GraphOptions};

impl Graph {
    pub fn generate(options: &GraphOptions) -> Self {
        let left_size: usize = options.left_size.try_into().unwrap();
        let right_size: usize = options.right_size.try_into().unwrap();

        let mut left_to_right: Vec<Vec<u8>> = vec![vec![0; right_size]; left_size];
        let mut right_to_left: Vec<Vec<u8>> = vec![vec![0; left_size]; right_size];

        populate_edges(&mut left_to_right);
        populate_edges(&mut right_to_left);

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

fn populate_edges(edge_matrix: &mut Vec<Vec<u8>>) {

}

fn cull_loops(edge_matrix_a: &mut Vec<Vec<u8>>, edge_matrix_b: &mut Vec<Vec<u8>>) {

}