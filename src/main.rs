use std::env;

fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check if the arguments match what we're looking for.
    // args[0] is the program path. We check args[1] and args[2].
    if args.len() > 2 && args[1] == "--lib" && args[2] == "c_functions" {
        println!("Success! Running logic for 'c_functions'...");
        // TODO: Call your actual library functions here.
    } else {
        // This is the default message if the arguments aren't passed correctly.
        println!("Hello, world!");
        println!("Hint: Try running with 'cargo run -- --lib c_functions'");
    }
}