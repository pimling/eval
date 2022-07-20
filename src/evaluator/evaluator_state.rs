use std::collections::LinkedList;
use crate::evaluator::evaluator_token::{EvaluatorOperator, EvaluatorToken};

pub struct EvaluatorState {
    pub tok: char,
    expr: String,
    i: usize,
    pub output_queue: LinkedList<EvaluatorToken>,
    pub op_stack: LinkedList<EvaluatorOperator>,
}

impl EvaluatorState {
    pub fn new(expr: String) -> EvaluatorState {
        EvaluatorState {
            expr,
            i: 0usize,
            tok: 0 as char,
            output_queue: LinkedList::<EvaluatorToken>::new(),
            op_stack: LinkedList::<EvaluatorOperator>::new(),
        }
    }

    pub fn push_output_queue(&mut self, num: EvaluatorToken) {
        self.output_queue.push_back(num);
    }

    pub fn pop_output_queue(&mut self) -> Option<EvaluatorToken> {
        self.output_queue.pop_front()
    }

    pub fn push_op_stack(&mut self, op: &EvaluatorOperator) {
        self.op_stack.push_front(op.clone());
    }

    pub fn peek_op_stack(&mut self) -> Option<&EvaluatorOperator> {
        self.op_stack.front()
    }

    pub fn pop_op_stack(&mut self) -> Option<EvaluatorOperator> {
        self.op_stack.pop_front()
    }

    pub fn pop_stack_to_output_while<F: Fn(&EvaluatorOperator) -> bool>(&mut self, pred: F) {
        loop {
            let cur_op = self.peek_op_stack();

            if cur_op.is_none() {
                break;
            }

            if pred(&cur_op.unwrap()) {
                let pop_op = self.pop_op_stack().expect("Expected valid operator on stack");
                self.push_output_queue(EvaluatorToken::new_op_token(&pop_op))
            } else {
                break;
            }
        }
    }

    pub fn increment(&mut self) -> bool {
        if self.i >= self.expr.len() {
            return false;
        }

        self.tok = self.expr.chars().nth(self.i).expect("Failed to get nth char");
        self.i += 1;

        true
    }

    pub fn decrement(&mut self) -> bool {
        if self.i <= 0 {
            return false;
        }

        self.i -= 1;
        self.tok = self.expr.chars().nth(self.i).expect("Failed to get nth char");

        true
    }
}