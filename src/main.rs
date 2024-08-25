use std::io;

fn main() {
    // Prompt for unit choice
    println!("Enter the unit type (1 for Celsius, 2 for Fahrenheit):");

    // Reading unit choice input
    let mut unit_choice = String::new();
    io::stdin()
        .read_line(&mut unit_choice)
        .expect("Failed to read line");

    // Convert the unit choice to an integer
    let unit_choice: i32 = unit_choice.trim().parse().expect("Please enter a number!");

    // Prompt for the temperature value
    println!("Enter the temperature:");

    // Reading temperature input
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    // Convert the temperature to a floating-point number
    let temperature: f32 = temperature.trim().parse().expect("Please enter a valid number!");

    // Initialize variable for converted temperature
    let converted_temperature;

    // Perform the conversion based on the unit choice
    if unit_choice == 1 {
        // Celsius to Fahrenheit
        converted_temperature = temperature * 9.0 / 5.0 + 32.0;
        println!("Temperature in Fahrenheit: {:.2}", converted_temperature);
    } else if unit_choice == 2 {
        // Fahrenheit to Celsius
        converted_temperature = (temperature - 32.0) * 5.0 / 9.0;
        println!("Temperature in Celsius: {:.2}", converted_temperature);
    } else {
        println!("Invalid unit choice. Please enter 1 for Celsius or 2 for Fahrenheit.");
    }
}