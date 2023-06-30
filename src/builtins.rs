use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;

#[derive(Clone)]
pub enum Node {
    Int(i64)
}

pub type Stack = Vec<Box<Node>>;

pub struct State {
    pub stack: Stack,
}

impl State {
    pub fn new() -> State {
        return State {
            stack: Vec::new()
        };
    }

    pub fn push_int(self: &mut State, int_val: i64) {
        self.stack.push(Box::from(Node::Int(int_val)));
    }

    pub fn add(self: &mut State) {
        let el: Node = *self.stack.pop().unwrap();
        let er: Node = *self.stack.pop().unwrap();

        if let Node::Int(vl) = el {
            if let Node::Int(vr) = er {
                self.push_int(vl + vr)
            } else {
                panic!("Expecting integer for right operand: {:?}", er)
            }
        } else {
            panic!("Expecting integer for left operand: {:?}", el)
        }
    }

    pub fn sub(self: &mut State) {
        let el: Node = *self.stack.pop().unwrap();
        let er: Node = *self.stack.pop().unwrap();

        if let Node::Int(vl) = el {
            if let Node::Int(vr) = er {
                self.push_int(vl - vr)
            } else {
                panic!("Expecting integer for right operand: {:?}", er)
            }
        } else {
            panic!("Expecting integer for left operand: {:?}", el)
        }
    }

    pub fn mul(self: &mut State) {
        let el: Node = *self.stack.pop().unwrap();
        let er: Node = *self.stack.pop().unwrap();

        if let Node::Int(vl) = el {
            if let Node::Int(vr) = er {
                self.push_int(vl * vr)
            } else {
                panic!("Expecting integer for right operand: {:?}", er)
            }
        } else {
            panic!("Expecting integer for left operand: {:?}", el)
        }
    }

    pub fn div(self: &mut State) {
        let el: Node = *self.stack.pop().unwrap();
        let er: Node = *self.stack.pop().unwrap();

        if let Node::Int(vl) = el {
            if let Node::Int(vr) = er {
                self.push_int(vl / vr)
            } else {
                panic!("Expecting integer for right operand: {:?}", er)
            }
        } else {
            panic!("Expecting integer for left operand: {:?}", el)
        }
    }

    pub fn print_top(self: &State) {
        let x: &Node = self.stack.get(self.stack.len() - 1).unwrap();
        println!("{}", x);
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(i) => {
                write!(f, "{}", i)
            }
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(i) => {
                write!(f, "Int({})", i)
            }
        }
    }
}
