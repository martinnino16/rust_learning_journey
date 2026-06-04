use std::{collections::HashMap, error::Error};

#[derive(Debug, PartialEq)]
pub enum CalcError {
    InvalidArity,
    UnknownOperation(String),
    ParseError(String),
    DivisionByZero,
    InvalidInput,
}

impl std::fmt::Display for CalcError  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcError::InvalidArity => write!(f, "Invalid number of arguments"),
            CalcError::UnknownOperation(op) => write!(f, "Unknown Operation {}", op),
            CalcError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CalcError::DivisionByZero => write!(f, "Division by zero"),
            CalcError::InvalidInput => write!(f, "Invalid input")
        }
    }
}

impl std::error::Error for CalcError{}


trait Operation {
    fn name(&self) -> &str;
    fn execute(&self, args: &[f64]) -> Result<f64, CalcError>;
    fn arity(&self) -> usize;
}

struct Add;
impl Operation for Add {
    fn name(&self) -> &str {
        "add"
    } 

    fn arity(&self) -> usize {
        2
    }

    fn execute(&self, args: &[f64]) -> Result<f64, CalcError> {
        if args.len() != 2 {
            return Err(CalcError::InvalidArity);
        }

        Ok(args[0] + args[1])
    }
}

struct Subtract;
impl Operation for Subtract {
    fn name(&self) -> &str {
        "subtract"
    } 

    fn arity(&self) -> usize {
        2
    }

    fn execute(&self, args: &[f64]) -> Result<f64, CalcError> {
        if args.len() != 2 {
            return Err(CalcError::InvalidArity);
        }

        Ok(args[0] - args[1])
    }
}

struct Multiply;
impl Operation for Multiply {
    fn name(&self) -> &str {
        "multiply"
    } 

    fn arity(&self) -> usize {
        2
    }

    fn execute(&self, args: &[f64]) -> Result<f64, CalcError> {
        if args.len() != 2 {
            return Err(CalcError::InvalidArity);
        }

        Ok(args[0] * args[1])
    }
}

struct Divide;
impl Operation for Divide {
    fn name(&self) -> &str {
        "divide"
    } 

    fn arity(&self) -> usize {
        2
    }

    fn execute(&self, args: &[f64]) -> Result<f64, CalcError> {
        if args.len() != 2 {
            return Err(CalcError::InvalidArity);
        }

        if args[1] == 0.0 {
            return Err(CalcError::DivisionByZero);
        }
        Ok(args[0] / args[1])
    }
}

struct Sqrt;
impl Operation for Sqrt {
    fn name(&self) -> &str {
        "sqrt"
    } 

    fn arity(&self) -> usize {
        1
    }

    fn execute(&self, args: &[f64]) -> Result<f64, CalcError> {
        if args.len() != 1 {
            return Err(CalcError::InvalidArity);
        }

        Ok(args[0].sqrt())
    }
}

struct Sin;
impl Operation for Sin {
    fn name(&self) -> &str {
        "sin"
    } 

    fn arity(&self) -> usize {
        1
    }

    fn execute(&self, args: &[f64]) -> Result<f64, CalcError> {
        if args.len() != 1 {
            return Err(CalcError::InvalidArity);
        }

        Ok(args[0].sin())
    }
}

struct Cos;
impl Operation for Cos {
    fn name(&self) -> &str {
        "cos"
    } 

    fn arity(&self) -> usize {
        1
    }

    fn execute(&self, args: &[f64]) -> Result<f64, CalcError> {
        if args.len() != 1 {
            return Err(CalcError::InvalidArity);
        }

        Ok(args[0].cos())
    }
}

struct Calculator {
    operations: HashMap<String, Box<dyn Operation>>,
}

impl Calculator {

    fn new() -> Self {
        let mut calc = Calculator {
            operations: HashMap::new()
        };

        calc.register(Box::new(Add));
        calc.register(Box::new(Subtract));
        calc.register(Box::new(Multiply));
        calc.register(Box::new(Divide));
        calc.register(Box::new(Sqrt));
        calc.register(Box::new(Sin));
        calc.register(Box::new(Cos));

        calc
    }

    fn register(&mut self, op: Box<dyn Operation>) {
        self.operations.insert(op.name().to_string(), op);
    }

    fn execute_operation(&self, name: &str, args: &[f64]) -> Result<f64, CalcError> {
        let op = self
            .operations
            .get(name)
            .ok_or_else(|| CalcError::UnknownOperation(name.to_string()))?;

        op.execute(args)
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Operator(char),
    Identifier(String),
    LeftParen,
    RightParen
}

fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    //tokenizar input es decir convetir "2 + 2" en un token valido
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&ch) = chars.peek() {
        match ch {
            current if current.is_whitespace() => {
                chars.next().unwrap();
            },
            '+' | '-' | '*' | '/' => {
                result.push(Token::Operator(ch));
                chars.next().unwrap();
            },
            '(' => {
                result.push(Token::LeftParen);
                chars.next().unwrap();
            },
            ')' => {
                result.push(Token::RightParen);
                chars.next().unwrap();
            },
            current if current.is_ascii_digit() => {
                let mut nums = String::new();
                let mut any_point: bool = false;
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        nums.push(chars.next().unwrap());
                    } else if next_ch == '.' {
                        if any_point {
                            return Err(CalcError::ParseError("Error invalid decimal".to_string()));
                        } else {
                            any_point = true;
                            nums.push(chars.next().unwrap());
                        }
                    } else {
                        break;
                    } 
                }
                let num_parse: f64 = nums.parse().or_else(|_| Err(CalcError::ParseError("Error parsing".to_string())))?;
                result.push(Token::Number(num_parse));
            },
            current if current.is_ascii_alphabetic() => {
                let mut identifier = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_alphanumeric() {
                        identifier.push(chars.next().unwrap());
                    } else if next_ch == '(' || next_ch == ')' {
                        break;
                    } else if next_ch == '_' {
                        identifier.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }               
                result.push(Token::Identifier(identifier));
            },
            _ => {
                return Err(CalcError::ParseError(format!("Invalid character: {}", ch)))
            }
        }
    }

    Ok(result)
}

fn precedence(op: char) -> usize {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn to_postfix(tokens: Vec<Token>) -> Result<Vec<Token>, CalcError> {
    let mut output: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(value) => {
                output.push(Token::Number(value));
            }

            Token::Identifier(name) => {
                operator_stack.push(Token::Identifier(name));
            }

            Token::LeftParen => {
                operator_stack.push(Token::LeftParen);
            }

            Token::RightParen => {
                let mut found_left_paren = false;

                while let Some(top) = operator_stack.pop() {
                    match top {
                        Token::LeftParen => {
                            found_left_paren = true;
                            break;
                        }
                        other => output.push(other),
                    }
                }

                if !found_left_paren {
                    return Err(CalcError::ParseError(
                            "Mismatched parentheses: missing '('".to_string(),
                    ));
                }

                if let Some(Token::Identifier(_)) = operator_stack.last() {
                    let func = operator_stack.pop().unwrap();
                    output.push(func);
                }
            }

            Token::Operator(current_op) => {
                while let Some(top) = operator_stack.last() {
                    match top {
                        Token::Operator(top_op)
                            if precedence(*top_op) >= precedence(current_op) =>
                            {
                                let op = operator_stack.pop().unwrap();
                                output.push(op);
                            }
                        _ => break,
                    }
                }

                operator_stack.push(Token::Operator(current_op));
            }
        }
    }

    while let Some(top) = operator_stack.pop() {
        match top {
            Token::LeftParen | Token::RightParen => {
                return Err(CalcError::ParseError(
                        "Mismatched parentheses".to_string(),
                ));
            }
            other => output.push(other),
        }
    }

    Ok(output)
}

fn eval_postfix(tokens: Vec<Token>, ops: &HashMap<String, Box<dyn Operation>>) -> Result<f64, CalcError> {
    let mut value_stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(value) => value_stack.push(value),
            Token::Operator(op) => {
                let operation = match op {
                    '+' => "add",
                    '-' => "subtract",
                    '/' => "divide",
                    '*' => "multiply",
                    _ => return Err(CalcError::ParseError("Unsupported operator".to_string()))
                };
                let operation_from_ops = ops.get(operation).ok_or_else(|| CalcError::UnknownOperation(operation.to_string()))?;
                if value_stack.len() >= 2 {
                    let right_operator = value_stack.pop().unwrap();
                    let left_operator = value_stack.pop().unwrap();
                    let numbers: [f64;2] = [left_operator,right_operator];
                    let args: &[f64] = &numbers;
                    let result = operation_from_ops.execute(args)?;
                    value_stack.push(result);
                } else {
                    return Err(CalcError::ParseError("Not enough operands".to_string()))
                }
            },
            Token::Identifier(identifier) => {
                let operation = ops.get(&identifier).ok_or_else(|| CalcError::UnknownOperation(identifier.to_string()))?;
                let arity = operation.arity();
                let mut numbers = Vec::with_capacity(arity);
                if value_stack.len() >= arity {
                   for _ in 0..arity {
                       let value =  value_stack
                           .pop()
                           .ok_or_else(|| CalcError::ParseError("Not enough operands".to_string()))?;
                       numbers.push(value);
                   } 
                } else {
                    return Err(CalcError::ParseError("Not enough operands".to_string()))
                }
                numbers.reverse();
                let result = operation.execute(&numbers)?;
                value_stack.push(result);
            },
            _ => return Err(CalcError::ParseError("Invalid parse token".to_string()))
        }
    }

    if value_stack.len() == 0 {
       return Err(CalcError::ParseError("Error parsing result".to_string()))
    };

    if value_stack.len() > 1 {
        return Err(CalcError::ParseError("There are more than a result".to_string()))
    };

    match value_stack.len() {
        1 => Ok(value_stack.pop().unwrap()),
        0 => Err(CalcError::ParseError("Error parsing result".to_string())),
        _ => Err(CalcError::ParseError(
            "Expected exactly one result on stack".to_string(),
        )),
    }

}

fn evaluate(tokens: Vec<Token>, ops: &HashMap<String, Box<dyn Operation>>) -> Result<f64, CalcError>{

    let to_postfix = to_postfix(tokens)?;
    let result = eval_postfix(to_postfix, ops)?;
    Ok(result)

}

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {

    let tokens = tokenize(input)?;
    let calc = Calculator::new();
    let result = evaluate(tokens, &calc.operations)?;
    println!("{result}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn add_works() {
        let calc = Calculator::new();
        let result = calc.execute_operation("add", &[2.0, 3.0]);


        assert_eq!(Ok(5.0), result);
    }

    #[test]
    fn unknown_operation_returns_error() {
        let calc = Calculator::new();
        let result = calc.execute_operation("pow", &[2.0, 3.0]);

        assert!(matches!(result, Err(CalcError::UnknownOperation(_))));
    }

    #[test]
    fn invalid_arity_returns_error() {
        let calc = Calculator::new();
        let result = calc.execute_operation("add", &[2.0]);

        assert_eq!(Err(CalcError::InvalidArity), result);
    }


    #[test]
    fn divide_by_zero_returns_error() {
        let calc = Calculator::new();
        let result = calc.execute_operation("divide", &[2.0, 0.0]);

        assert_eq!(Err(CalcError::DivisionByZero), result);
    }

    #[test]
    fn sqrt_works() {
        let calc = Calculator::new();
        let result = calc.execute_operation("sqrt", &[16.0]);

        assert_eq!(Ok(4.0), result);
    }

    #[test]
    fn sin_works() {
        let calc = Calculator::new();
        let result = calc.execute_operation("sin", &[0.0]);

        assert_eq!(Ok(0.0), result);
    }

    #[test]
    fn cos_works() {
        let calc = Calculator::new();
        let result = calc.execute_operation("cos", &[0.0]);

        assert_eq!(Ok(1.0), result);
    }

    #[test]
    fn tokenizes_single_integer() {
        let single_integer = "2";
        let result = tokenize(single_integer).expect("tokenize should succeed");

        assert_eq!(1, result.len());
        assert_eq!(Token::Number(2.0), result[0]);
    }


    #[test]
    fn tokenizes_multiple_digit_integer() {
        let multiple_digit_integer = "123";
        let result = tokenize(multiple_digit_integer).expect("tokenize should succeed");

        assert_eq!(1, result.len());
        assert_eq!(Token::Number(123.0), result[0]);
    }


    #[test]
    fn tokenizes_decimal_number() {
        let decimal_number = "12.3";
        let result = tokenize(decimal_number).expect("tokenize should succeed");

        assert_eq!(1, result.len());
        assert_eq!(Token::Number(12.3), result[0]);
    }

    #[test]
    fn tokenizes_simple_addition() {
        let simple_addition = "2 + 2";
        let result = tokenize(simple_addition).expect("tokenize should succeed");

        assert_eq!(3, result.len());
        assert_eq!(Token::Number(2.0), result[0]);
        assert_eq!(Token::Operator('+'), result[1]);
        assert_eq!(Token::Number(2.0), result[2]);
    }

    #[test]
    fn tokenizes_expression_with_spaces() {
        let expression_with_spaces = "2  +  2";
        let result = tokenize(expression_with_spaces).expect("tokenize should succeed");

        assert_eq!(3, result.len());
        assert_eq!(Token::Number(2.0), result[0]);
        assert_eq!(Token::Operator('+'), result[1]);
        assert_eq!(Token::Number(2.0), result[2]);
    }


    #[test]
    fn tokenizes_expression_with_parentheses() {
        let expression_with_parentheses = "(";
        let result = tokenize(expression_with_parentheses).expect("tokenize should succeed");

        assert_eq!(1, result.len());
        assert_eq!(Token::LeftParen, result[0]);

    }

    #[test]
    fn tokenizes_builtin_sqrt_function_call() {
        let sqrt_function = "sqrt(14)";
        let result = tokenize(sqrt_function).expect("tokenize should succeed");

        assert_eq!(4, result.len());
        assert_eq!(Token::Identifier("sqrt".to_string()), result[0]);
        assert_eq!(Token::LeftParen, result[1]);
        assert_eq!(Token::Number(14.0), result[2]);
        assert_eq!(Token::RightParen, result[3]);
    }

    #[test]
    fn tokenizes_builtin_sin_function_call() {
        let result = tokenize("sin(0)").expect("tokenize should succeed");

        assert_eq!(
            vec![
            Token::Identifier("sin".to_string()),
            Token::LeftParen,
            Token::Number(0.0),
            Token::RightParen,
            ],
            result
        );
    }

    #[test]
    fn tokenizes_identifier_with_underscore() {
        let result = tokenize("custom_op(5)").expect("tokenize should succeed");

        assert_eq!(
            vec![
            Token::Identifier("custom_op".to_string()),
            Token::LeftParen,
            Token::Number(5.0),
            Token::RightParen,
            ],
            result
        );
    }

    #[test]
    fn tokenizes_identifier_with_digits() {
        let result = tokenize("custom1(5)").expect("tokenize should succeed");

        assert_eq!(
            vec![
            Token::Identifier("custom1".to_string()),
            Token::LeftParen,
            Token::Number(5.0),
            Token::RightParen,
            ],
            result
        );
    }

    #[test]
    fn tokenizes_trailing_dot_decimal() {
        let result = tokenize("5.").expect("tokenize should succeed");

        assert_eq!(vec![Token::Number(5.0)], result);
    }

    #[test]
    fn rejects_leading_dot_decimal() {
        let result = tokenize(".5");

        assert!(matches!(result, Err(CalcError::ParseError(_))));
    }

    #[test]
    fn rejects_multiple_decimal_points() {
        let result = tokenize("12.3.4");

        assert!(matches!(result, Err(CalcError::ParseError(_))));
    }

    #[test]
    fn rejects_double_dot_number() {
        let result = tokenize("1..2");

        assert!(matches!(result, Err(CalcError::ParseError(_))));
    }

    #[test]
    fn rejects_invalid_character() {
        let result = tokenize("2 @ 3");

        assert!(matches!(result, Err(CalcError::ParseError(_))));
    }

    #[test]
    fn rejects_unsupported_operator() {
        let result = tokenize("2 ^ 3");

        assert!(matches!(result, Err(CalcError::ParseError(_))));
    }

    #[test]
    fn tokenizes_empty_function_call_without_lexical_error() {
        let result = tokenize("sqrt()").expect("tokenize should succeed");

        assert_eq!(
            vec![
            Token::Identifier("sqrt".to_string()),
            Token::LeftParen,
            Token::RightParen,
            ],
            result
        );
    }

    #[test]
    fn simple_pattern() {
        let pattern = "2 + 3";
        let calc = Calculator::new();
        let tokens = tokenize(pattern).expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations).expect("evaluator should succeed");

        assert_eq!(5.0, result);
    }

    #[test]
    fn simple_divide_pattern() {
        let pattern = "10 / 2";
        let calc = Calculator::new();
        let tokens = tokenize(pattern).expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations).expect("evaluator should succeed");

        assert_eq!(5.0, result); 
    }

    #[test]
    fn simple_multiply_pattern() {
        let pattern = "4 * 5";
        let calc = Calculator::new();
        let tokens = tokenize(pattern).expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations).expect("evaluator should succeed");

        assert_eq!(20.0, result);  
    }

    #[test]
    fn identifier_pattern() {
        let pattern = "sqrt(16)";
        let calc = Calculator::new();
        let tokens = tokenize(pattern).expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations).expect("evaluator should succeed");

        assert_eq!(4.0, result);  

    }

    #[test]
    fn division_by_zero_pattern(){
        let pattern = "10 / 0";
        let calc = Calculator::new();
        let tokens = tokenize(pattern).expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations);

        assert!(matches!(result, Err(CalcError::DivisionByZero)));   
    }

    #[test]
    fn unknown_operation_pattern() {
        let pattern = "pow(2)";
        let calc = Calculator::new();
        let tokens = tokenize(pattern).expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations);

        assert_eq!(
            Err(CalcError::UnknownOperation("pow".to_string())),
            result
        );
    }

    #[test]
    fn parse_error_pattern() {
        let pattern = "2 + * 3";
        let calc = Calculator::new();
        let tokens = tokenize(pattern).expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations);

        assert_eq!(
            Err(CalcError::ParseError("Pattern not allow".to_string())),
            result
        );

    }

    #[test]
    fn evaluate_sin_pattern() {
        let pattern = "sin(0)";
        let calc = Calculator::new();
        let tokens = tokenize(pattern).expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations).expect("evaluator should succeed");

        assert_eq!(0.0, result);  
    }

    #[test]
    fn evaluate_cos_pattern() {
        let pattern = "cos(0)";
        let calc = Calculator::new();
        let tokens = tokenize(pattern).expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations).expect("evaluator should succeed");

        assert_eq!(1.0, result);  
    } 


    #[test]
    fn to_postfix_add() {
        let infix_add = "2 + 3";
        let tokens = tokenize(infix_add).expect("tokenize should succeed");
        let postfix = to_postfix(tokens).expect("postfix process should succeed");

        assert_eq!(
           vec![Token::Number(2.0), Token::Number(3.0), Token::Operator('+')],
           postfix
        )
    }

    #[test]
    fn to_postfix_multiply() {
        let infix_multiply = "4 * 5";
        let tokens = tokenize(infix_multiply).expect("tokenize should succeed");
        let postfix = to_postfix(tokens).expect("postfix process should succeed");

        assert_eq!(
           vec![Token::Number(4.0), Token::Number(5.0), Token::Operator('*')],
           postfix
        )
    }

    #[test]
    fn to_postfix_precedence() {
        let infix_precedence = "2 + 3 * 4";
        let tokens = tokenize(infix_precedence).expect("tokenize should succeed");
        let postfix = to_postfix(tokens).expect("postfix process should succeed");

        assert_eq!(
           vec![Token::Number(2.0), Token::Number(3.0),Token::Number(4.0), Token::Operator('*'), Token::Operator('+')],
           postfix
        )
 
    }

    #[test]
    fn to_postfix_parentheses() {
        let infix_parentheses = "(2 + 3) * 4";
        let tokens = tokenize(infix_parentheses).expect("tokenize should succeed");
        let postfix = to_postfix(tokens).expect("postfix process should succeed");

        assert_eq!(
           vec![Token::Number(2.0), Token::Number(3.0),Token::Operator('+'), Token::Number(4.0), Token::Operator('*')],
           postfix
        )
 
    }

    #[test]
    fn to_postfix_function_call() {
        let infix_function_call = "sqrt(16)";
        let tokens = tokenize(infix_function_call).expect("tokenize should succeed");
        let postfix = to_postfix(tokens).expect("postfix process should succeed");

        assert_eq!(
           vec![Token::Number(16.0), Token::Identifier("sqrt".to_string())],
           postfix
        )
 
    }
    #[test]
    fn evaluates_operator_precedence() {
        let calc = Calculator::new();
        let tokens = tokenize("2 + 3 * 4").expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations).expect("evaluate should succeed");

        assert_eq!(14.0, result);
    }
    #[test]
    fn evaluates_parenthesized_expression() {
        let calc = Calculator::new();
        let tokens = tokenize("(2 + 3) * 4").expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations).expect("evaluate should succeed");

        assert_eq!(20.0, result);
    }

    #[test]
    fn evaluates_nested_parentheses_expression() {
        let calc = Calculator::new();
        let tokens = tokenize("10 * (5 + 3)").expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations).expect("evaluate should succeed");

        assert_eq!(80.0, result);
    }

    #[test]
    fn evaluates_sqrt_expression() {
        let calc = Calculator::new();
        let tokens = tokenize("sqrt(16)").expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations).expect("evaluate should succeed");

        assert_eq!(4.0, result);
    }

    #[test]
    fn evaluates_sin_expression() {
        let calc = Calculator::new();
        let tokens = tokenize("sin(0)").expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations).expect("evaluate should succeed");

        assert_eq!(0.0, result);
    }

    #[test]
    fn evaluates_cos_expression() {
        let calc = Calculator::new();
        let tokens = tokenize("cos(0)").expect("tokenize should succeed");
        let result = evaluate(tokens, &calc.operations).expect("evaluate should succeed");

        assert_eq!(1.0, result);
    }
}

