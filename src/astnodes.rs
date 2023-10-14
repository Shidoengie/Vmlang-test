#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(Debug, Clone)]
pub enum UnaryOp {
    Negate,
}
#[derive(Debug, Clone)]
pub enum Node {
    Value(f32),
    BinaryNode(BinaryOp, Box<Self>, Box<Self>),
    UnaryNode(UnaryOp, Box<Self>),
    Def(String, Box<Self>),
    Assign(String, Box<Self>),
    Ident(String),
}
