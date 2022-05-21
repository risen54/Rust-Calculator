use std::io;

fn main() {
    loop {
        println!("Enter first number:");
        let mut num1 = String::new();

        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");
        
        println!("Enter second number:");
        let mut num2 = String::new();

        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line");

        println!("Enter operator:");
        let mut op = String::new();
        
        io::stdin()
            .read_line(&mut op)
            .expect("Failed to read line");
        
        let num1: u32 = num1.trim().parse().expect("Please enter a number!");
        let num2: u32 = num2.trim().parse().expect("Please enter a number!");
        let op = op.trim();

        if op == "+"{
            let sum = num1 + num2;
            println!("The sum is: {}", sum);
        } else if op == "-"{
            let differ = num1 - num2;
            println!("The difference is: {}", differ);
        } else if op == "/"{
            let quo = num1 / num2;
            println!("The answer is: {}", quo);
        } else if op == "*"{
            let ans = num1 * num2;
            println!("The answer is: {}", ans);
        } else {
            println!("Invalid operator!")
        }
    }
}
