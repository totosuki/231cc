#[derive(PartialEq, Eq)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num,
    EQ,
    NE,
    LT,
    LE,
}

pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: Option<usize>,
}

impl Node {
    pub fn new_node(kind: NodeKind, lhs: Node, rhs: Node) -> Self {
        Node {
            kind,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
            val: None,
        }
    }

    pub fn new_node_num(val: usize) -> Self {
        Node {
            kind: NodeKind::Num,
            lhs: None,
            rhs: None,
            val: Some(val),
        }
    }
}
