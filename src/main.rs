mod evaluator;

fn main() {
    println!("Welcome to Evaluator, please enter the expression you'd like to evaluate: ");

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).expect("Failed to read expression");
    input_string = input_string.split_whitespace().collect();

    let result = evaluator::evaluator::evaluate(&input_string).expect("Failed to parse expression");

    println!("{} resulted in {}", input_string, result);
}
