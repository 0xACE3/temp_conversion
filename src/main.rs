use colored::*;
use std::io;
fn main() {
    loop {
        let mut response = String::new();

        println!(
            "\n{}\t{}",
            "(1).Celsius To Fahrenheit.".bright_cyan(),
            "(2).Fahrenheit To Celsius".bright_cyan(),
        );

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        let response: u32 = match response.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number");
                continue;
            }
        };
        if response >= 2 {
            let num = ask_input();
            let r = fahrenheit_to_celsius(num);
            println!("\n{}°C", r);
            break;
        } else {
            let num = ask_input();
            let r = celsius_to_fahrenheit(num);
            println!("\n{}°F", r);
            break;
        }
    }
}

fn fahrenheit_to_celsius(far: i32) -> i32 {
    (far - 32) * 5 / 9
}

fn celsius_to_fahrenheit(cel: i32) -> i32 {
    (cel * 9 / 5) + 32
}

fn ask_input() -> i32 {
    let mut number = String::new();
    println!("{}", "\nEnter a number: ".purple());
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number.trim().parse().expect("Enter a number!");

    number
}
