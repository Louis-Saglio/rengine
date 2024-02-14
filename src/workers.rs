use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, SendError};
use std::thread;

#[derive(Debug)]
struct DataToProcess {
    name: &'static str,
    age: u8,
}

fn worker_main(data_to_process_receiver: Arc<Mutex<Receiver<DataToProcess>>>) -> u8 {
    let data = data_to_process_receiver.lock().unwrap().recv().unwrap();
    return data.age;
}


pub fn run() {
    let (data_to_process_sender, data_to_process_receiver) = mpsc::channel();
    let data_to_process_receiver = Arc::new(Mutex::new(data_to_process_receiver));
    // Create workers
    // launch workers
    let mut workers = Vec::new();
    for _ in 0..5 {
        let data_to_process_receiver = Arc::clone(&data_to_process_receiver);
        workers.push(thread::spawn(|| worker_main(data_to_process_receiver)));
    }
    // distribute work to workers
    for i in 0..15 {
        let sending_result = data_to_process_sender.send(DataToProcess {name: "Louis", age: i});
        match sending_result {
            Ok(_) => {println!("{} sent successfully", i)}
            Err(error) => {
                match error { SendError(error) => {
                    println!("failed to send {error:?}");
                } }
            }
        }
    }
    // close workers
    drop(data_to_process_sender);
    // wait for workers
    let mut results = vec![];
    for worker in workers {
        let result = worker.join();
        let b = result.unwrap();
        results.push(b);
    }
    // collect workers output

    // display output
    println!("{results:?}");
}