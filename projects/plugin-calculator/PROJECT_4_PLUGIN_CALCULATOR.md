# Project 4: Plugin Calculator

## Overview
An extensible calculator written in Rust using `traits`, `HashMap<String, Box<dyn Operation>>`,
tokenization, infix-to-postfix conversion with the Shunting Yard algorithm, and postfix evaluation.

## Learning Goals
This project was designed to practice:

- Trait objects (`dyn Trait`)
- Dynamic dispatch
- Plugin-style architecture
- Tokenization and parsing
- Shunting Yard algorithm
- Postfix evaluation
- Custom error handling
- Incremental testing

## Implemented Features

### Basic Operations
- `+`
- `-`
- `*`
- `/`

### Built-in Functions
- `sqrt(x)`
- `sin(x)`
- `cos(x)`

### Tokenization
Supports:
- integer numbers
- decimal numbers
- identifiers
- operators
- parentheses

### Parsing / Evaluation Pipeline
1. `tokenize(input)` -> `Vec<Token>`
2. `to_postfix(tokens)` -> `Vec<Token>`
3. `eval_postfix(postfix, ops)` -> `f64`
4. `evaluate(tokens, ops)` as the final orchestrator

### Error Handling
Custom error type:
- `InvalidArity`
- `UnknownOperation`
- `ParseError`
- `DivisionByZero`
- `InvalidInput`

## Example Expressions

```bash
calc "2 + 3"
# 5
