use crate::ast::{ AstOp, Expr, Stmt };

use super::node::Node;

pub struct Compiler {
    node_stack: Vec<Node>,
    ast: Vec<Stmt>,
    cursor: usize,
}

impl Compiler {
    fn current(&self) -> &Stmt {
        return self.ast.get(self.cursor).unwrap_or(self.ast.last().unwrap());
    }

    fn peek(&self) -> &Stmt {
        return self.ast.get(self.cursor + 1).unwrap_or(self.ast.last().unwrap());
    }
}

impl Compiler {
    pub fn new(ast: Vec<Stmt>) -> Self {
        Compiler {
            node_stack: vec![],
            ast,
            cursor: 0usize,
        }
    }

    pub fn compile(&mut self) {
        let ast: Vec<Stmt> = self.ast.drain(0..).collect();
        for stmt in ast {
            self.compile_stmt(stmt);
        }

        println!("Compiled IR:\n{:#?}", self.node_stack);
    }

    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Binding { span: _, mutable, name, initializer, annotation } => {
                let value = initializer.unwrap_or_else(|| { unimplemented!() });
                self.compile_expr(*value);

                let value = self.node_stack.pop().unwrap();
                if mutable {
                    self.node_stack.push(Node::StoreMut { symbol: name, value: Box::new(value) });
                } else {
                    self.node_stack.push(Node::StoreConst { symbol: name, value: Box::new(value) });
                }
            }
            _ => unimplemented!(),
        }
    }

    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Integer { span: _, value } => self.node_stack.push(Node::Integer(value)),
            Expr::Symbol { span: _, name } => self.node_stack.push(Node::Symbol(name)),

            Expr::Binary { span: _, lhs, rhs, op } => {
                match op {
                    AstOp::Plus => {
                        self.compile_expr(*lhs);
                        self.compile_expr(*rhs);
                        let rhs = self.node_stack.pop().unwrap();
                        let lhs = self.node_stack.pop().unwrap();
                        self.node_stack.push(Node::Add { lhs: Box::new(lhs), rhs: Box::new(rhs) });
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}
