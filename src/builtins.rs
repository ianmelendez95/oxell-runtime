use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;

#[derive(Clone)]
pub enum Node {
    Int(i64),
    ThunkRef(Box<Thunk>)
}

pub type Stack = Vec<Node>;

pub struct State {
    pub stack: Stack,
}

#[derive(Clone)]
pub enum Thunk {
    UThunk(fn() -> Node),
    EThunk(Node)
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
            Node::ThunkRef(t) => unreachable!("Asked to display thunk: {:?}", t), 
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(i) => {
                write!(f, "Int({})", i)
            }, 
            Node::ThunkRef(t) => {
                write!(f, "{:?}", t)
            }
        }
    }
}

impl fmt::Debug for Thunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Thunk::UThunk(func) => write!(f, "#[UNEVALED]"),
            Thunk::EThunk(val) => write!(f, "#[{:?}]", val),
        }
    }
}

pub fn int(state: &mut State, int_val: i64) {
    state.stack.push(Node::Int(int_val));
}

pub fn add(nl: Node, nr: Node) -> Node {
    let vl: Node = eval(nl);
    let vr: Node = eval(nr);

    if let Node::Int(vl) = vl {
        if let Node::Int(vr) = vr {
            return Node::Int(vl + vr);
        } else {
            panic!("Expecting integer for right operand: {:?}", vr)
        }
    } else {
        panic!("Expecting integer for left operand: {:?}", vl)
    }
}

pub fn eval(node: Node) -> Node {
    match node {
        Node::Int(_) => node,
        Node::ThunkRef(t) => {
            match *t {
                Thunk::UThunk(func) => {
                    return func();
                },
                Thunk::EThunk(val) => val,
            }
        }
    }
}

// pub fn add(state: &mut State) {
//     let nl: Node = state.stack.pop().unwrap();
//     let nr: Node = state.stack.pop().unwrap();

//     let vl: Node = eval(state, nl);
//     let vr: Node = eval(state, nr);

//     if let Node::Int(vl) = vl {
//         if let Node::Int(vr) = vr {
//             int(state, vl + vr)
//         } else {
//             panic!("Expecting integer for right operand: {:?}", vr)
//         }
//     } else {
//         panic!("Expecting integer for left operand: {:?}", vl)
//     }
// }

pub fn sub(state: &mut State) {
    let el: Node = state.stack.pop().unwrap();
    let er: Node = state.stack.pop().unwrap();

    if let Node::Int(vl) = el {
        if let Node::Int(vr) = er {
            int(state, vl - vr)
        } else {
            panic!("Expecting integer for right operand: {:?}", er)
        }
    } else {
        panic!("Expecting integer for left operand: {:?}", el)
    }
}

pub fn mul(state: &mut State) {
    let el: Node = state.stack.pop().unwrap();
    let er: Node = state.stack.pop().unwrap();

    if let Node::Int(vl) = el {
        if let Node::Int(vr) = er {
            int(state, vl * vr)
        } else {
            panic!("Expecting integer for right operand: {:?}", er)
        }
    } else {
        panic!("Expecting integer for left operand: {:?}", el)
    }
}

pub fn div(state: &mut State) {
    let el: Node = state.stack.pop().unwrap();
    let er: Node = state.stack.pop().unwrap();

    if let Node::Int(vl) = el {
        if let Node::Int(vr) = er {
            int(state, vl / vr)
        } else {
            panic!("Expecting integer for right operand: {:?}", er)
        }
    } else {
        panic!("Expecting integer for left operand: {:?}", el)
    }
}

// pub fn thunk(state: &mut State, func: fn(&mut State)) {
//     state.stack.push(Node::ThunkRef(Box::from(Thunk::UThunk(func))));
// }

// pub fn eval(state: &mut State, node: Node) -> Node {
//     match node {
//         Node::Int(_) => node,
//         Node::ThunkRef(t) => {
//             match *t {
//                 Thunk::UThunk(func) => {
//                     // TODO: implement reuse
//                     let mut thunk_state = State::new();
//                     func(&mut thunk_state);
//                     return thunk_state.stack.pop().unwrap();
//                 },
//                 Thunk::EThunk(val) => val,
//             }
//         }
//     }
// }

pub fn print_top(state: &State) {
    let x: &Node = state.stack.get(state.stack.len() - 1).unwrap();
    println!("{}", x);
}