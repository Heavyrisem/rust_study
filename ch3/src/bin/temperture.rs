use std::io;

fn main() {
    let mut input_type = String::new();
    let mut value = String::new();

    println!("Input temperture type (C, F)");
    io::stdin()
        .read_line(&mut input_type)
        .expect("Failed to read temperture type");
    let input_type = input_type.trim();

    println!("Input temperture");
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read temperture");
    let value: f64 = value.trim().parse().expect("invalid temperture value");

    let result = if input_type == "C" {
        celsius_to_fahrenheit(value)
    } else if input_type == "F" {
        fahrenheit_to_celsius(value)
    } else {
        println!("invalid temperture type");
        return;
    };

    println!("Result is: {}", result);
}

fn celsius_to_fahrenheit(c: f64) -> String {
    format!("{} F", c * 1.8 + 32.0)
}

fn fahrenheit_to_celsius(f: f64) -> String {
    format!("{} C", (f - 32.0) * 5.0 / 9.0)
}
