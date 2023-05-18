struct calcNode {
    item: Item,
    left: Option<Box<calcNode>>,
    right: Option<Box<calcNode>>,
}
impl Default for calcNode {
    fn default() -> Self {
        Self {
            item: Item::Num(1),
            left: None,
            right: None,
        }
    }
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
#[derive(Default)]
pub struct calcTree {
    root: calcNode,
}

impl calcTree {}
