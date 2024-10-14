// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // Create an Arc containing a Mutex wrapping JobStatus
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        // Clone the Arc to share the Mutex among threads
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // Lock the mutex before updating the shared value
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1; // Increment the completed jobs
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // Wait for all threads to finish
    }

    // Lock the mutex to read the final value of jobs_completed
    let final_status = status.lock().unwrap();
    println!("jobs completed {}", final_status.jobs_completed); // Print the final count
}