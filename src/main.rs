use std::todo;
use rayon::prelude::*;
use rand::prelude::*;

#[derive(Debug, Clone)]
struct BookAssignment {
    assignments: Vec<u8>,
    cost: usize,
}

impl BookAssignment {
    fn new(assignment: Vec<u8>) -> BookAssignment {
        BookAssignment { assignments: assignment, cost: 0 }
    }
}

fn main() {

    let person_count: usize = 4;

    let books: Vec<(u32, u32)> = vec![(0, 1), (0, 1), (0, 1), (0, 1)];

    let wishes: Vec<Vec<u8>> = vec![
            vec![0, 2, 1, 3],
            vec![3, 2, 0, 1],
            vec![3, 1, 0, 2],
            vec![3, 2, 1, 0]
        ];

    let mut current_pool: Vec<BookAssignment> = generate_random_pool(person_count, books, 100);

    current_pool.par_iter_mut().for_each(
        |mut book_assignment| {
            activation_function(&mut book_assignment, &wishes);
        }
    );
 
    for _ in 0..1000 {
        /*
        current_pool.par_iter_mut().for_each(
            |mut book_assignment| {
                mutate_assignments(&mut book_assignment.assignments, 3.0);
                // TODO: Minor optimizations: check if easy improvement exists
                activation_function(&mut book_assignment, &wishes);
            }
        );
        */

        current_pool.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());

        let l = current_pool.len();
        for i in 0..l / 2 {
            let mut book_assignment: BookAssignment = current_pool[i].clone();
            mutate_assignments(&mut book_assignment.assignments, 3.0); // TODO: Hard coded value
            activation_function(&mut book_assignment, &wishes);
            current_pool[i + l / 2] = book_assignment;
        }

        println!("{:?}", current_pool);
    }
}

fn generate_random_pool(person_count: usize, books: Vec<(u32, u32)>, pool_size: u32) -> Vec<BookAssignment> {
    let mut pool: Vec<BookAssignment> = vec![BookAssignment::new(vec![0, 1, 2, 3]), BookAssignment::new(vec![3, 0, 1, 2])];
    // let mut nums: Vec<i32> = (1..100).collect();
    // nums.shuffle(&mut rng);

    pool
}

fn mutate_assignments(assignment: &mut Vec<u8>, amount: f64) {
    let mut rng = rand::thread_rng();
    let random: f64 = rng.gen();

    let iterations: usize = (random * amount) as usize + 1;

    for _ in 0..iterations {
        let i: usize = (rng.gen::<f64>() * (assignment.len() as f64)) as usize;
        let mut j = i;
        while i == j {
            j = (rng.gen::<f64>() * (assignment.len() as f64)) as usize;
        }

        let i_value = assignment[i];
        assignment[i] = assignment[j];
        assignment[j] = i_value;
    }
}

fn activation_function(book_assignment: &mut BookAssignment, wishes: &Vec<Vec<u8>>) {
    let mut book_distance: Vec<usize> = Vec::with_capacity(book_assignment.assignments.len());

    let mut distance_sum: usize = 0;
    for (assignment, wish) in book_assignment.assignments.iter().zip(wishes) {
        let distance: usize = wish.into_iter().position(|v| *v == *assignment).unwrap_or(5);
        distance_sum += distance * distance;
    }

    book_assignment.cost = distance_sum;
}

