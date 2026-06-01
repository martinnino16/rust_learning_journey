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

fn evaluate(tokens: Vec<Token>, ops: &HashMap<String, Box<dyn Operation>>) -> Result<f64, CalcError>{
    //evaluar tokens
    //recorrer los tokens
    //si el token es un numero obtener el numero
    //si el token es operador parsear el operador por el nombre de la operacion
    //ejecutar operacion atravez de las operaciones del hashmap
    //devolver el resultado o error (ok) error

    let result = match tokens.as_slice() {
        [Token::Number(first_value), Token::Operator(operator), Token::Number(second_value)] => {
            let array_numbers: [f64; 2] = [*first_value,*second_value];
            let args: &[f64] = &array_numbers; 
            let op = match operator {
                '+' => "add",
                '-' => "subtract",
                '/' => "divide",
                '*' => "multiply",
                _ => return Err(CalcError::UnknownOperation("Unknown Operation".to_string()))
            };
            let operation = ops.get(op).ok_or_else(|| CalcError::UnknownOperation(op.to_string()))?;
            let result = operation.execute(args)?;
            result
        },
        [Token::Identifier(identifier), Token::LeftParen, Token::Number(value), Token::RightParen] => {
            let array_numbers: [f64; 1] = [*value];
            let args: &[f64] = &array_numbers;
            let operation = ops.get(identifier).ok_or_else(|| CalcError::UnknownOperation(identifier.to_string()))?;
            let result = operation.execute(args)?;
            result
        },
        _ => return Err(CalcError::ParseError("Pattern not allow".to_string())) 
    };
    Ok(result)

}

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {

    let tokens = tokenize(input)?;
    let calc = Calculator::new();
    let result = evaluate(tokens, &calc.operations )?;
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
    fn tokenizes_syntactically_invalid_expression_without_lexical_error() {
        let result = tokenize("2 + * 3").expect("tokenize should succeed");

        assert_eq!(
            vec![
            Token::Number(2.0),
            Token::Operator('+'),
            Token::Operator('*'),
            Token::Number(3.0),
            ],
            result
        );
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
}

