use std::io;

enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

fn main() {
    println!("=====================");
    println!("Temperature Converter");
    println!("=====================");

    loop {
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Exit");
    
        let mut choice = String::new();
    
        io::stdin()
            .read_line(&mut choice)
            .expect("❌ Failed to read input!");
    
        let choice: u64 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid input, please enter a number.");
                return;
            }
        };
    
        match choice {
            1 => {
                let temperature = get_temperature_input(TemperatureUnit::Celsius);
                let converted_temperature = celsius_to_fahrenheit(temperature);
                println!("🌡 {}°C is equal to {}°F", temperature, converted_temperature);
                println!();
            }
            2 => {
                let temperature = get_temperature_input(TemperatureUnit::Fahrenheit);
                let converted_temperature = fahrenheit_to_celsius(temperature);
                println!("🌡 {}°F is equal to {}°C", temperature, converted_temperature);
                println!();
            }
            3 => {
                println!("👋 Exiting program...");
                break;
            }
            _ => {
                println!("❌ Invalid option selected.");
            }
        }    
    }
}

fn get_temperature_input(unit: TemperatureUnit) -> f64 {
    let unit_str = match unit {
        TemperatureUnit::Celsius => "Celsius",
        TemperatureUnit::Fahrenheit => "Fahrenheit",
    };

    loop {
        let mut temperature = String::new();
        println!("Enter the temperature in {}: ", unit_str);

        io::stdin()
            .read_line(&mut temperature)
            .expect("❌ Failed to read line!");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid input, please enter a number.");
                continue;
            }
        };
        return temperature;
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
