use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::RefMut;
use crate::gc::*;

pub struct State {
    pub stack: Stack
}

pub type StateFn = fn(state: &mut State);

impl State {
    pub fn new() -> Self {
        State {
            stack: Vec::new()
        }
    }
}

#[derive(Clone, Copy)]
pub struct FnDef {
    name: &'static str,
    arity: u32,
    fn_ref: StateFn
}

#[derive(Clone)]
pub enum Node {
    Int(i64),
    FnDef(FnDef),
    App(Box<Node>, Box<Node>),
    ThunkRef(Rc<RefCell<Thunk>>)
}

impl Node {
    pub fn eval(self) -> Node {
        self.reduce();

        match self {
            Node::Int(_) => self,
            Node::ThunkRef(t_ref) => {
                match &*t_ref.borrow() {
                    Thunk::UThunk(_) => panic!(),
                    Thunk::EThunk(value) => {
                        value.clone()
                    }
                }
            }
            Node::App(_, _) => todo!(),
            Node::FnDef(_) => todo!(),
        }
    }

    fn reduce(&self) {
        if let Node::ThunkRef(t_ref) = self {
            RefMut::map(t_ref.as_ref().borrow_mut(), |t_mut| {
                if let Thunk::UThunk(thunk) = t_mut {
                    *t_mut = Thunk::EThunk(thunk.eval_thunk().eval());
                    t_mut
                } else {
                    t_mut  // noop
                }
            });

            if let Thunk::UThunk(_) = &*t_ref.borrow() {
                self.reduce();
            }
        }
    }
}

pub enum Thunk {
    UThunk(Box<dyn ThunkEval>),
    EThunk(Node)
}

pub trait ThunkEval {
    fn eval_thunk(&self) -> Node;
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(i) => {
                write!(f, "{}", i)
            }
            Node::ThunkRef(t) => {
                unreachable!("Asked to display thunk: {:?}", (*t).borrow());
            },
            Node::App(_, _) => {
                unreachable!("Asked to display application: {:?}", self);
            },
            Node::FnDef(_) => {
                unreachable!("Asked to display function: {:?}", self);
            }
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(i) => write!(f, "Int({})", i), 
            Node::ThunkRef(t) => write!(f, "{:?}", (*t).borrow()),
            Node::App(el, er) => write!(f, "@({:?}, {:?})", el, er),
            Node::FnDef(def) => write!(f, "fn<{}:{}>", def.name, def.arity),
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


/* ***************** *
 * Builtin Functions *
 * ***************** */

impl State {
    pub fn push_int(&mut self, int_val: i64) {
        self.stack.push(Node::Int(int_val));
    }

    pub fn push_fn(&mut self, fn_def: FnDef) {
        self.stack.push(Node::FnDef(fn_def));
    }

    pub fn app(&mut self) {
        let nl = self.stack.pop().unwrap();
        let nr = self.stack.pop().unwrap();

        self.stack.push(Node::App(Box::new(nl), Box::new(nr)));
    }

    pub fn unwind(&mut self) {
        todo!()
    }
}

pub static FN_ADD: FnDef = FnDef {
    name: "add",
    arity: 2,
    fn_ref: eval_add
};

pub fn eval_add(state: &mut State) {

}

pub fn int(int_val: i64) -> Node {
    Node::Int(int_val)
}

pub fn thunk(boxed_t: Box<dyn ThunkEval>) -> Node {
    Node::ThunkRef(Rc::new(RefCell::new(Thunk::UThunk(boxed_t))))
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

pub struct TracedThunk {
    name: String,
    value: Node
}

impl ThunkEval for TracedThunk {
    fn eval_thunk(&self) -> Node {
        println!("Evaling: {}", self.name);
        self.value.clone()
    }
}

impl TracedThunk {
    pub fn new(name: String, value: Node) -> Node {
        thunk(Box::new(TracedThunk { name, value }))
    }
}

// macro_rules! bin_thunk {
//     ($thunk_name:ident, $eval_fn:ident, $fn_name:ident) => {
//         struct $thunk_name {
//             nl: Node,
//             nr: Node
//         }

//         impl ThunkEval for $thunk_name {
//             fn eval_thunk(&self) -> Node {
//                 $eval_fn(self.nl.clone(), self.nr.clone())
//             }
//         }

//         pub fn $fn_name(nl: Node, nr: Node) -> Node {
//             thunk(Box::new($thunk_name { nl, nr }))
//         }
//     }
// }

// bin_thunk!(AddThunk, eval_add, add);
// bin_thunk!(SubThunk, eval_sub, sub);
// bin_thunk!(DivThunk, eval_div, div);
// bin_thunk!(MulThunk, eval_mul, mul);


// fn eval_add(nl: Node, nr: Node) -> Node {
//     bin_arith!(nl, nr, +);
// }

// fn eval_sub(nl: Node, nr: Node) -> Node {
//     bin_arith!(nl, nr, -);
// }

// fn eval_div(nl: Node, nr: Node) -> Node {
//     bin_arith!(nl, nr, /);
// }

// fn eval_mul(nl: Node, nr: Node) -> Node {
//     bin_arith!(nl, nr, *);
// }