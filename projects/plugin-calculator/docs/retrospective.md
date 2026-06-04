# Retrospective - Project 4: Plugin Calculator

## Project Status
Project completed at a solid functional level.

### Completed goals
- [x] Basic operations (`+`, `-`, `*`, `/`)
- [x] Parentheses and operator precedence
- [x] At least 3 built-in functions (`sqrt`, `sin`, `cos`)
- [x] Plugin-style operation registry with `HashMap<String, Box<dyn Operation>>`
- [x] Tokenization of expressions
- [x] Infix to postfix conversion using the Shunting Yard algorithm
- [x] Postfix evaluation
- [x] Relevant tests for core logic, tokenizer, postfix conversion, and evaluation

---

## What went well
- The core architecture was relatively easy to build, especially with the guide as a reference.
- Separating the solution into stages made the project much easier to reason about:
  - tokenization
  - infix to postfix conversion
  - postfix evaluation
- Implementing the tokenizer was one of the most satisfying parts of the project.
- Using a plugin-style registry with `traits` and `HashMap<String, Box<dyn Operation>>` made the design feel extensible and clean.
- Adding new functions like `sin` and `cos` became straightforward once the architecture was in place.
- Writing tests incrementally helped validate each stage of the project.

---

## What was difficult
- The hardest part was not Rust syntax itself, but defining the logic and structure of the solution.
- I often needed help to understand how to think about the problem step by step.
- Implementing the tokenizer required careful reasoning about:
  - `peek()` vs `next()`
  - loops
  - token boundaries
- The Shunting Yard algorithm was conceptually harder than the initial core setup.
- Postfix evaluation required a better understanding of stacks and operand order.
- Error handling became more complex once the project moved beyond the initial MVP.

---

## What I learned
This project helped me learn a lot about Rust and software design.

### Rust concepts reinforced
- loops in different forms:
  - `for`
  - `while`
  - `while let`
- iterators and `peekable()`
- `HashMap` usage
- pattern matching with `match`
- borrowing and references
- custom error types with `Result`

### Software engineering concepts reinforced
- building software incrementally
- separating parsing and evaluation
- designing around clear phases
- using traits for extensibility
- using stacks in parsing algorithms
- writing tests to support refactoring

---

## What I want to improve
The main thing I want to improve is my problem-solving process.

More specifically, I want to become better at:

- structuring a solution on my own before coding
- identifying the minimum viable version of a problem
- breaking a complex feature into smaller steps
- deciding implementation order with more confidence
- depending less on external guidance for architectural thinking

I do not think this means I failed.  
It means I still need to strengthen my reasoning process, especially when solving new problems from scratch.

---

## What I would do differently next time
- I would define the grammar and scope earlier before implementing the tokenizer.
- I would write the expected pipeline earlier:
  - tokenize
  - convert to postfix
  - evaluate postfix
- I would add more end-to-end tests earlier in the project.
- I would spend more time writing down the algorithm in plain language before coding.
- I would be more explicit about what the MVP supports and what is intentionally postponed.

---

## Design decisions
- Used `HashMap<String, Box<dyn Operation>>` to register operations dynamically.
- Modeled functions as `Identifier(String)` tokens.
- Used the Shunting Yard algorithm to support precedence and parentheses.
- Used postfix evaluation with a value stack.
- Chose to accept `5.` as a valid decimal format.
- Chose to reject `.5` as an invalid decimal format.

---

## Limitations
This version does not support:
- unary minus
- variables
- multi-argument functions
- dynamic plugin loading (`.so` / `.dll`)
- REPL mode
- history
- advanced error diagnostics

---

## Future improvements
Possible next steps:
- add `log(x)`
- add `--list-functions`
- support unary minus
- support multi-argument functions
- improve error messages
- add REPL mode
- support dynamic plugins

---

## Personal conclusion
This project taught me much more than just Rust syntax.

It showed me that building software is not only about writing code, but also about:

- structuring the problem
- separating responsibilities
- building in stages
- testing continuously
- improving the way I think

The core part of the project felt easier because I had guidance, but the later stages forced me to reason much more deeply.  
That was difficult, but also one of the most valuable parts of the experience.

I also feel that I learned a lot of Rust through this project, especially around:
- loops
- iterators
- `HashMap`
- `match`
- borrowing

Overall, this project helped me grow both in Rust and in problem-solving.
