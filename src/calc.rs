use std::io::Error;

#[derive(Clone)]
pub struct CalcNode {
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
        if let Oper(operation) = &item {
            match &self.root.item {
                Num(_) => {
                    if matches!(*operation, Operation::Div) || matches!(*operation, Operation::Mult)
                    {
                        let inserted = CalcNode::new(item, self.root.right.clone(), None);
                        self.root.right = Some(Box::new(inserted));
                    } else {
                        self.root = CalcNode::new(item, Some(Box::new(self.root.clone())), None);
                    }
                }
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
    pub fn resolve(root: &CalcNode) -> Item {
        use Item::*;
        if let Oper(operation) = &root.item {
            let mut a: i32 = 0;
            let mut b: i32 = 0;
            if let Some(node) = &root.left {
                match node.item {
                    Num(value) => a = value,
                    Oper(_) => {
                        if let Num(value) = CalcTree::resolve(&node) {
                            a = value
                        }
                    }
                }
            }
            if let Some(node) = &root.right {
                match node.item {
                    Num(value) => b = value,
                    Oper(_) => {
                        if let Num(value) = CalcTree::resolve(&node) {
                            b = value
                        }
                    }
                }
            }
            match operation {
                Operation::Add => Num(a + b),
                Operation::Sub => Num(a - b),
                Operation::Div => Num(a / b),
                Operation::Mult => Num(a * b),
            }
        } else {
            root.item.clone()
        }
    }
}
