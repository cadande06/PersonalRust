
//use std::f64;//module provides constants which are specific to the implementation of the f64 floating point data type
use std::io;

fn main() {
    println!("Quadratic Equation Solver");
    println!("For an equation of the form ax^2 + bx + c = 0");

    // Prompt user for coefficients a, b, and c
    let a = get_coefficient("a");
    let b = get_coefficient("b");
    let c = get_coefficient("c");

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Check the discriminant to determine the nature of the roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The equation has two real roots: x1 = {:.2}, x2 = {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The equation has one real root: x = {:.2}", root);
    } else {
        // When the discriminant is negative, calculate complex roots
        let real_part = -b / (2.0 * a);
        let imaginary_part = (-discriminant).sqrt() / (2.0 * a);
        println!(
            "The equation has two complex roots: x1 = {:.2} + {:.2}i, x2 = {:.2} - {:.2}i",
            real_part, imaginary_part, real_part, imaginary_part
        );//{:.2} specifies that the value hould have two decimal places
    }
}

fn get_coefficient(name: &str) -> f64 {
    loop {
        println!("Enter the coefficient {}: ", name);
        let mut input = String::new();
        
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse::<f64>() /*converts the input from String to f64*/{
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid number for {}", name),
        };
    }
}

