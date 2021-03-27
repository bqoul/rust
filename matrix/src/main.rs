use rand::Rng;
use colored::*;

fn main() {
    loop {
        let num = rand::thread_rng().gen_range(0..2).to_string();
        print!("{}", num
               .on_truecolor(clr(), clr(), clr())
               .truecolor(clr(), clr(), clr()));
    }
}

fn clr() -> u8 {
    let color = rand::thread_rng().gen_range(0..255);
    color
}
