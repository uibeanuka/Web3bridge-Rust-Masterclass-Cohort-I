fn add(a: i32, b: i32) -> i64 {
    (a + b).into()
}

fn subtract(a: i32, b: i32) -> i64 {
    (a - b).into()
}

fn multiply(a: i32, b: i32) -> i64 {
    (a * b).into()
}
fn divide(a: i32, b: i32) -> i64 {
    (a / b).into()
}

fn main() {
    println!("Hello, world!");
    println!("Addition: {}", add(50, 100000));
    println!("Subtraction: {}", subtract(50, 100000));
    println!("Multiplication: {}", multiply(50, 100000));
    println!("Division: {}", divide(50, 100000));
}
