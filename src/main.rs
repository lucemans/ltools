fn main() {
    let args = std::env::args();
    let len = args.len();

    if len == 0 {
        println!("Sweet");
    } else {
        println!("Awesome");
    }
}
