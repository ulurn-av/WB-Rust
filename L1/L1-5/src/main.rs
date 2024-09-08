use std::future::Future;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use flume::{unbounded, Receiver, RecvError, Sender};
use tokio::signal;


#[tokio::main]
async fn main() {
    let count_of_producers = 4;
    let count_of_consumers = 4;

    let is_shutdown= Arc::new(Mutex::new(false));

    let (tx, rx): (Sender<String>, Receiver<String>) = unbounded();

    let mut producers_handlers = vec![];
    for _ in 0..count_of_producers {
        let producer_tx = tx.clone();
        let is_shutdown = Arc::clone(&is_shutdown);
        let handler = tokio::spawn(async move {
            loop {
                let thread_id = thread::current().id();

                if *is_shutdown.lock().unwrap() {
                    println!("Shutting down thread (producer) {thread_id:?}");
                    break;
                }

                println!("Hello from {:?}", thread_id);
                producer_tx.send_async(format!("{:?}", thread_id)).await.unwrap();
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });
        producers_handlers.push(handler);
    }

    let ctrl_c_future = signal::ctrl_c();

    let mut consumers_handlers = vec![];
    for _ in 0..count_of_consumers {
        let consumer_rx = rx.clone();
        let is_shutdown = Arc::clone(&is_shutdown);
        let handler = tokio::spawn(async move {
            loop {
                let thread_id = thread::current().id();

                if *is_shutdown.lock().unwrap() {
                    println!("Shutting down thread (consumer) {thread_id:?}");
                    break;
                }

                match consumer_rx.recv_async().await {
                    Ok(received_thread) => {
                        println!("I: {:?} got: {:?}", thread_id, received_thread);
                    }
                    Err(err) => {
                        println!("Consumer thread {:?} exiting due to channel closed", thread_id);
                        break;
                    }
                }
            }
        });
        consumers_handlers.push(handler);
    }

    println!("{:?}", consumers_handlers);
    tokio::select! {
        _ = ctrl_c_future => {
            *is_shutdown.lock().unwrap() = true;
            println!("Ctrl+C -> shutting down");
        }
    }

    drop(tx);

    for handle in consumers_handlers {
        handle.await.unwrap();
        println!("consumer");
    }
    for handle in producers_handlers {
        handle.await.unwrap();
        println!("producer");
    }
    println!("All tasks have been shut down gracefully.");
}
