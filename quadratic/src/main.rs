use std::io::{self, Write};

fn main() {
    loop {
        let a = input("a = ");
        let b = input("b = ");
        let c = input("c = ");

        let disc = b * b - 4.0 * a * c; 
        println!("\nD = {}", disc);

        if disc < 0.0 {
            println!("no roots\n");
        } else if disc == 0.0 {
            let x = -b + disc.sqrt() / 2.0 * a;
            println!("x = {}\n", x);
        } else {
            let x1 = (-b + disc.sqrt()) / (2.0 * a);
            let x2 = (-b - disc.sqrt()) / (2.0 * a);
            println!("x1 = {}", x1);
            println!("x2 = {}\n", x2);
        }
    }
}

fn input(message: &str) -> f64 {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(format!("input error ({})", message).as_str());
    let input: f64 = input.trim().parse().expect(format!("parsing error ({})", message).as_str());
    input
}
