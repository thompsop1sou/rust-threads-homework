use std::thread;
use rand::Rng;

fn main() {
    const MEBI: usize = 1024*1024;
    let vector = create_rand_vec(8*MEBI, -128, 128);
    println!("Sum of vector (loop): {}", sum_vec_loop(&vector));
    println!("Sum of vector (threads): {}", sum_vec_threads(&vector, 5));
}

// Creates a vector of indicated length filled with random numbers between min and max
fn create_rand_vec(length: usize, min: i32, max: i32) -> Vec<i32> {
    let mut vector: Vec<i32> = vec![];
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        vector.push(rng.gen_range(min..=max));
    }
    vector
}

// Performs summation over an i32 vector using a simple for loop
// (Used as the standard with which to compare the threaded function)
fn sum_vec_loop(vector: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in 0..vector.len() {
        sum += vector[i];
    }
    sum
}

// Performs summation over an i32 vector using threads
fn sum_vec_threads(vector: &Vec<i32>, thread_num: usize) -> i32 {
    let mut children = vec![];
    let mut sum = 0;
    let thread_size = vector.len() / thread_num;
    assert!(thread_num < vector.len());

    // Create all the threads
    for i in 0..thread_num {
        // Figure out which section of the vector this thread should sum up
        let start = i * thread_size;
        let end = if i == thread_num - 1 {
            vector.len()
        } else {
            (i + 1) * thread_size
        };

        // Copy this section over to a new vector called sub_vector
        // (I know this partially negates the speed that you gain with threads, but I couldn't figure out how
        // to make it work otherwise. I tried using a reference to vector, but there was an issue with lifetimes.)
        let mut sub_vector: Vec<i32> = vec![];
        for j in start..end {
            sub_vector.push(vector[j]);
        }

        // Run the thread to sum up this section of the vector
        children.push(thread::spawn(move|| {
            let mut sub_sum = 0;
            for k in 0..sub_vector.len() {
                sub_sum += sub_vector[k];
            }
            sub_sum
        }));
    }

    // Get the sub sum from each thread
    for child in children {
        sum += child.join().unwrap();
    }

    // Return the total sum
    sum
}