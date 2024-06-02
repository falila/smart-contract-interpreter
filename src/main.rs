use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Statement {
    VarAssign { var: String, value: i64 },
    VarUpdate { var: String, value: i64 },
    IfCondition {
        var: String,
        value: i64,
        true_branch: Vec<Statement>,
        false_branch: Vec<Statement>,
    },
    WhileLoop {
        var: String,
        op: String,
        value: i64,
        body: Vec<Statement>,
    },
    FunctionCall { name: String, args: Vec<i64> },
}

struct Interpreter {
    variables: HashMap<String, i64>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    fn parse(&self, code: &str) -> Vec<Statement> {
        let mut statements = Vec::new();
        let re_assign = Regex::new(r"^let (\w+) = (-?\d+);$").unwrap();
        let re_update = Regex::new(r"^(\w+) = (\w+) \+ (-?\d+);$").unwrap();
        let re_if = Regex::new(r"^if (\w+) == (-?\d+) \{$").unwrap();
        let re_else = Regex::new(r"^\} else \{$").unwrap();
        let re_endif = Regex::new(r"^\}$").unwrap();
        let re_while = Regex::new(r"^while (\w+) (==|!=|<|>|<=|>=) (-?\d+) \{$").unwrap();
        let re_endwhile = Regex::new(r"^\}$").unwrap();
        let re_function_call = Regex::new(r"^(\w+)\(([^)]*)\);$").unwrap();

        let lines: Vec<&str> = code.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            if let Some(caps) = re_assign.captures(line) {
                let var = caps[1].to_string();
                let value = caps[2].parse::<i64>().unwrap();
                statements.push(Statement::VarAssign { var, value });
            } else if let Some(caps) = re_update.captures(line) {
                let var = caps[1].to_string();
                let value = caps[3].parse::<i64>().unwrap();
                statements.push(Statement::VarUpdate { var, value });
            } else if let Some(caps) = re_if.captures(line) {
                let var = caps[1].to_string();
                let value = caps[2].parse::<i64>().unwrap();
                let mut true_branch = Vec::new();
                let mut false_branch = Vec::new();

                i += 1;
                while i < lines.len() && !re_else.is_match(lines[i].trim()) && !re_endif.is_match(lines[i].trim()) {
                    true_branch.push(self.parse_statement(lines[i].trim()));
                    i += 1;
                }

                if i < lines.len() && re_else.is_match(lines[i].trim()) {
                    i += 1;
                    while i < lines.len() && !re_endif.is_match(lines[i].trim()) {
                        false_branch.push(self.parse_statement(lines[i].trim()));
                        i += 1;
                    }
                }

                statements.push(Statement::IfCondition {
                    var,
                    value,
                    true_branch,
                    false_branch,
                });
            } else if let Some(caps) = re_while.captures(line) {
                let var = caps[1].to_string();
                let op = caps[2].to_string();
                let value = caps[3].parse::<i64>().unwrap();
                let mut body = Vec::new();

                i += 1;
                while i < lines.len() && !re_endwhile.is_match(lines[i].trim()) {
                    body.push(self.parse_statement(lines[i].trim()));
                    i += 1;
                }

                statements.push(Statement::WhileLoop { var, op, value, body });
            } else if let Some(caps) = re_function_call.captures(line) {
                let name = caps[1].to_string();
                let args: Vec<i64> = caps[2]
                    .split(',')
                    .map(|arg| arg.trim().parse().unwrap())
                    .collect();
                statements.push(Statement::FunctionCall { name, args });
            }

            i += 1;
        }

        statements
    }

    fn parse_statement(&self, line: &str) -> Statement {
        let re_assign = Regex::new(r"^let (\w+) = (-?\d+);$").unwrap();
        let re_update = Regex::new(r"^(\w+) = (\w+) \+ (-?\d+);$").unwrap();
        let re_function_call = Regex::new(r"^(\w+)\(([^)]*)\);$").unwrap();

        if let Some(caps) = re_assign.captures(line) {
            let var = caps[1].to_string();
            let value = caps[2].parse::<i64>().unwrap();
            Statement::VarAssign { var, value }
        } else if let Some(caps) = re_update.captures(line) {
            let var = caps[1].to_string();
            let value = caps[3].parse::<i64>().unwrap();
            Statement::VarUpdate { var, value }
        } else if let Some(caps) = re_function_call.captures(line) {
            let name = caps[1].to_string();
            let args: Vec<i64> = caps[2]
                .split(',')
                .map(|arg| arg.trim().parse().unwrap())
                .collect();
            Statement::FunctionCall { name, args }
        } else {
            panic!("Invalid statement: {}", line);
        }
    }

    fn evaluate(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            match statement {
                Statement::VarAssign { var, value } => {
                    self.variables.insert(var, value);
                }
                Statement::VarUpdate { var, value } => {
                    if let Some(var_value) = self.variables.get_mut(&var) {
                        *var_value += value;
                    }
                }
                Statement::IfCondition {
                    var,
                    value,
                    true_branch,
                    false_branch,
                } => {
                    if let Some(var_value) = self.variables.get(&var) {
                        if *var_value == value {
                            self.evaluate(true_branch);
                        } else {
                            self.evaluate(false_branch);
                        }
                    }
                }
                Statement::WhileLoop { var, op, value, body } => {
                    while self.evaluate_condition(&var, &op, value) {
                        self.evaluate(body.clone());
                    }
                }
                Statement::FunctionCall { name, args } => match name.as_str() {
                    "print" => {
                        for arg in args {
                            print!("{} ", arg);
                        }
                        println!();
                    }
                    _ => panic!("Unknown function: {}", name),
                },
            }
        }
    }

    fn evaluate_condition(&self, var: &String, op: &String, value: i64) -> bool {
        if let Some(var_value) = self.variables.get(var) {
            match op.as_str() {
                "==" => *var_value == value,
                "!=" => *var_value != value,
                "<" => *var_value < value,
                ">" => *var_value > value,
                "<=" => *var_value <= value,
                ">=" => *var_value >= value,
                _ => false,
            }
        } else {
            false
        }
    }
}

fn main() {
    let mut interpreter = Interpreter::new();

    let code = r#"
        let x = 10;
        let y = 20;
        x = x + 5;
        if x == 15 {
            print(1, 2, 3);
        } else {
            print(4, 5, 6);
        }
    "#;

    let statements = interpreter.parse(code);
    interpreter.evaluate(statements);

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
