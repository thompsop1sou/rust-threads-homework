use std::thread;
use rand::Rng;

fn main() {
    println!("Creating random vector...");
    const MEBI: usize = 1024*1024;
    let vector = create_rand_vec(16*MEBI, -128, 128);
    println!("Summing vector elements with a loop...");
    println!("  vector sum = {}", sum_vec_loop(&vector));
    println!("Summing vector elements with threads...");
    println!("  vector sum = {}", sum_vec_threads(&vector, 16));
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
    let mut sum = 0;
    let thread_size = vector.len() / thread_num;
    assert!(thread_num < vector.len());

    // Create a thread scope so that the rust compiler knows all threads will be joined
    // by the end (this lets us borrow a reference to the vector inside of each thread)
    thread::scope(|thread_scope| {
        let mut children = vec![];

        // Create all the threads
        for i in 0..thread_num {
            // Figure out which section of the vector this thread should sum up
            let start = i * thread_size;
            let end = if i == thread_num - 1 {
                vector.len()
            } else {
                (i + 1) * thread_size
            };

            // Run the thread to sum up this section of the vector
            let child = thread_scope.spawn(move || {
                let mut sub_sum = 0;
                for j in start..end {
                    sub_sum += vector[j];
                }
                sub_sum
            });
            children.push(child);
        }

        // Get the sub sum from each thread
        for child in children {
            sum += child.join().unwrap();
        }
    });

    // Return the total sum
    sum
}