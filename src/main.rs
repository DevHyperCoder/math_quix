use rand::Rng;
use std::io;

#[derive(Debug)]
enum Operations {
    Add,
    Subtract,
    Multiply,
}

fn main() {
    let operation = get_custom_operation();

    let numbers = get_numbers(1000);

    println!("{} {}", numbers.0, numbers.1);
    println!("{:?}", operation);

    let mut result_str = String::new();

    io::stdin().read_line(&mut result_str).unwrap();

    let result = get_result(result_str);

    if is_correct(operation, numbers, result) {
        println!("Correct!!");
        return;
    }

    eprintln!("Not correct!");
}

fn get_result(result_str: String) -> u32 {
    result_str
        .trim()
        .replace("\n", "")
        .parse::<u32>()
        .expect("not valid duh")
}

fn is_correct(operation: Operations, numbers: (u32, u32), num: u32) -> bool {
    match operation {
        Operations::Add => num == numbers.0 + numbers.1,
        Operations::Multiply => num == numbers.0 * numbers.1,
        Operations::Subtract => num == numbers.0 - numbers.1,
    }
}

fn get_custom_operation() -> Operations {
    let rng = rand::thread_rng().gen_range(0..3);

    match rng {
        0 => Operations::Add,
        1 => Operations::Subtract,
        2 => Operations::Multiply,
        _ => Operations::Add,
    }
}

fn get_numbers(limit: u32) -> (u32, u32) {
    let mut rng = rand::thread_rng();

    (rng.gen_range(0..limit), rng.gen_range(0..limit))
}
