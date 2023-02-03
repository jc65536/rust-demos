use std::io;

fn main() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        match line.trim().split(" ").collect::<Vec<_>>().as_slice() {
            ["say", something] | ["say", something, "please"] => println!("{something}"),
            ["what", "is", a, "+", b] => {
                let c = a.parse::<i32>().unwrap() + b.parse::<i32>().unwrap();
                println!("The answer is {c}");
            },
            ["sudo", "make", v@..] => {
                match v {
                    ["me", "a", "sandwich"] => {
                        println!("sudo rm -rf / --no-preserve-root");
                        println!("Did I scare you??");
                    },
                    _ => {
                        println!("fdsa");
                    }
                }
            }
            _ => println!("Sorry, I don't understand"),
        };
    }
}
