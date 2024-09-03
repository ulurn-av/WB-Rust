use std::io;
use std::thread::spawn;

fn calculate_squares(n: u32) {
    let mut handles = vec![];

    for i in 1..=n {
        let handle = spawn(move || {
            let square = i*i;
            println!("{square}");
        });

        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Не удалось прочитать входные данные");
    let n: u32 = n.trim().parse().expect("Введите положительное число");

    calculate_squares(n);
}