use std::thread;
use std::sync::mpsc;

fn main() {
    let n = 10;
    let mut handles = Vec::with_capacity(n as usize);
    let (tx, rx) = mpsc::channel();

    for i in 1..=n {
        let thread_tx = tx.clone();
        let handel = thread::spawn(move || {
            let square = i*i;
            thread_tx.send(square).unwrap();
        });
        handles.push(handel);
    }

    drop(tx);

    for handel in handles {
        handel.join().unwrap();
    }

    let mut res = 0;
    for receive in rx {
        res += receive;
        println!("receive: {receive}");
    }
    println!("res = {res}")
}