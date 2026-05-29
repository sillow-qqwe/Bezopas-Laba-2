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

// Экспорт в JSON
fn export_to_json(users: &Vec<User>, filename: &str) -> Result<(), Box<dyn Error>> {
    let json_string = serde_json::to_string_pretty(users)?;
    fs::write(filename, json_string)?;
    Ok(())
}

// Импорт из JSON
fn import_from_json(filename: &str) -> Result<Vec<User>, Box<dyn Error>> {
    let json_string = fs::read_to_string(filename)?;
    let users: Vec<User> = serde_json::from_str(&json_string)?;
    Ok(users)
}

// Функция для создания JSON с намеренными ошибками
fn create_broken_json_files() -> Result<(), Box<dyn Error>> {
    // Ошибка: пропущена кавычка у поля email
    fs::write("users_broken.json", r#"[
        {"id": 1, "name": "Alice", "email": "alice@test.com", "age": 25},
        {"id": 2, "name": "Bob", "email: "bob@test.com", "age": 30}
    ]"#)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Создаём валидные данные
    let users = vec![
        User::new(1, "Alice", "alice@example.com", 25),
        User::new(2, "Bob", "bob@example.com", 30),
    ];
    
    // Экспортируем валидный JSON
    export_to_json(&users, "users_valid.json")?;
    println!("✅ Валидный JSON сохранён");
    
    // Успешный импорт
    match import_from_json("users_valid.json") {
        Ok(imported) => println!("✅ Импорт успешен: {:?}", imported),
        Err(e) => println!("❌ Ошибка: {}", e),
    }
    
    // Создаём и пробуем импортировать сломанный JSON
    create_broken_json_files()?;
    println!("\n📁 Пробуем импортировать сломанный JSON:");
    
    match import_from_json("users_broken.json") {
        Ok(users) => println!("❌ Неожиданно успешно: {:?}", users),
        Err(e) => println!("✅ Ожидаемая ошибка импорта: {}", e),
    }
    
    Ok(())
}