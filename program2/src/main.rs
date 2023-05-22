use std::sync::{Arc, Mutex};
use std::thread;

// Number of threads to run
const THREADS: u32 = 30;
// Number of transactions per thread
const TRANSACTIONS: u32 = 1000;

fn main() {
    let balance = Arc::new(Mutex::new(0.0));
    let mut children = vec![];

    // Create all the threads
    for i in 0..THREADS {
        let balance_clone = Arc::clone(&balance);
        let child = thread::spawn(move|| {
            if i % 2 == 0 {
                // If even, deposit 100
                for _ in 0..TRANSACTIONS {
                    deposit(&balance_clone, 100.0);
                }
            } else {
                // If odd, withdraw 100
                for _ in 0..TRANSACTIONS {
                    withdraw(&balance_clone, 100.0);
                }
            }
        });
        children.push(child);
    }

    // Wait for all the threads to finish
    for child in children {
        child.join().unwrap();
    }

    // Print out the final balance
    println!("balance = {}", *balance.lock().unwrap());
}

// Function adds amount to the value stored in balance
fn deposit(balance: &Arc<Mutex<f64>>, amount: f64) {
    let mut b = balance.lock().unwrap();
    *b += amount;
}

// Function subtracts amount from the value stored in balance
fn withdraw(balance: &Arc<Mutex<f64>>, amount: f64) {
    let mut b = balance.lock().unwrap();
    *b -= amount;
}