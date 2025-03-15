#[derive(Debug)]
pub enum Node {
    Integer(i32),
    Symbol(String),

    StoreMut {
        symbol: String,
        value: Box<Node>,
    },
    StoreConst {
        symbol: String,
        value: Box<Node>,
    },
    Add {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
