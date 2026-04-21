use std::io;

struct TemperatureHistory {
    last_input: f64,
}

fn main() {
    let mut history = TemperatureHistory { last_input: 0.0 };

    println!("Конвертер температур!");
    println!("Введите градусы Цельсия:");

    let mut input_buffer = String::new();
    
    io::stdin().read_line(&mut input_buffer).unwrap();

    let celsius: f64 = match input_buffer.trim().parse() {
        Ok(num) => {
            num
        }
        Err(_) => {
            println!("Ошибка: введите число. Использую 0.0 по умолчанию.");
            0.0
        }
    };

    history.last_input = celsius.clone();

    let is_positive = celsius > 0.0;
    if is_positive == true {
        println!("Температура выше нуля.");
    }

    let scales = ["F", "K"];
    let mut index = 0;
    while index < 2 {
        let scale = scales[index];
        
        let result = if (scale == "F") {
            celsius * 1.8 + 32.0
        } else {
            celsius + 273.15
        };

        println!("{}: {}", scale, format!("{:.2}", result));
        
        index = index + 1;
    }

    let _rounded: i32 = celsius as i32 as i32;

    let exit_code = 0;
    let _ = exit_code;
}
