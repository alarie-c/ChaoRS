use std::{ collections::HashMap, vec };

use crate::{ ast::{ Expr, Stmt }, errors::{ self, CompilerError } };

#[derive(Debug, PartialEq, Eq)]
enum Type {
    None,
    Integer,
    String,
}

struct Symbol<'a> {
    name: &'a String,
    mutable: bool,
    typ: Type,
}

struct Context<'a> {
    pub symbols: HashMap<&'a String, Symbol<'a>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            symbols: HashMap::new(),
        }
    }

    pub fn load(&mut self, key: &'a String, mutable: bool, typ: Type) -> bool {
        return self.symbols.insert(key, Symbol { name: key, mutable, typ }).is_some();
    }

    pub fn check(&self, key: &'a String) -> bool {
        return self.symbols.get(key).is_some();
    }
}

pub struct Resolver<'a> {
    pub errors: Vec<CompilerError>,
    global: Vec<Context<'a>>,
    current_ctx: usize,
    // (TODO) keep track of user defined types?
}

impl<'a> Resolver<'a> {
    pub fn new() -> Self {
        Resolver {
            errors: vec![],
            global: vec![Context::new()],
            current_ctx: 0usize,
        }
    }

    pub fn resolve_symbol(&mut self, expr: &'a Expr) -> bool {
        match expr {
            Expr::Symbol { span, name } => {
                if !self.global[self.current_ctx].check(name) {
                    self.errors.push(
                        CompilerError::new(
                            errors::Kind::NameError,
                            errors::Flag::Abort,
                            span.line,
                            span.start,
                            span.stop - span.start,
                            format!("unknown symbol '{}'", name).as_str()
                        )
                    );
                }
                return false;
            }
            _ => true,
        }
    }

    pub fn resolve_expr(&mut self, expr: &'a Expr) -> Type {
        match expr {
            // Expr::Symbol { span, name } => {
            //     match name.as_str() {
            //         "int" => Type::Integer,
            //         "str" => Type::String,
            //         _ => {
            // self.errors.push(
            //     CompilerError::new(
            //         errors::Kind::NameError,
            //         errors::Flag::Abort,
            //         span.line,
            //         span.start,
            //         span.stop - span.start,
            //         format!("unknown symbol '{}'", name).as_str()
            //     )
            // );
            //             return Type::None;
            //         }
            //     }
            // }
            Expr::Integer { span: _, value: _ } => Type::Integer,
            Expr::String { span: _, value: _ } => Type::String,

            Expr::Binary { span, lhs, rhs, op: _ } => {
                // Check to see if these symbols exist
                if !self.resolve_symbol(&**lhs) || !self.resolve_symbol(&**rhs) {
                    return Type::None;
                }

                // Resolve the types of each node
                let lhs_type = self.resolve_expr(&**lhs);
                let rhs_type = self.resolve_expr(&**rhs);
                if lhs_type == rhs_type {
                    return lhs_type;
                } else {
                    self.errors.push(
                        CompilerError::new(
                            errors::Kind::TypeError,
                            errors::Flag::Abort,
                            span.line,
                            span.start,
                            span.stop - span.start,
                            "these types are not compatible in a binary expression"
                        )
                    );
                    return Type::None;
                }
            }
            _ => unimplemented!(),
        }
    }

    pub fn resolve_names(&mut self, ast: &'a Vec<Stmt>) {
        for stmt in ast {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &'a Stmt) {
        match stmt {
            Stmt::Binding { span, mutable, name, initializer, annotation: _ } => {
                let value = initializer.as_ref().unwrap_or_else(|| { unimplemented!() });
                let typ = self.resolve_expr(&*value);
                self.global[self.current_ctx].load(name, *mutable, typ);
            }
            _ => unimplemented!(),
        }
    }
}
