use crate::astnodes::*;
use crate::parser::*;
use crate::stacknodes::StackOp as Op;
use crate::stacknodes::*;
pub struct IRgen {
    ast: Node,
    stack: Stack,
    idents: Vec<String>,
}
impl IRgen {
    pub fn new(input: &str) -> Self {
        let mut parser = Parser::new(input.clone());
        Self {
            ast: parser.parse(),
            stack: vec![],
            idents: vec![],
        }
    }
    pub fn binary_to_op(&self, kind: BinaryOp) -> StackOp {
        match kind {
            BinaryOp::Add => StackOp::Add,
            BinaryOp::Divide => StackOp::Div,
            BinaryOp::Multiply => StackOp::Mult,
            BinaryOp::Subtract => StackOp::Sub,
        }
    }
    pub fn unary_to_op(&self, kind: UnaryOp) -> StackOp {
        match kind {
            UnaryOp::Negate => StackOp::Neg,
        }
    }
    pub fn generate(&mut self) -> Stack {
        println!("{:?}", self.ast);
        self.node_gen(self.ast.clone());
        self.stack.clone()
    }
    fn node_gen(&mut self, node: Node) {
        match node {
            Node::Value(val) => self.stack.push(Op::Push(val)),
            Node::BinaryNode(kind, lhs, rhs) => {
                self.node_gen(*lhs);
                self.node_gen(*rhs);
                self.stack.push(self.binary_to_op(kind));
            }
            Node::UnaryNode(kind, target) => {
                self.node_gen(*target);
                self.stack.push(self.unary_to_op(kind));
            }
            Node::Def(name, expr) => {
                self.node_gen(*expr);
                self.idents.push(name);
            }
            Node::Assign(name, expr) => {
                let index = self
                    .idents
                    .binary_search(&name)
                    .expect("Non Existent variable dumbasss");
                self.node_gen(*expr);
                self.stack.push(StackOp::Store(index));
            }
            Node::Ident(name) => {
                let index = self
                    .idents
                    .binary_search(&name)
                    .expect("Non Existent variable dumbasss");
                self.stack.push(Op::Load(index))
            }
            _ => {
                todo!()
            }
        }
    }
}
