use std::time::Instant;

// use rayon::prelude::*;
// use std::time::Instant;
//
// fn main() {
//     let myname: Vec<String> = (0..100_000).map(|i| format!("Item {}", i)).collect();
//
//     // Start timing
//     let start = Instant::now();
//
//     myname.par_iter().for_each(|val| {
//         // Simulate processing
//         std::thread::sleep(std::time::Duration::from_millis(10));
//         println!("{}", val); // Processing logic here
//     });
//
//     // End timing
//     let duration = start.elapsed();
//     println!("Concurrent processing with thread pool took {:?}", duration);
// }

// use tokio::sync::mpsc;
// use tokio::time::{sleep, Duration};
//
// #[tokio::main]
// async fn main() {
//     let myname: Vec<String> = (0..100_000).map(|i| format!("Item {}", i)).collect();
//
//     // Create a channel
//     let (tx, mut rx) = mpsc::channel(100); // Buffer size of 100
//
//     // Start timing
//     let start = std::time::Instant::now();
//
//     // Spawn tasks for each item
//     for val in myname {
//         let tx_clone = tx.clone();
//         // Spawn an asynchronous task
//         tokio::spawn(async move {
//             // Simulate work
//             sleep(Duration::from_millis(10)).await;
//             tx_clone.send(val).await.unwrap();
//         });
//     }
//
//     // Drop the sender to close the channel when all tasks are done
//     drop(tx);
//
//     let mut received_count = 0;
//     while let Some(_received) = rx.recv().await {
//         received_count += 1;
//         // Simulate processing
//         println!("Received: {}", received_count);
//     }
//
//     // End timing
//     let duration = start.elapsed();
//     println!("Concurrent processing with Tokio took {:?}", duration);
// }

// fn main() {
//     let myname: Vec<String> = (0..100_000).map(|i| format!("Item {}", i)).collect();
//     let start = Instant::now();
//     for _val in myname {
//         // Process each item
//         // println!("{}", val);
//     }
//     let duration = start.elapsed();
//     println!("Sequential processing took {:?}", duration);
// }
use std::sync::mpsc;
use std::thread;
// // use std::time::Duration;
//
fn main() {
    let myname: Vec<String> = (0..100_000).map(|i| format!("Item {}", i)).collect();

    let (tx, rx) = mpsc::channel();

    let start = Instant::now();

    for val in myname {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            // Simulate processing
            // thread::sleep(Duration::from_millis(10));
            tx_clone.send(val).unwrap();
            // drop(tx_clone);
        });
    }

    drop(tx); // Close the channel to end the receiver loop

    for _received in rx {
        // println!("{}", received);
    }
    let duration = start.elapsed();
    println!("Concurrent processing took {:?}", duration);
}
// use std::sync::mpsc::{self, Receiver, Sender};
// use std::thread;
// // use std::time::Duration;
//
// fn main() {
//     let myname: Vec<String> = vec![
//         "Sony".to_string(),
//         "Nurdianto".to_string(),
//         "Bin".to_string(),
//         "Lutfianto".to_string(),
//     ];
//
//     let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
//
//     let start = Instant::now();
//     for val in myname.into_iter() {
//         let tlx: Sender<String> = tx.clone();
//         thread::spawn(move || {
//             tlx.send(val.to_owned()).unwrap();
//             // thread::sleep(Duration::from_secs(1));
//             // drop(tlx);
//         });
//     }
//
//     drop(tx);
//
//     // let mut sure_name: String = String::new();
//
//     for _received in rx {
//         // sure_name.push_str(&received);
//         // println!("Got: {}", received);
//     }
//
//     // println!("{}", sure_name);
//     let duration = start.elapsed();
//     println!("Concurrent processing took {:?}", duration);
// }
