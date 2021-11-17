// THIS PACKAGE IS WORK IN PROGRESS, PLEASE DO NOT DELETE FROM AUR

fn main() {
    let args = std::env::args();
    let len = args.len();

    if len == 0 {
        println!("Sweet");
    } else {
        println!("Awesome");
    }
}
