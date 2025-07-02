# Rust notes 

## Standard

In rust, `std` is the short form of `standard`.
It is in the standard library that we get things like **I/O** and more stuff like that.

## Rand library

We can use the `rand` library to get random numbers.

Using rand:

```rust
    use rand::Rng;
    let secret_number = rand::rng().random_range(1..=100);
```

## Shadowing

Shadowing is the term used when a variable replaces a previously declared variable.

It is not like mutable variables as shadowing allows changing the variable type whereas mutable variables can only get their values changed.

Example:

```rust
let x = 1; // declared a variable
let x = "string"; // redeclared a variable which sends the previous variable named `x` out of scope
```

## Data types

### Floating types

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

### Boolean

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

### Character type

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

### Tuple type

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

### Array type

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```
