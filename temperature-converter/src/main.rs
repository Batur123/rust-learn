// Temperature Converter for learning purposes from rust documentation

use std::io;

fn convert_celcius_to_kelvin(degree: f64) -> f64 {
    degree + 273.15
}

fn convert_celcius_to_fahrenheit(degree: f64) -> f64 {
    (degree * (9.0 / 5.0)) + 32.0
}

fn main() {
    let mut guess = String::new();
    println!("Please input your degree to convert into kelvin and fahrenheit.");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: f64 = match guess.trim().parse() {
        Ok(number) => number,
        Err(e) => {
            println!("Error parsing input: {}", e);
            return;
        },
    };

    let kelvin = convert_celcius_to_kelvin(guess);
    let fahrenheit = convert_celcius_to_fahrenheit(guess);
    println!("Celcius: {}, Fahrenheit: {}, Kelvin: {}", guess, fahrenheit, kelvin);
}