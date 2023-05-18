use std::io::Error;

#[derive(Clone)]
struct CalcNode {
    item: Item,
    left: Option<Box<CalcNode>>,
    right: Option<Box<CalcNode>>,
}

impl CalcNode {
    pub fn new(item: Item, left: Option<Box<CalcNode>>, right: Option<Box<CalcNode>>) -> Self {
        Self { item, left, right }
    }
}
#[derive(Clone)]
pub enum Item {
    Num(i32),
    Oper(Operation),
}
#[derive(Clone)]
pub enum Operation {
    Add,
    Sub,
    Div,
    Mult,
}
pub struct CalcTree {
    root: CalcNode,
}

impl CalcTree {
    pub fn new(item: Item) -> Self {
        let root = CalcNode::new(item, None, None);
        Self { root }
    }
    pub fn insert(&mut self, item: Item) {
        use Item::*;
        if let Oper(_) = item {
            match &self.root.item {
                Num(_) => self.root = CalcNode::new(item, Some(Box::new(self.root.clone())), None),
                Oper(_) => panic!("fuck off"),
            }
        } else {
            match &self.root.item {
                Num(_) => panic!("fuck off"),
                Oper(_) => {
                    if let None = self.root.right {
                        self.root.right = Some(Box::new(CalcNode::new(item, None, None)));
                    } else if let None = self.root.left {
                        self.root.left = Some(Box::new(CalcNode::new(item, None, None)));
                    } else {
                        self.insert(item)
                    }
                }
            }
        }
    }
}
