use std::env;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

async fn start_program() {
    let (tx, mut rx) = mpsc::channel(100);

    let tx_clone = tx.clone();

    tokio::spawn(async move {
        let mut i = 0;
        loop {
            if tx_clone.send(i).await.is_err() {
                println!("Receiver dropped");
                return
            }
            i += 1;
            time::sleep(Duration::from_millis(1000)).await;
        }
    });

    tokio::spawn(async move {
        while let Some(value) = rx.recv().await {
            println!("I got - {value}");
        }
    }).await.unwrap();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u64 = args.get(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(5);

    let timer = time::sleep(Duration::from_secs(n));
    let start_program_handle = tokio::spawn(start_program());

    tokio::select! {
        _ = start_program_handle => (),
        _ = timer => {
            println!("Time is over! Stopping the program.");
        }
    }

    println!("Program stopped");
}
