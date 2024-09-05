# Описание
*Файл с решение находится в `WB-Rust/L1/L1-2/src
/main.rs`*

## Считывание размера массива через консоль
```rust
let mut n = String::new();
io::stdin().read_line(&mut n).expect("Не удалось прочитать входные данные");
let n: u32 = n.trim().parse().expect("Введите положительное число");
```

## Подсчитывание квадратов
```rust
fn calculate_squares(n: u32) {
    // Vec для хранения потоков
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(n as usize);
    
    // Запускаем каждый поток для вычисления квадрата числа
    for i in 1..=n {
        // Создаем новый поток
        let handle: JoinHandle<()> = spawn(move || {
            let square = i*i;
            println!("{square}");
        });
        
        // Сохранение handle, чтобы дождаться завершения
        handles.push(handle);
    }
    
    // Дожидаемся завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }
}
```