use std::{env, thread};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_of_workers: usize = args.get(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(2);

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    thread::spawn(move || {
        let mut cnt = 0;
        loop {
            cnt += 1;
            tx.send(cnt).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    let mut handles = vec![];

    for _ in 0..num_of_workers {
        let worker_rx = rx.clone();
        let handle = thread::spawn(move || {
            loop{
                let receive = worker_rx.lock().unwrap().recv();
                let thread_id = thread::current().id();
                match receive {
                    Ok(result) => {
                        println!("{thread_id:?}: {result}");
                    }
                    Err(e) => {
                        println!("{e}");
                        println!("{thread_id:?}: closed channel -> worker shutting down");
                        break;
                    }
                }
            };
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
}
