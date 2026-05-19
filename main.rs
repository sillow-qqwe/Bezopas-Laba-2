use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// структура с полями, которые никогда не читаются
struct User {
    id: u32,
    name: String,
    age: Option<u8>,
}

fn calculate_average(values: &Vec<i32>) -> f64 {
    // использование .unwrap() без проверки
    let first = values.first().unwrap();
    
    // итерация с индексами вместо итератора
    let mut sum = 0;
    for i in 0..values.len() {
        sum += values[i];
    }
    
    // неиспользуемая переменная
    let unused_var = 42;
    
    // избыточный return
    return (sum as f64) / (values.len() as f64);
}

// функция с передачей владения без необходимости
fn read_config(path: String) -> Result<String, std::io::Error> {
    // забираем владение path без нужды (можно &str)
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// match на единственный вариант
fn check_option(opt: Option<i32>) -> i32 {
    match opt {
        Some(x) => x,
        _ => 0,
    }
}

fn main() {
    // создание string из &str неэффективным способом
    let greeting = "hello".to_owned();
    
    // пустой HashMap
    let mut map = HashMap::new();
    map.insert("key", "value");
    
    // двойная ссылка
    let x = 5;
    let y = &x;
    let z = **&y;
    
    // использование format! с одним аргументом
    let msg = format!("{}", greeting);
    
    // клонирование значения, которое можно просто скопировать
    let num = 10;
    let cloned = num.clone(); // i32 реализует copy
    
    // сравнение i32 с bool
    if cloned == true {
        println!("True");
    }
}
