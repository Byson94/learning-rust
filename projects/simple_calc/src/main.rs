use std::io;

struct Commands {
    exit: String,
    compute: String,
}

fn main() {
    println!("Welcome to simple calc");

    let valid_commands = Commands {
        exit: "/exit".to_string(),
        compute: "/cmpt".to_string(),
    };

    loop {
        let mut user_command = String::new();

        io::stdin()
            .read_line(&mut user_command)
            .expect("Unable to read command line");

        // sanitizing user_command
        if user_command.ends_with("\n") {
            user_command.pop();
        }

        // shadowing user_command and split the whitespaces
        let mut user_command: Vec<&str> = user_command.split_whitespace().collect();

        if user_command[0] == valid_commands.exit {
            break;
        }

        if user_command.len() == 2 && user_command[0] == valid_commands.compute {
            compute(user_command[1]);
        } else {
            println!("Invalid command")
        }
    }
}

fn compute(compt_string: &str) -> &str {
    println!("{}", compt_string);

    compt_string
}