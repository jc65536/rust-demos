const ERR: i32 = -1;
const HELP: i32 = 0;
const PRINT_ARGS: i32 = 1;
const PRINT_FILE: i32 = 2;

fn main() {
    let arglist: Vec<String> = std::env::args().skip(1).collect();
    let mut action = PRINT_ARGS;
    let mut file = "";

    let mut iterator = (&arglist).into_iter();

    while let Some(arg) = iterator.next() {
        if arg.starts_with("-") {
            match arg.as_str() {
                "-h" => action = HELP,
                "-f" => {
                    match iterator.next() {
                        Some(f) => file = f,
                        None => {
                            action = ERR;
                            break;
                        }
                    }
                    action = if action == HELP { HELP } else { PRINT_FILE }
                }
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
        },
        PRINT_FILE => println!("{file}"),
        _ => println!("Error message.")
    }
}
