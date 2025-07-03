use std::io;

fn main() {
    let mut analyze_data = String::new();
    let exit_command = "/c exit".to_string();

    loop {
        println!("Input the string that you want to analyze");
        println!("Type '{exit_command}' to exit\n");

        // took a peek at ../The-rust-book/guessing_game code to get this
        io::stdin()
            .read_line(&mut analyze_data)
            .expect("Failed to read line");

        // sanitising the data (information from https://stackoverflow.com/questions/37888042/remove-single-trailing-newline-from-string-without-cloning)
        if analyze_data.ends_with("\n") {
            analyze_data.pop();
        }

        // check exit
        if analyze_data == exit_command {
            println!("Exiting...");
            break;
        }

        // statistics
        println!("Word statistics:");
        println!("  - Word entered: {}", analyze_data);
        println!("  - Word count: {}\n", analyze_data.trim().len());

        // clear the data
        analyze_data = String::new();
    }
}