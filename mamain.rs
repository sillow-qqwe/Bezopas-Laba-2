use serde::{Serialize, Deserialize};
use std::fs;
use std::error::Error;

// Структура для экспорта/импорта
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

fn export_to_json(users: &Vec<User>, filename: &str) -> Result<(), Box<dyn Error>> {
    let json_string = serde_json::to_string_pretty(users)?;
    fs::write(filename, json_string)?;
    Ok(())
}

fn import_from_json(filename: &str) -> Result<Vec<User>, Box<dyn Error>> {
    let json_string = fs::read_to_string(filename)?;
    let users: Vec<User> = serde_json::from_str(&json_string)?;
    Ok(users)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut users = vec![
        User::new(1, "Regina", "regina@example.com", 25),
        User::new(2, "Bob", "bob@example.com", 150),  // Намеренная ошибка: возраст 150
        User::new(3, "Peter", "no-email", 30),  // Намеренная ошибка: невалидный email
    ];
    
    export_to_json(&users, "users.json")?;
    println!("Данные экспортированы");
    
    let imported_users = import_from_json("users.json")?;
    
    for user in &imported_users {
        if user.age > 120 {
            println!("Ошибка: у пользователя {} невалидный возраст: {}", user.name, user.age);
        }
        
        if !user.email.contains('@') {
            println!("Ошибка: у пользователя {} невалидный email: {}", user.name, user.email);
        }
        
        if user.name.is_empty() {
            println!("Ошибка: пустое имя у пользователя с id {}", user.id);
        }
    }
    
    Ok(())
}