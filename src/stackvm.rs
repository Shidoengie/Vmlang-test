use crate::codegen::IRgen;
use crate::stacknodes::{Stack, StackOp};
pub struct StackVM {
    proc: Vec<StackOp>,
    counter: usize,
    pub values: Vec<f32>,
}
impl StackVM {
    pub fn new(proc: Vec<StackOp>) -> Self {
        Self {
            proc,
            counter: 0,
            values: vec![],
        }
    }
    pub fn exec_from(source: &str) -> f32 {
        let stack = IRgen::new(source).generate();
        Self::new(stack).exec()
    }
    pub fn exec(&mut self) -> f32 {
        let stacklen = self.proc.len();

        loop {
            self.exec_op(self.proc[self.counter].clone());
            if self.counter > stacklen - 1 {
                break;
            }
        }
        return self.values[0];
    }

    fn exec_op(&mut self, op: StackOp) {
        match op {
            StackOp::Push(val) => {
                println!("{:?}", &self.values);
                self.values.push(val);
                self.counter += 1;
            }
            StackOp::Load(index) => {
                self.push(self.values[index]);
                self.counter += 1;
            }
            StackOp::Store(index) => {
                let val = self.pop();
                self.values[index] = val;
                self.counter += 1;
            }
            StackOp::Pop => {
                self.pop();
                self.counter += 1;
            }
            StackOp::Add => {
                let (left, right) = self.pop_pair();
                self.push(left + right);
                self.counter += 1;
            }
            StackOp::Sub => {
                let (left, right) = self.pop_pair();
                self.push(left - right);
                self.counter += 1;
            }
            StackOp::Div => {
                let (left, right) = self.pop_pair();
                self.push(left / right);
                self.counter += 1;
            }
            StackOp::Mult => {
                let (left, right) = self.pop_pair();
                self.push(left * right);
                self.counter += 1;
            }
            StackOp::Neg => {
                let value = self.pop();
                self.push(-value);
                self.counter += 1;
            }
            StackOp::Goto(line, cond) => {
                if line > self.proc.len() {
                    panic!("Invalid goto line ya cunt the stack len is {} but your stupid ass supplied {line}",self.proc.len())
                }
                if cond {
                    self.counter = line;
                }
            }
        }
    }
    fn pop(&mut self) -> f32 {
        self.values.pop().expect("Expected another value")
    }
    fn push(&mut self, value: f32) {
        self.values.push(value);
        println!("a");
    }
    fn pop_pair(&mut self) -> (f32, f32) {
        let pair = (self.pop(), self.pop());
        (pair.1, pair.0)
    }
}
