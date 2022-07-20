use std::collections::LinkedList;
use crate::evaluator::evaluator_error::EvaluatorError;
use crate::evaluator::evaluator_state::EvaluatorState;
use crate::evaluator::evaluator_token::{EvaluatorOperator, EvaluatorOperatorType, EvaluatorToken, EvaluatorTokenType};

fn run_operator(calc: &mut LinkedList::<f64>, tok: &EvaluatorOperator) -> f64 {
    let num_1 = calc.pop_front().expect("Expected stack not to be empty");
    let num_2 = calc.pop_front().expect("Expected stack not to be empty");

    match tok.op_type {
        EvaluatorOperatorType::MULTIPLY => num_2 * num_1,
        EvaluatorOperatorType::DIVIDE => num_2 / num_1,
        EvaluatorOperatorType::ADD => num_2 + num_1,
        EvaluatorOperatorType::SUBTRACT => num_2 - num_1,
        EvaluatorOperatorType::POWER => num_2.powf(num_1),
        _ => { 1f64 }
    }
}

fn run_state_eval(state: &mut EvaluatorState) -> Option<f64> {
    let mut calc = LinkedList::<f64>::new();

    loop {
        let wrapped_tok = state.pop_output_queue();

        if wrapped_tok.is_none() {
            break;
        }

        let tok = wrapped_tok.unwrap();

        match tok.token_type {
            EvaluatorTokenType::NUMBER => calc.push_front(tok.get_num_value()),
            EvaluatorTokenType::OPERATOR => {
                let num = run_operator(&mut calc, &tok.get_op_value());
                calc.push_front(num)
            }
        }
    }

    calc.pop_front()
}

fn handle_num(state: &mut EvaluatorState) -> Result<(), EvaluatorError> {
    let mut num = String::from(state.tok);

    while state.increment() {
        if !state.tok.is_numeric() && state.tok != '.' {
            state.decrement();
            break;
        }

        num.push(state.tok)
    }

    let parsed_num = num.parse::<f64>().expect(format!("Failed to parse number expression {}", num).as_str());
    state.push_output_queue(EvaluatorToken::new_num_token(parsed_num));

    Result::Ok(())
}

fn handle_op(state: &mut EvaluatorState) -> Result<(), EvaluatorError> {
    let op = EvaluatorOperator::from_char(state.tok);
    state.pop_stack_to_output_while(|o| o.prec >= op.prec && o.op_type != EvaluatorOperatorType::LBRAC);
    state.push_op_stack(&op);

    Result::Ok(())
}

fn handle_closing_bracket(state: &mut EvaluatorState) -> Result<(), EvaluatorError> {
    state.pop_stack_to_output_while(|op| op.op_type != EvaluatorOperatorType::LBRAC);
    state.pop_op_stack();

    Result::Ok(())
}

pub fn evaluate(input: &String) -> Option<f64> {
    let mut state = EvaluatorState::new(input.clone());

    while state.increment() {
        let res = match state.tok {
            tok if tok.is_numeric() => handle_num(&mut state),
            '(' => {
                state.push_op_stack(&EvaluatorOperator::from_char('('));
                Result::Ok(())
            }
            ')' => handle_closing_bracket(&mut state),
            tok if EvaluatorOperator::is_operator(tok) => handle_op(&mut state),
            _ => { Result::Ok(()) }
        };

        if res.is_err() {
            break;
        }
    }

    state.pop_stack_to_output_while(|_| true);
    run_state_eval(&mut state)
}