## Трейт Action с методом say
```rust
trait Action {
    fn say(&self) -> ();
}
```

## Структура Person
```rust
struct Person {
    name: String,
}
```

## Реализация трейта Action для Person
```rust
impl Action for Person {
    fn say(&self) -> () {
        println!("Hello, {}", self.name);
    }
}
```
## Использование
- Создаем экземляр структуры Person;
- Вызываем метод say через реализованный trait Action для Person
```rust
fn main() {
    let person: Person = Person {
        name: "Andrey".to_string(),
    };

    person.say();
}
```