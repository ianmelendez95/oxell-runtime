use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub enum Node {
    Int(i64),
    ThunkRef(Box<Thunk>)
}

impl Node {
    pub fn eval(self: Node) -> Node {
        match self {
            Node::Int(_) => self,
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
            Thunk::UThunk(_) => write!(f, "#[UNEVALED]"),
            Thunk::EThunk(val) => write!(f, "#[{:?}]", val),
        }
    }
}

pub fn int(int_val: i64) -> Node {
    return Node::Int(int_val);
}

pub fn thunk(tfun: fn() -> Node) -> Node {
    return Node::ThunkRef(Box::from(Thunk::UThunk(tfun)));
}

macro_rules! bin_arith {
    ($nl:ident, $nr:ident, $op:tt) => {
        let vl: Node = $nl.eval();
        let vr: Node = $nr.eval();

        if let Node::Int(vl) = vl {
            if let Node::Int(vr) = vr {
                return Node::Int(vl $op vr);
            } else {
                panic!("Expecting integer for right operand: {:?}", vr)
            }
        } else {
            panic!("Expecting integer for left operand: {:?}", vl)
        }
    };
}

pub fn add(nl: Node, nr: Node) -> Node {
    bin_arith!(nl, nr, +);
}

pub fn sub(nl: Node, nr: Node) -> Node {
    bin_arith!(nl, nr, -);
}

pub fn div(nl: Node, nr: Node) -> Node {
    bin_arith!(nl, nr, /);
}

pub fn mul(nl: Node, nr: Node) -> Node {
    bin_arith!(nl, nr, *);
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

// pub fn sub(state: &mut State) -> Node {
//     let el: Node = state.stack.pop().unwrap();
//     let er: Node = state.stack.pop().unwrap();

//     if let Node::Int(vl) = el {
//         if let Node::Int(vr) = er {
//             int(vl - vr)
//         } else {
//             panic!("Expecting integer for right operand: {:?}", er)
//         }
//     } else {
//         panic!("Expecting integer for left operand: {:?}", el)
//     }
// }

// pub fn mul(state: &mut State) {
//     let el: Node = state.stack.pop().unwrap();
//     let er: Node = state.stack.pop().unwrap();

//     if let Node::Int(vl) = el {
//         if let Node::Int(vr) = er {
//             int(state, vl * vr)
//         } else {
//             panic!("Expecting integer for right operand: {:?}", er)
//         }
//     } else {
//         panic!("Expecting integer for left operand: {:?}", el)
//     }
// }

// pub fn div(state: &mut State) {
//     let el: Node = state.stack.pop().unwrap();
//     let er: Node = state.stack.pop().unwrap();

//     if let Node::Int(vl) = el {
//         if let Node::Int(vr) = er {
//             int(state, vl / vr)
//         } else {
//             panic!("Expecting integer for right operand: {:?}", er)
//         }
//     } else {
//         panic!("Expecting integer for left operand: {:?}", el)
//     }
// }

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