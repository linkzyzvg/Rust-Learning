use std::io::{self, Write};

fn main() {
    println!("Rust Temperature Converter");

    loop {
        // Get user input
        print!("\nEnter a temperature value (or type 'exit' to quit): ");
        // Ensure the prompt is displayed before reading input
        io::stdout().flush().unwrap();

        let mut input_value = String::new();
        io::stdin()
            .read_line(&mut input_value)
            .expect("Failed to read line");

        // Trim whitespace and check if the user wants to exit
        let input_value = input_value.trim();

        if input_value.to_lowercase() == "exit" {
            println!("Exiting...");
            break;
        }

        // Convert string input to a float
        let value: f64 = match input_value.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Get the current starting unit
        print!("Is this value in (C)elsius, (F)ahrenheit, or (K)elvin?: ");
        io::stdout().flush().unwrap();

        let mut input_from_unit = String::new();
        io::stdin()
            .read_line(&mut input_from_unit)
            .expect("Failed to read line");
        let from_unit = input_from_unit.trim().to_uppercase();

        // Get the target conversion unit
        print!("Convert it to (C)elsius, (F)ahrenheit, or (K)elvin?: ");
        io::stdout().flush().unwrap();

        let mut input_to_unit = String::new();
        io::stdin()
            .read_line(&mut input_to_unit)
            .expect("Failed to read line");
        let to_unit = input_to_unit.trim().to_uppercase();

        // Process all combinations using a Tuple Match
        match (from_unit.as_str(), to_unit.as_str()) {
            // Same unit conversions
            ("C", "C") | ("F", "F") | ("K", "K") => {
                println!(
                    "No conversion needed! It is already {:.2}°{}",
                    value, to_unit
                );
            }
            // Celsius conversions
            ("C", "F") => println!("{:.2}°C is {:.2}°F", value, (value * 9.0 / 5.0) + 32.0),
            ("C", "K") => println!("{:.2}°C is {:.2} K", value, value + 273.15),

            // Fahrenheit conversions
            ("F", "C") => println!("{:.2}°F is {:.2}°C", value, (value - 32.0) * 5.0 / 9.0),
            ("F", "K") => println!(
                "{:.2}°F is {:.2} K",
                value,
                (value - 32.0) * 5.0 / 9.0 + 273.15
            ),

            // Kelvin conversions
            ("K", "C") => println!("{:.2} K is {:.2}°C", value, value - 273.15),
            ("K", "F") => println!(
                "{:.2} K is {:.2}°F",
                value,
                (value - 273.15) * 9.0 / 5.0 + 32.0
            ),

            // Fallback for invalid characters
            _ => {
                println!("❌ Invalid unit selection. Please use C, F, or K.");
            }
        }
    }
}
