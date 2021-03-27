use colored::*;
use rand::Rng;
use std::io::{self, Write};

fn main() {
    let min = input(String::from("range (min): "));
    let max = input(String::from("range (max): ")) + 1;

    loop {
        let num_1 = rand::thread_rng().gen_range(min..max);
        let num_2 = rand::thread_rng().gen_range(min..max); 
        
        let answer;
        let user_input;

        let operation = rand::thread_rng().gen_range(1..5);

        if operation == 1 {
            user_input = input(format!("\n{} • {} = ", bracers(num_1), bracers(num_2)));
            answer = num_1 * num_2; 
        } else if operation == 2 {
            answer = num_1 - num_2;
            user_input = input(format!("\n{} - {} = ", bracers(num_1), bracers(num_2)));
        } else if operation == 3 && num_2 != 0 && num_1 % num_2 == 0 {
            answer = num_1 / num_2;
            user_input = input(format!("\n{} : {} = ", bracers(num_1), bracers(num_2)));
        } else {
            answer = num_1 + num_2;
            user_input = input(format!("\n{} + {} = ", bracers(num_1), bracers(num_2)));
        }

        if user_input == answer {
            println!("{}", "correct!".black().on_green());
        } else {
            println!("{}", "wrong!".black().on_red());
        }
    }
}

fn input(message: String) -> i32 {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");

    if input.trim() == "" {
        input = String::from("123456789");
    }

    let input: i32 = input.trim().parse().expect("parsing error");
    input
}

fn bracers(number: i32) -> String {
    let mut string = number.to_string();
    if number < 0 {
        string = format!("({})", string);
    }
    string
}
