fn main() {
    // s1 owns the value.
    let s1 = String::from("str1");

    // it borrows the value
    // it is an immutable borrow so the original owner stays in scope
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    { // a new temporary scope
        println!("SCOPE TEST 1");
        // borrows s1 as a mutable
        // there can be multiple borrowers but rust allows it
        // as long as all of them are not referenced in a single expression
        let mut s2 = String::from("str2");
        let s3 = &mut s2;

        // the following will throw an error because
        // s3 borrows the value of s2, but when both of them are used
        // at the same time, rust compiler will throw an error.
        // but it is important to mention that this case is only for mutable borrows.
        // the compiler wont throw any error if an owner and a borrowed immutable
        // is used in the expression
        // println!("{s2} {s3}");
    }

    // example of failed code
    {
        println!("SCOPE TEST 2");
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        let r3 = &mut s; // BIG PROBLEM

        println!("{}, {}, and {}", r1, r2, r3);
    }

    {
        println!("SCOPE TEST 3");
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{r1} and {r2}");
        // Variables r1 and r2 will not be used after this point.

        let r3 = &mut s; // no problem
        println!("{r3}");
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
