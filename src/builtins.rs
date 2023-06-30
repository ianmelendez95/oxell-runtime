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

pub fn push_int(state: &mut State, int_val: i64) {
    state.stack.push(Box::from(Node::Int(int_val)));
}

pub fn add(state: &mut State) {
    let el: Node = *state.stack.pop().unwrap();
    let er: Node = *state.stack.pop().unwrap();

    if let Node::Int(vl) = el {
        if let Node::Int(vr) = er {
            push_int(state, vl + vr)
        } else {
            panic!("Expecting integer for right operand: {:?}", er)
        }
    } else {
        panic!("Expecting integer for left operand: {:?}", el)
    }
}

pub fn sub(state: &mut State) {
    let el: Node = *state.stack.pop().unwrap();
    let er: Node = *state.stack.pop().unwrap();

    if let Node::Int(vl) = el {
        if let Node::Int(vr) = er {
            push_int(state, vl - vr)
        } else {
            panic!("Expecting integer for right operand: {:?}", er)
        }
    } else {
        panic!("Expecting integer for left operand: {:?}", el)
    }
}

pub fn mul(state: &mut State) {
    let el: Node = *state.stack.pop().unwrap();
    let er: Node = *state.stack.pop().unwrap();

    if let Node::Int(vl) = el {
        if let Node::Int(vr) = er {
            push_int(state, vl * vr)
        } else {
            panic!("Expecting integer for right operand: {:?}", er)
        }
    } else {
        panic!("Expecting integer for left operand: {:?}", el)
    }
}

pub fn div(state: &mut State) {
    let el: Node = *state.stack.pop().unwrap();
    let er: Node = *state.stack.pop().unwrap();

    if let Node::Int(vl) = el {
        if let Node::Int(vr) = er {
            push_int(state, vl / vr)
        } else {
            panic!("Expecting integer for right operand: {:?}", er)
        }
    } else {
        panic!("Expecting integer for left operand: {:?}", el)
    }
}

pub fn print_top(state: &State) {
    let x: &Node = state.stack.get(state.stack.len() - 1).unwrap();
    println!("{}", x);
}