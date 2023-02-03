const ERR: i32 = -1;
const HELP: i32 = 0;
const PRINT_ARGS: i32 = 1;

fn main() {
    let arglist: Vec<String> = std::env::args().skip(1).collect();
    let mut action = PRINT_ARGS;

    for arg in &arglist {
        if arg.starts_with("-") {
            match arg.as_str() {
                "-h" => action = HELP,
                _ => {
                    action = ERR;
                    break;
                }
            }
        }
    }

    match action {
        HELP => println!("Help message."),
        PRINT_ARGS => {
            for arg in arglist {
                println!("{arg}");
            }
        }
        _ => println!("Error message.")
    }
}
