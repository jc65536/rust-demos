fn main() {
    for arg in std::env::args().skip(1) {
        println!("got arg: {}", arg);
        arg.starts_with("-")
    }
}
