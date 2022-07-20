#[derive(Copy, Clone, PartialEq)]
pub enum EvaluatorOperatorType {
    MULTIPLY,
    DIVIDE,
    ADD,
    SUBTRACT,
    LBRAC,
    POWER,
}

#[derive(Copy, Clone)]
pub struct EvaluatorOperator {
    pub op_type: EvaluatorOperatorType,
    pub prec: i8,
}

impl EvaluatorOperator {
    pub fn is_operator(c: char) -> bool {
        return vec!['*', '/', '+', '-', '^'].contains(&c);
    }

    pub fn from_char(c: char) -> EvaluatorOperator {
        match c {
            '*' => Option::Some(EvaluatorOperator {
                op_type: EvaluatorOperatorType::MULTIPLY,
                prec: 1,
            }),
            '/' => Option::Some(EvaluatorOperator {
                op_type: EvaluatorOperatorType::DIVIDE,
                prec: 1,
            }),
            '+' => Option::Some(EvaluatorOperator {
                op_type: EvaluatorOperatorType::ADD,
                prec: 0,
            }),
            '-' => Option::Some(EvaluatorOperator {
                op_type: EvaluatorOperatorType::SUBTRACT,
                prec: 0,
            }),
            '(' => Option::Some(EvaluatorOperator {
                op_type: EvaluatorOperatorType::LBRAC,
                prec: 3,
            }),
            '^' => Option::Some(EvaluatorOperator {
                op_type: EvaluatorOperatorType::POWER,
                prec: 2,
            }),
            _ => Option::None
        }.expect("Unknown Operator")
    }
}

pub enum EvaluatorTokenType {
    NUMBER,
    OPERATOR,
}

pub struct EvaluatorToken {
    pub token_type: EvaluatorTokenType,
    operator_value: Option<EvaluatorOperator>,
    number_value: Option<f64>,
}

impl EvaluatorToken {
    pub fn new(token_type: EvaluatorTokenType) -> EvaluatorToken {
        EvaluatorToken {
            token_type,
            operator_value: Option::None,
            number_value: Option::None,
        }
    }

    pub fn new_num_token(value: f64) -> EvaluatorToken {
        let mut token = EvaluatorToken::new(EvaluatorTokenType::NUMBER);
        token.set_num_value(value);
        token
    }

    pub fn new_op_token(value: &EvaluatorOperator) -> EvaluatorToken {
        let mut token = EvaluatorToken::new(EvaluatorTokenType::OPERATOR);
        token.set_op_value(value.clone());
        token
    }

    pub fn set_op_value(&mut self, value: EvaluatorOperator) {
        self.operator_value = Option::Some(value.clone());
    }

    pub fn set_num_value(&mut self, value: f64) {
        self.number_value = Option::Some(value);
    }

    pub fn get_op_value(&self) -> EvaluatorOperator {
        self.operator_value.expect("No operator value present")
    }

    pub fn get_num_value(&self) -> f64 {
        self.number_value.expect("No number value present")
    }
}