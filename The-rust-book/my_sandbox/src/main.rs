#[derive(Debug)]
enum Light {
    On,
    Off
}

struct 

fn main() {
    {
        println!("The rust book - chapter 6")
        let mut light_state = Light::Off; // default value
        let provided_args: Vec<String> = std::env::args().collect();

        if provided_args.len() == 2 && provided_args[1] == "On" {
            light_state = Light::On;
        }

        match light_state {
            Light::On => println!("The light is currently on!"),
            Light::Off => println!("The light is current off!"),
        }
    }
}
