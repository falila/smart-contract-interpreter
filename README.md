# Smart Contract Interpreter

A minimalistic interpreter for a simple smart contract written in Solidity.
Solidity language documentation [here](https://docs.soliditylang.org/en/v0.8.26/)
## Features

- **Variable Declaration:** Declare and initialize variables.
- **Arithmetic Operations:** Support for basic arithmetic operations.
- **Conditional Statements:** Basic if-else conditions.
- **While Loops:** Looping with while conditions.
- **Function Calls:** Calling predefined functions (e.g., `print`).

## Language Syntax

- **Variable Declaration:** `let <var> = <value>;`
- **Arithmetic Operation:** `<var> = <var> + <value>;`
- **Conditional Statement:**
  ```plaintext
  if <var> == <value> {
      // statements
  } else {
      // statements
  }
  
  let x = 0;
  while x < 5 {
      x = x + 1;
      print(x);
  }

### Build the project:

```sh
cargo build
```
### Run the project:

```sh
cargo run
```

## Usage 

```code
fn main() {
    let mut interpreter = Interpreter::new();

    let code = r#"
        let x = 0;
        while x < 5 {
            x = x + 1;
            print(x);
        }
    "#;

    let statements = interpreter.parse(code);
    interpreter.evaluate(statements);
}

```
