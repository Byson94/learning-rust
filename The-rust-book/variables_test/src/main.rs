fn main() {

    // the rust book: Chap 3.1
    // > introduction
    {
        println!("The rust book, chapter 3.1 (introduction)");
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }

    // the rust book: Chap 3.1
    // > constant variables
    {
        println!("\n The rust book, chapter 3.1 (variables)");
        // The use of const is to store variables that will never change
        // `let` can be shadowed later on but const cannot
        // const is defined during compile time and let is only set during run-time
        const RUST_REQUIRES_CONST_TO_BE_IN_ALL_CAPS: u32 = 60 * 60 * 3;
        println!("{RUST_REQUIRES_CONST_TO_BE_IN_ALL_CAPS}");
    }

    // the rust book: Chap 3.1
    // > shadowing
    {
        // shadowing is a recommended topic in rust
        // unlike other languages, it is good to shadow in rust
        println!("\n The rust book, chapter 3.1 (shadowing)")
        let x = 5;
        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }

        println!("The value of x is: {x}");
    }
}
