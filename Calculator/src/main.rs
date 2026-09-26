// Maybe eventually update to add sin, cos, and tan
use std::io;

fn main() {
    println!("Enter first number:");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please enter a number");

    println!("Enter an operator (+, -, *, /):");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
    let operator = operator.trim();

    println!("Enter second number:");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please enter a number");

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                panic!("Division by zero");
            }
            num1 / num2
        }
        _ => panic!("Invalid operator"),
    };

    println!("Result: {} {} {} = {}", num1, operator, num2, result);
}
