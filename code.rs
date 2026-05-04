use std::fs;
use std::io;

// Константа с неправильным именованием (должна быть SCREAMING_SNAKE_CASE)
const max_retries: u32 = 3;
// Неиспользуемая константа
const UNUSED_VALUE: i32 = 100;

// Структура без документации
struct UserData {
    name: String,
    email: Option<String>,
    age: i32, // возраст добавляется, но не используется
}

// Реализация с клонированием там, где можно использовать ссылки
impl UserData {
    fn new(name: &str, email: &str) -> UserData {
        UserData {
            name: name.to_string(),
            email: Some(email.to_string()),
            age: 0,
        }
    }

    // Метод с излишним клонированием
    fn print_info(&self) {
        let name = self.name.clone();
        let email = self.email.clone().unwrap_or_else(|| "no email".to_string());
        println!("User: {} <{}>", name, email);
    }

    // Метод, возвращающий Result, который часто игнорируется
    fn save_to_file(&self, path: &str) -> Result<(), io::Error> {
        let content = format!("{}:{}", self.name, self.email.as_ref().unwrap_or(&"unknown".to_string()));
        fs::write(path, content)
    }
}

// Функция с небезопасным кодом без явного unsafe-блока (ошибка компиляции, но плохая практика)
// На самом деле unsafe требуется, этот пример показывает raw pointer
fn dangerous_operation() {
    let mut value = 42;
    let ptr: *mut i32 = &mut value;
    unsafe {
        *ptr = 100; // Изменение через сырой указатель
        println!("Dangerous value: {}", *ptr);
    }
}

// Функция с избыточным match
fn process_option(opt: Option<i32>) -> i32 {
    match opt {
        Some(x) => x,
        None => 0,
    }
}

// Функция с ошибкой: заимствование после перемещения
fn ownership_mistake() {
    let user = UserData::new("Alice", "alice@example.com");
    let user2 = user; // user перемещен
    // println!("{}", user.name); // ошибка: использование перемещенного значения
    user2.print_info();
}

// Функция со сложной логикой (высокая цикломатическая сложность)
fn complex_decision(a: i32, b: i32, c: i32, d: i32) -> &'static str {
    if a > b {
        if c > d {
            if a + c > b + d {
                "Path 1"
            } else {
                if a > c {
                    "Path 2"
                } else {
                    "Path 3"
                }
            }
        } else if c == d {
            "Path 4"
        } else {
            "Path 5"
        }
    } else if a == b {
        "Path 6"
    } else {
        "Path 7"
    }
}

fn main() {
    // Игнорирование Result
    fs::write("/tmp/test.txt", b"Hello");

    // Неиспользуемая переменная
    let unused_var = "I'm not used";

    // Изменяемая переменная, которая не изменяется
    let mut should_be_const = 42;

    // Излишнее создание String
    let greeting: String = "Hello".to_string();
    println!("{}", greeting);

    // Ошибка: пустой цикл
    loop {
        // Забыли добавить break
        break; // Исправлено для компиляции, в реальном коде может отсутствовать
    }

    // Неправильное использование итератора
    let numbers = vec![1, 2, 3, 4, 5];
    for i in 0..numbers.len() {
        println!("{}", numbers[i]); // Вместо for num in &numbers
    }

    // Вызов функции, возвращающей Result, без обработки
    let user = UserData::new("Bob", "bob@example.com");
    user.save_to_file("/tmp/user.txt"); // Result игнорируется

    // Использование deprecated или неидиоматичного кода
    let opt = Some(10);
    let val = process_option(opt);
    println!("Value: {}", val);

    // Неправильный формат строки
    println!("Complex result: {}", complex_decision(1, 2, 3, 4));

    // Потенциальное переполнение стека при рекурсии (если бы она была)
    should_be_const += 1;
    println!("Should be const: {}", should_be_const);

    // Вызов опасной операции
    dangerous_operation();

    // Перемещение после заимствования
    ownership_mistake();

    // Мертвый код (unreachable из-за предыдущего break)
    let dead_code = "This will never execute";
    println!("{}", dead_code);
}
