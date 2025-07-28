use std::io;

fn main() {
    println!("This is a simple calculator in Rust");
    
    
    println!("Enter a number, then an operator (+, -, *, /, **), then another number.");

    
    let number_1: f64 = loop {
        println!("Please enter the first number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Invalid number. Try again."),
        }
    };

    let operator: String = loop {
        println!("Select operator (+, -, *, /, **):");
       
        let mut op = String::new();
       
        io::stdin().read_line(&mut op).expect("Failed to read line.");
        let op = op.trim();
        
        if ["+", "-", "*", "/", "**"].contains(&op) {
            break op.to_string();
        } else {
            println!("Invalid operator. Try again.");
        }
        
    };

    
    
    let number_2: f64 = loop {
        println!("Please enter the second number:");
        let mut input = String::new();
        
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Invalid number. Try again."),
        }
    };

    
    
    
    let result = match operator.as_str() {
        "+" => number_1 + number_2,
        "-" => number_1 - number_2,
        "*" => number_1 * number_2,
        "/" => {
            if number_2 == 0.0 {
                println!("Error: Division by zero.");
                return;
            }
            number_1 / number_2
        }
        "**" => number_1.powf(number_2),
        _ => {
           

            println!("Unexpected error.");
            return;
        }
    };

    println!("The result is: {}", result);
}

