#[derive(Debug, Clone)]
pub enum StackOp {
    Push(f32),
    Load(usize),
    Store(usize),
    Pop,
    Add,
    Sub,
    Mult,
    Div,
    Neg,
    Goto(usize, bool),
}
pub type Stack = Vec<StackOp>;
