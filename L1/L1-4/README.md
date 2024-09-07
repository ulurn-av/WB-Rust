## Как запустить

Для запуска программы используйте следующую команду:

```bash
cargo run -- <число_потоков>
```

Где `<число_потоков>` — это количество рабочих потоков. Если аргумент не указан, программа по умолчанию создаст 2 потока.

Например:

```bash
cargo run -- 4
```

## Пояснение кода

```rust
use std::{env, thread};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

fn main() {
    // Получаем количество рабочих потоков из аргументов командной строки или по умолчанию 2
    let args: Vec<String> = env::args().collect();
    let num_of_workers: usize = args.get(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(2);
    
    // Создаем канал для передачи данных
    let (tx, rx) = mpsc::channel();
    // Делаем приемник многопоточным с помощью Arc и Mutex
    let rx = Arc::new(Mutex::new(rx));
    
    // Постоянная запись в канал
    thread::spawn(move || {
        let mut cnt = 0;
        loop {
            cnt += 1;
            // Отправляем число через канал
            tx.send(cnt).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    let mut handles = vec![];
    
    // Запускаем воркеров
    for _ in 0..num_of_workers {
        let worker_rx = rx.clone();
        let handle = thread::spawn(move || {
            loop{ 
                // Получаем данные из канала
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
    
    // Ожидание завершения всех потоков
    for handle in handles{
        handle.join().unwrap();
    }
}
```