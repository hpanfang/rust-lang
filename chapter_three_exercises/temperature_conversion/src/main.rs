use std::io;

fn main() {
    println!("enter a temperature value");

    let mut temperature = String::new();

    io::stdin().read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: u32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("[1] Fahrenheit");
    println!("[2] Celsius");
    println!("please enter the number corresponding to the temperature scale of your input");

    let mut scale = String::new();

    io::stdin().read_line(&mut scale)
        .expect("Failed to read line");

    let scale: u32 = match scale.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut converted_temperature :u32 = 0;

    if scale == 2 {
        converted_temperature = convert_to_fahrenheit(temperature);
    } else if scale == 1 {
        converted_temperature = convert_to_celsius(temperature);
    } else {
        println!("not a valid options! Please select 1 (F) or 2 (C)");
    }

    println!("The converted temperature is: {}", converted_temperature);
}

fn convert_to_fahrenheit(temperature_celsius: u32) -> u32 {
    temperature_celsius * 9 / 5 + 32
}

fn convert_to_celsius(temperature_fahrenheit: u32) -> u32 {
    (temperature_fahrenheit - 32) * 5 / 9
}