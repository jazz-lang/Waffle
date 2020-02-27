use crate::bytecode::*;
use basicblock::BasicBlock;
use instruction::{BinOp, Instruction, UnaryOp};

pub struct BaselineJIT {
    stack: usize,
    buffer: String,
}

impl BaselineJIT {
    pub fn translate_instruction(&mut self, ins: Instruction) {
        match ins {
            Instruction::Branch(block) => self.write(&format!("goto bb{}", block)),
            Instruction::ConditionalBranch(value, if_true, if_false) => {
                self.write(&format!("bool cmp_result = value_to_bool(r{});", value));
                self.write(&format!(
                    "if (cmp_result) goto bb{} else goto bb{};",
                    if_true, if_false
                ));
            }
            Instruction::BranchIfTrue(value, if_true) => {
                self.write(&format!(
                    "
                        bool cmp_result = value_to_bool(r{});

                        if (cmp_result) goto bb{};
                    ",
                    value, if_true
                ));
            }
            Instruction::Binary(op,dest,lhs,rhs) => {
                let name = match op {
                    BinOp::Add => "add",
                    BinOp::Sub => "sub",
                    BinOp::Mul => "mul",
                    BinOp::Div => "div",
                    BinOp::Mod => "mod",
                    BinOp::Greater => "gt",
                    BinOp::Less => "lt",
                    BinOp::LessOrEqual => "lte",
                    BinOp::GreaterOrEqual => "gte",
                    BinOp::Equal => "eq",
                    BinOp::NotEqual => "neq",
                    _ => unimplemented!()
                };
                self.write(&format!("r{} = value_{}(r{},r{})",dest,name,lhs,rhs));
            }
            Instruction::Call(dest,callee,argc) => {
                self.write(&format!("r{} = value_call(r{},{},stack)",dest,callee,argc));
            }
            Instruction::Throw(_) => panic!("Cannot JIT functions thay may throw exceptions"),
            Instruction::CatchBlock { .. } => panic!("Cannot JIT functions with try/catch"),
            _ => unimplemented!(),
        }
    }
    pub fn write(&mut self, string: &str) {
        self.buffer.push_str(string);
    }
    pub fn translate_block(&mut self, bb: &BasicBlock, index: usize) {
        self.write(&format!("bb{}: ", index));
        for ins in bb.instructions.iter() {
            self.translate_instruction(*ins);
        }
    }
}
