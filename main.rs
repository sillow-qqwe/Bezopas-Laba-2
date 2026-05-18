use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// Структура с полями, которые никогда не читаются
struct User {
    id: u32,
    name: String,
    age: Option<u8>,
}

// Функция с множеством линтерных ошибок
fn calculate_average(values: &Vec<i32>) -> f64 {
    // Ошибка: использование .unwrap() без проверки
    let first = values.first().unwrap();
    
    // Ошибка: итерация с индексами вместо итератора
    let mut sum = 0;
    for i in 0..values.len() {
        sum += values[i];
    }
    
    // Ошибка: неиспользуемая переменная
    let unused_var = 42;
    
    // Ошибка: избыточный return
    return (sum as f64) / (values.len() as f64);
}

// Функция с передачей владения без необходимости
fn read_config(path: String) -> Result<String, std::io::Error> {
    // Ошибка: забираем владение path без нужды (можно &str)
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Функция с ОШИБКОЙ: match на единственный вариант
fn check_option(opt: Option<i32>) -> i32 {
    match opt {
        Some(x) => x,
        _ => 0, // Клиппи предложит убрать match
    }
}

fn main() {
    // Ошибка: создание String из &str неэффективным способом
    let greeting = "hello".to_owned();
    
    // Ошибка: пустой HashMap, который можно не создавать
    let mut map = HashMap::new();
    map.insert("key", "value");
    
    // Ошибка: двойная ссылка
    let x = 5;
    let y = &x;
    let z = **&y; // Избыточное разыменование
    
    // Ошибка: использование format! с одним аргументом
    let msg = format!("{}", greeting);
    
    // Ошибка: клонирование значения, которое можно просто скопировать
    let num = 10;
    let cloned = num.clone(); // i32 реализует Copy
    
    // Ошибка: сравнение с bool
    if cloned == true { // i32 нельзя сравнивать с bool (тут компиляция упадет)
        println!("True");
    }
}
