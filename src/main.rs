//! A Basic Temperature Conversion Program
//!
//! This program converts temperatures from Fahrenheit to Celsius and vice versa.
//!
//! It prompts the user to input the temperature unit ('f' for Fahrenheit or 'c' for Celsius) followed by the temperature value.
//! The program then converts the temperature to the other unit and displays the result.
//!
//! # Example
//!
//! ```
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
//! * `get_user_input() -> (char, i32)`: Prompts the user for input and returns the unit and degrees as a tuple.

use std::io;

fn main() {
    display_banner();
    let user_input = get_user_input(); 

    let unit = user_input.0;
    let degrees = user_input.1; 

    let converted_degrees = if unit == 'f' 
    {
        println!("You entered {} degrees Fahrenheit.",degrees);
        fahrenheit_to_celsius(degrees)
    } 
    else 
    {
        celsius_to_fahrenheit(degrees)
    };

    println!("Converted degrees: {}", converted_degrees);
}

/// Converts degrees Fahrenheit to Celsius.
/// 
/// * 'degrees' - Degrees in Fahrenheit.
/// 
/// Returns degrees in Celsius
fn fahrenheit_to_celsius(degrees: i32) -> f32
{
    (degrees as f32 - 32.0) * (5.0/9.0)
}

fn celsius_to_fahrenheit(degrees: i32) -> f32
{
    (degrees as f32 * (9.0/5.0)) + 32.0
}

fn display_banner()
{
   
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

fn get_user_input() -> (char,i32){
    println!("Determine degrees in fahrenheit or celsius by entering 'f' or 'c' respectively followed by a space and the degrees.");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
               .expect("Failed to read line");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 
    {
        println!("Invalid input format. Please enter 'f' or 'c' followed by a space and the degrees.");
    }

    let unit = parts[0].chars().next().expect("Expected a character for unit");
    let degrees = parts[1].parse::<i32>().expect("Expected an integer for degrees");
    
    return (unit, degrees)
}