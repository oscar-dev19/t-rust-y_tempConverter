//! A Basic Temperature Conversion Program
//!
//! This program converts temperatures from Fahrenheit to Celsius and vice versa.
//!
//! It prompts the user to input the temperature unit ('f' for Fahrenheit or 'c' for Celsius) followed by the temperature value.
//! The program then converts the temperature to the other unit and displays the result.
//!
//! # Example
//!
//! ```bash
//! $ cargo run
//! Determine degrees in fahrenheit or celsius by entering 'f' or 'c' respectively followed by a space and the degrees.
//! f 100
//! You entered 100 degrees Fahrenheit.
//! Converted degrees: 37.77777777777778
//! ```
//!
//! # Functions
//!
//! * `fahrenheit_to_celsius(degrees: i32) -> f32`: Converts degrees Fahrenheit to Celsius.
//! * `celsius_to_fahrenheit(degrees: i32) -> f32`: Converts degrees Celsius to Fahrenheit.
//! * `display_banner()`: Displays a banner to the console.
//! * `input_degrees() -> i32`:prompts user to enter degrees to convert.
//! * `input_unit()` -> char`: prompts userr to input unit of entered degrees.

use std::io;

fn main() {
    display_banner();
    let degrees = input_degrees();
    let unit = input_unit();
    let converted_degrees = loop {
        match unit {
            'f' => {
                println!("You entered {} degrees Fahrenheit.", degrees);
                break fahrenheit_to_celsius(degrees);
            },
            'c' => {
                println!("You entered {} degrees Celsius.", degrees);
                break celsius_to_fahrenheit(degrees);
            },
            _ => {
                println!("Invalid unit. Please enter 'f' for Fahrenheit or 'c' for Celsius.");
                continue;
            },
        }
    };
    println!("Converted degrees: {}", converted_degrees);
}

fn fahrenheit_to_celsius(degrees: i32) -> f32 {
    (degrees as f32 - 32.0) * (5.0/9.0)
}

fn celsius_to_fahrenheit(degrees: i32) -> f32 {
    (degrees as f32 * (9.0/5.0)) + 32.0
}

fn display_banner() {
    println!("--- ___ .-.. ..-.--- ___ . .    ---.--.  ..-.       ");
    println!(" |      |-'| |`-. |       Y      | |- |'\\/||-'       ");
    println!(" '      '`-`-'`-' '       '      ' '--'  ''         ");
    println!("                                                     ");
    println!("                                                     ");
    println!(" .-.-.. .. ..--.-.---.--.-.                         ");
    println!("(  | ||\\|| ||- |-' | |- |-'                         ");
    println!(" `-`-'' ' ` '--'`- ' '--'`-                         ");
    println!("                                                     ");
    println!(" --  --  --  --  --  --  --  --  --  --  --  --  -- ");
    println!(" --  --  --  --  --  --  --  --  --  --  --  --  -- ");
}

fn input_degrees() -> i32 {
    loop {
        println!("Enter degrees to convert:");
        let mut degrees = String::new();
        io::stdin().read_line(&mut degrees).expect("Failed to read degree input.");
        match degrees.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a valid numerical value.");
                continue;
            }
        }
    }
}

fn input_unit() -> char {
    loop {
        println!("Enter degree unit to convert from (f for fahrenheit or c for celsius):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().to_lowercase().as_str() {
            "f" => break 'f',
            "c" => break 'c',
            _ => {
                println!("Please enter a valid unit ('f' for fahrenheit or 'c' for celsius)");    
                continue;
            }
        }; 
    }
}