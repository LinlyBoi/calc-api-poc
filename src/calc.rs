struct calcNode {
    item: Item,
    left: Option<Box<calcNode>>,
    right: Option<Box<calcNode>>,
}

impl calcNode {
    pub fn new(item: Item, left: Option<Box<calcNode>>, right: Option<Box<calcNode>>) -> Self {
        Self { item, left, right }
    }
}
pub enum Item {
    Num(i32),
    Oper(Operation),
}
pub enum Operation {
    Add,
    Sub,
    Div,
    Mult,
}
pub struct calcTree {
    root: calcNode,
}

impl calcTree {
    pub fn new(item: Item) -> Self {
        let root = calcNode::new(item, None, None);
        Self { root }
    }
}
