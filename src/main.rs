use std::todo;
use rayon::prelude::*;

fn main() {

    let person_count: u32 = 4;

    let books: Vec<(u32, u32)> = vec![(0, 1), (0, 1), (0, 1), (0, 1)];

    let wishes: Vec<Vec<u8>> = vec![
            vec![0, 2, 1, 3],
            vec![3, 2, 0, 1],
            vec![3, 1, 0, 2],
            vec![3, 2, 1, 0]
        ];

    let mut current_pool: Vec<(Vec<u8>, f32)> = generate_random_pool(person_count, books, 100);

    for i in 0..10 {
        current_pool.par_iter_mut().for_each(
            |(assignment, cost)| {
                assignment = mutate_assignment(assignment);
                cost = activation_function(assignment, wishes);
            }
        );
    }
}

fn generate_random_pool(person_count: u32, books: Vec<(u32, u32)>, pool_size: u32) -> Vec<Vec<u8>> {
    todo!();
}

fn mutate_assignment(assignment: Vec<u8>) {

}

fn activation_function(assignment: Vec<u8>, wishes: Vec<Vec<u8>>) -> f32 {
    todo!();
}

