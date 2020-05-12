use zipper;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    zipper::files(args[1].to_string());
}
