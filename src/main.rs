fn main() {
    let mut args = std::env::args();

    let file_name = args.nth(1).unwrap_or_else(|| {
        println!("Error: Expected a file");
        std::process::exit(0);
    });

    println!("Command line: {:#?}", file_name);

    let contents = std::fs::read_to_string(file_name).expect("Failed to read file");

    println!("File: {}", contents);
}
