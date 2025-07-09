fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> f32 {
    a as f32 / b as f32
}

fn main() {
    let a = 10;
    let b = 5;

    println!("Addition: {} + {} = {}", a, b, add(a, b));
    println!("Subtraction: {} - {} = {}", a, b, subtract(a, b));
    println!("Multiplication: {} * {} = {}", a, b, multiply(a, b));
    println!("Division: {} / {} = {}", a, b, divide(a, b));
}
