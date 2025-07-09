fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

fn main() {
    let a = 10.0;
    let b = 5.0;

    println!("Addition: {}", add(a, b));
    println!("Subtraction: {}", subtract(a, b));
    println!("Multiplication: {}", multiply(a, b));

    match divide(a, b) {
        Some(result) => println!("Division: {}", result),
        None => println!("Cannot divide by zero"),
    }
}
