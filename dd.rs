use serde::{Serialize, Deserialize};
use std::fs;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u8,
}

impl User {
    fn new(id: u32, name: &str, email: &str, age: u8) -> Self {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            age,
        }
    }
}

// Функция для экспорта в JSON
fn export_to_json(users: &Vec<User>, filename: &str) -> Result<(), Box<dyn Error>> {
    let json_string = serde_json::to_string_pretty(users)?;
    fs::write(filename, json_string)?;
    Ok(())
}

// Функция для импорта из JSON
fn import_from_json(filename: &str) -> Result<Vec<User>, Box<dyn Error>> {
    let json_string = fs::read_to_string(filename)?;
    let users: Vec<User> = serde_json::from_str(&json_string)?;
    Ok(users)
}

// НОВАЯ ФУНКЦИЯ: создаёт JSON-файл с намеренными ошибками
fn create_broken_json_files() -> Result<(), Box<dyn Error>> {
    // Ошибка 1: Невалидный JSON (пропущена кавычка)
    fs::write("users_broken_1.json", r#"[
        {"id": 1, "name": "Alice", "email": "alice@test.com", "age": 25},
        {"id": 2, "name": "Bob", "email: "bob@test.com", "age": 30}
    ]"#)?;
    
    // Ошибка 2: Неправильный тип (строка вместо числа)
    fs::write("users_broken_2.json", r#"[
        {"id": "один", "name": "Alice", "email": "alice@test.com", "age": 25}
    ]"#)?;
    
    // Ошибка 3: Отсутствует обязательное поле
    fs::write("users_broken_3.json", r#"[
        {"id": 1, "name": "Alice", "age": 25}
    ]"#)?;
    
    // Ошибка 4: Лишняя запятая в конце массива
    fs::write("users_broken_4.json", r#"[
        {"id": 1, "name": "Alice", "email": "alice@test.com", "age": 25},
    ]"#)?;
    
    // Ошибка 5: Число вне диапазона типа (больше 255 для u8)
    fs::write("users_broken_5.json", r#"[
        {"id": 1, "name": "Alice", "email": "alice@test.com", "age": 300}
    ]"#)?;
    
    // Ошибка 6: Невалидный UTF-8 (бинарные данные)
    fs::write("users_broken_6.json", b"{\"id\": 1, \"name\": \"Alice\xFF\"}")?;
    
    Ok(())
}

// НОВАЯ ФУНКЦИЯ: портит уже существующий валидный JSON
fn corrupt_existing_json(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut content = fs::read_to_string(filename)?;
    
    // Удаляем последний символ (закрывающую скобку)
    content.pop();
    content.push_str(",,,}"); // Добавляем мусор
    
    fs::write(filename, content)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Создаём тестовые данные
    let users = vec![
        User::new(1, "Alice", "alice@example.com", 25),
        User::new(2, "Bob", "bob@example.com", 30),
        User::new(3, "Charlie", "charlie@example.com", 35),
    ];
    
    // 1. Нормальный экспорт
    export_to_json(&users, "users_valid.json")?;
    println!("✅ Экспортирован валидный JSON в users_valid.json");
    
    // 2. Пробуем импортировать валидный файл (работает)
    match import_from_json("users_valid.json") {
        Ok(imported) => println!("✅ Импорт валидного файла успешен: {} пользователей", imported.len()),
        Err(e) => println!("❌ Ошибка: {}", e),
    }
    
    // 3. Создаём сломанные JSON-файлы
    create_broken_json_files()?;
    println!("\n📁 Созданы файлы с намеренными ошибками:");
    
    // 4. Пробуем импортировать каждый сломанный файл
    let broken_files = vec![
        "users_broken_1.json",
        "users_broken_2.json", 
        "users_broken_3.json",
        "users_broken_4.json",
        "users_broken_5.json",
        "users_broken_6.json",
    ];
    
    println!("\n=== ТЕСТИРОВАНИЕ ОШИБОК ИМПОРТА ===\n");
    
    for (i, file) in broken_files.iter().enumerate() {
        println!("📄 Файл {} (ошибка типа {}):", file, i+1);
        match import_from_json(file) {
            Ok(users) => println!("   ❌ НЕ ОЖИДАЛОСЬ: импорт успешен! {:?}", users),
            Err(e) => println!("   ✅ Ожидаемая ошибка: {}", e),
        }
        println!();
    }
    
    // 5. Демонстрация порчи существующего файла
    println!("=== ТЕСТИРОВАНИЕ ПОВРЕЖДЕНИЯ ФАЙЛА ===");
    fs::copy("users_valid.json", "users_to_corrupt.json")?;
    println!("📄 Создана копия users_to_corrupt.json");
    
    // Портирует файл
    corrupt_existing_json("users_to_corrupt.json")?;
    println!("💥 Файл намеренно испорчен");
    
    match import_from_json("users_to_corrupt.json") {
        Ok(users) => println!("❌ Неожиданно успешно: {:?}", users),
        Err(e) => println!("✅ Ошибка импорта после порчи: {}", e),
    }
    
    Ok(())
}