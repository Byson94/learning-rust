fn main() {
    println!("Hello, world!");

    let multV1 = 5;
    let multV2 = 3;
    println!("{multV1} times {multV2}: {}", multiply(multV1, multV2));
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}