use std::io;

fn main() {
    println!("Temperature Converter");

    loop {
        println!("Choose an option:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Quit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => celsius_to_fahrenheit(),
            "2" => fahrenheit_to_celsius(),
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please enter 1, 2, or 3."),
        }
    }
}

fn celsius_to_fahrenheit() {
    let temp = get_temperature("Celsius");
    let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
    println!("{temp}°C is equal to {fahrenheit:.2}°F\n");
}

fn fahrenheit_to_celsius() {
    let temp = get_temperature("Fahrenheit");
    let celsius = (temp - 32.0) * 5.0 / 9.0;
    println!("{temp}°F is equal to {celsius:.2}°C\n");
}

fn get_temperature(scale: &str) -> f64 {
    println!("Enter temperature in {scale}:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            get_temperature(scale)
        }
    }
}
