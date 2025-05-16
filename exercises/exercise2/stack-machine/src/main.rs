fn main() {
    let mut stack_machine = StackMachine::new();
    let instructions = vec![
        Instruction::LOAD(2),
        Instruction::LOAD(3),
        Instruction::ADD,
        Instruction::LOAD(4),
        Instruction::MUL,
    ];

    stack_machine.process_instructions(&instructions);

    println!("Result: {:?}", stack_machine.data);
}

enum Instruction {
    LOAD(u8),
    ADD,
    SUB,
    MUL,
    DIV,
}

struct StackMachine {
    data: Vec<u8>
}

impl StackMachine {
    fn new() -> Self {
        StackMachine { data: Vec::new() }
    }

    fn process_instructions(&mut self, instructions: &Vec<Instruction>) {
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
    }
}