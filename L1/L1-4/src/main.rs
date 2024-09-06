use std::thread;
use std::sync::mpsc;

fn main() {
    let num_of_workers = 8;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut cnt = 0;
        loop {
            cnt += 1;
            tx.send(cnt).unwrap();
        }
    });
}
