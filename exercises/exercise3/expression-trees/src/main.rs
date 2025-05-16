fn main() {
    let tree = Node::new(
        Expression::ADD,
        Some(Box::new(Node::new(
            Expression::DIV,
            Some(Box::new(Node::new(
                Expression::Value(6),
                None,
                None
            ))),
            Some(Box::new(Node::new(
                Expression::Value(3),
                None,
                None
            )))
        ))),
        Some(Box::new(Node::new(
            Expression::MUL,
            Some(Box::new(Node::new(
                Expression::Value(2),
                None,
                None
            ))),
            Some(Box::new(Node::new(
                Expression::SUB,
                Some(Box::new(Node::new(
                    Expression::Value(4),
                    None,
                    None
                ))),
                Some(Box::new(Node::new(
                    Expression::Value(2),
                    None,
                    None
                )))
            )))
        )))
    );

    let result = tree.eval();
    println!("Result: {}", result);

    let instructions = tree.compile();
    println!("{:?}", instructions);

    let mut stack_machine = StackMachine::new();
    let result = stack_machine.process_instructions(&instructions);
    println!("Result: {}", result);


    let tree2 = Node::new(
        Expression::MUL,
        Some(Box::new(Node::new(
            Expression::ADD,
            Some(Box::new(Node::new(
                Expression::Value(2),
                None,
                None
            ))),
            Some(Box::new(Node::new(
                Expression::Value(3),
                None,
                None
            )))
        ))),
        Some(Box::new(Node::new(
            Expression::Value(4),
            None,
            None
        )))
    );

    let result = tree2.eval();
    println!("Result: {}", result);

    let instructions = tree2.compile();
    println!("{:?}", instructions);

    let mut stack_machine = StackMachine::new();
    let result = stack_machine.process_instructions(&instructions);
    println!("Result: {}", result);
}

enum Expression {
    Value(u32),
    ADD,
    SUB,
    DIV,
    MUL,
}

struct Node {
    value: Expression,
    left: Option<Box<Node>>, // boxed for infinite size error
    right: Option<Box<Node>>, // boxed for infinite size error
}

impl Node {
    fn new(value: Expression, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self {
            value,
            left,
            right,
        }
    }

    fn eval(&self) -> u32 {
        match &self.value {
            Expression::Value(value) => *value,
            Expression::ADD => self.left.as_ref().unwrap().eval() + self.right.as_ref().unwrap().eval(),
            Expression::SUB => self.left.as_ref().unwrap().eval() - self.right.as_ref().unwrap().eval(),
            Expression::MUL => self.left.as_ref().unwrap().eval() * self.right.as_ref().unwrap().eval(),
            Expression::DIV => self.left.as_ref().unwrap().eval() / self.right.as_ref().unwrap().eval(),
        }
    }

    fn compile(&self) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        match &self.value {
            Expression::Value(value) => {
                instructions.push(Instruction::LOAD(*value));
            },
            Expression::ADD => {
                instructions.extend(self.left.as_ref().unwrap().compile());
                instructions.extend(self.right.as_ref().unwrap().compile());
                instructions.push(Instruction::ADD);
            },
            Expression::SUB => {
                instructions.extend(self.left.as_ref().unwrap().compile());
                instructions.extend(self.right.as_ref().unwrap().compile());
                instructions.push(Instruction::SUB);
            },
            Expression::MUL => {
                instructions.extend(self.left.as_ref().unwrap().compile());
                instructions.extend(self.right.as_ref().unwrap().compile());
                instructions.push(Instruction::MUL);
            },
            Expression::DIV => {
                instructions.extend(self.left.as_ref().unwrap().compile());
                instructions.extend(self.right.as_ref().unwrap().compile());
                instructions.push(Instruction::DIV);
            }
        }
        instructions
    }
}

#[derive(Debug)]
enum Instruction {
    LOAD(u32),
    ADD,
    SUB,
    DIV,
    MUL,
}

struct StackMachine {
    data: Vec<u32>
}

impl StackMachine {
    fn new() -> Self {
        StackMachine { data: Vec::new() }
    }

    fn process_instructions(&mut self, instructions: &Vec<Instruction>) -> u32 {
        for instruction in instructions {
            match instruction {
                Instruction::LOAD(value) => self.data.push(*value),
                Instruction::ADD => {
                    let a = self.data.pop().unwrap();
                    let b = self.data.pop().unwrap();
                    self.data.push(a + b);
                }
                Instruction::SUB => {
                    let a = self.data.pop().unwrap();
                    let b = self.data.pop().unwrap();
                    self.data.push(b - a);
                }
                Instruction::MUL => {
                    let a = self.data.pop().unwrap();
                    let b = self.data.pop().unwrap();
                    self.data.push(a * b);
                },
                Instruction::DIV => {
                    let a = self.data.pop().unwrap();
                    let b = self.data.pop().unwrap();
                    self.data.push(b / a);
                }
            }
        }
        self.data.pop().unwrap()
    }
}
