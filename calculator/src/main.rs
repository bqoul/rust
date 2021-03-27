use colored::*;

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("input error");

        if input.trim() == "" {
            input = String::from("a"); //meval drops an error if input = "" or " "
        }

        match meval::eval_str(input) {
            Ok(result) => println!("{}\n", result.to_string().green()),
            Err(_) => println!("{}\n", "cant count this".red())
        }
    }
}
