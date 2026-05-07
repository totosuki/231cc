use crate::node::*;
use crate::tokenizer::*;

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Parser { tokenizer }
    }

    pub fn expr(&mut self) -> Node {
        self.euqality()
    }

    pub fn euqality(&mut self) -> Node {
        let mut node: Node = self.relational();

        loop {
            if self.tokenizer.consume(&['=', '=']) {
                node = Node::new_node(NodeKind::EQ, node, self.relational());
            } else if self.tokenizer.consume(&['!', '=']) {
                node = Node::new_node(NodeKind::NE, node, self.relational());
            } else {
                return node;
            }
        }
    }

    pub fn relational(&mut self) -> Node {
        let mut node: Node = self.add();

        loop {
            if self.tokenizer.consume(&['<']) {
                node = Node::new_node(NodeKind::LT, node, self.add());
            } else if self.tokenizer.consume(&['<', '=']) {
                node = Node::new_node(NodeKind::LE, node, self.add());
            } else if self.tokenizer.consume(&['>']) {
                node = Node::new_node(NodeKind::LT, self.add(), node);
            } else if self.tokenizer.consume(&['>', '=']) {
                node = Node::new_node(NodeKind::LE, self.add(), node);
            } else {
                return node;
            }
        }
    }

    pub fn add(&mut self) -> Node {
        let mut node: Node = self.mul();

        loop {
            if self.tokenizer.consume(&['+']) {
                node = Node::new_node(NodeKind::Add, node, self.mul());
            } else if self.tokenizer.consume(&['-']) {
                node = Node::new_node(NodeKind::Sub, node, self.mul());
            } else {
                return node;
            }
        }
    }

    pub fn mul(&mut self) -> Node {
        let mut node: Node = self.unary();

        loop {
            if self.tokenizer.consume(&['*']) {
                node = Node::new_node(NodeKind::Mul, node, self.unary());
            } else if self.tokenizer.consume(&['/']) {
                node = Node::new_node(NodeKind::Div, node, self.unary());
            } else {
                return node;
            }
        }
    }

    pub fn unary(&mut self) -> Node {
        if self.tokenizer.consume(&['+']) {
            self.primary()
        } else if self.tokenizer.consume(&['-']) {
            Node::new_node(NodeKind::Sub, Node::new_node_num(0), self.primary())
        } else {
            self.primary()
        }
    }

    pub fn primary(&mut self) -> Node {
        if self.tokenizer.consume(&['(']) {
            let node: Node = self.expr();
            self.tokenizer.expect(&[')']);
            return node;
        }

        Node::new_node_num(self.tokenizer.expect_number())
    }
}
